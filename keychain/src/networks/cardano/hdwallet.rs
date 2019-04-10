//! Hierarchical Deterministic (HD) Wallet
//!
//! Follow the Ed25519-BIP32 paper
//!
//! Supports:
//! * Transform Seed to Extended Private key
//! * Hard and Soft derivation using 32 bits indices
//! * Derivation Scheme V2
//!
use cryptoxide::digest::Digest;
use cryptoxide::sha2::Sha512;
use cryptoxide::hmac::Hmac;
use cryptoxide::mac::Mac;
use cryptoxide::ed25519::signature_extended;
use cryptoxide::ed25519;
use cryptoxide::util::fixed_time_eq;

use bip39;

use std::{fmt, result};
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};
use util::{hex, securemem};

pub const XPRV_SIZE: usize = 96;
pub const XPUB_SIZE: usize = 64;
pub const SIGNATURE_SIZE: usize = 64;

pub const PUBLIC_KEY_SIZE: usize = 32;
pub const CHAIN_CODE_SIZE: usize = 32;

/// HDWallet errors
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Error {
    /// the given extended private key is of invalid size. The parameter is the given size.
    ///
    /// See `XPRV_SIZE` for the expected size.
    InvalidXPrvSize(usize),
    /// The given extended private key is of invalid format for our usage of ED25519.
    ///
    /// This is not a problem of the size, see `Error::InvalidXPrvSize`
    InvalidXPrv(&'static str),
    HexadecimalError(hex::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::InvalidXPrvSize(sz) => {
               write!(f, "Invalid XPrv Size, expected {} bytes, but received {} bytes.", XPRV_SIZE, sz)
            },
            &Error::InvalidXPrv(ref err) => {
               write!(f, "Invalid XPrv: {}", err)
            },
            &Error::HexadecimalError(_) => {
               write!(f, "Invalid hexadecimal.")
            },
        }
    }
}
impl From<hex::Error> for Error {
    fn from(e: hex::Error) -> Error { Error::HexadecimalError(e) }
}
impl ::std::error::Error for Error {
    fn cause(&self) -> Option<& ::std::error::Error> {
        match self {
            Error::HexadecimalError(ref err) => Some(err),
            _ => None
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

/// Ed25519-bip32 Scheme Derivation version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivationScheme {
    V2,
}
impl Default for DerivationScheme {
    fn default() -> Self { DerivationScheme::V2 }
}

/// HDWallet extended private key
///
/// Effectively this is ed25519 extended secret key (64 bytes) followed by a chain code (32 bytes)
pub struct XPrv([u8; XPRV_SIZE]);
impl XPrv {
    pub fn generate_from_bip39(bytes: &bip39::Seed) -> Self {
        let mut out = [0u8; XPRV_SIZE];

        mk_ed25519_extended(&mut out[0..64], &bytes.as_ref()[0..32]);
        out[31] &= 0b1101_1111; // set 3rd highest bit to 0 as per the spec
        out[64..96].clone_from_slice(&bytes.as_ref()[32..64]);

        Self::from_bytes(out)
    }

    // Create a XPrv from the given bytes.
    //
    // This function does not perform any validity check and should not be used outside
    // of this module.
    fn from_bytes(bytes: [u8;XPRV_SIZE]) -> Self { XPrv(bytes) }

    /// Create a `XPrv` by taking ownership of the given array
    ///
    /// This function may returns an error if it does not have the expected
    /// format.
    pub fn from_bytes_verified(bytes: [u8;XPRV_SIZE]) -> Result<Self> {
        let scalar = &bytes[0..32];
        let last   = scalar[31];
        let first  = scalar[0];

        if (last & 0b1110_0000) != 0b0100_0000 {
            return Err(Error::InvalidXPrv("expected 3 highest bits to be 0b010"))
        }
        if (first & 0b0000_0111) != 0b0000_0000 {
            return Err(Error::InvalidXPrv("expected 3 lowest bits to be 0b000"))
        }

        Ok(XPrv(bytes))
    }

    /// Create a `XPrv` from the given slice. This slice must be of size `XPRV_SIZE`
    /// otherwise it will return `Err`.
    ///
    fn from_slice(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != XPRV_SIZE {
            return Err(Error::InvalidXPrvSize(bytes.len()));
        }
        let mut buf = [0u8;XPRV_SIZE];
        buf[..].clone_from_slice(bytes);
        Ok(XPrv::from_bytes(buf))
    }

    /// Get the associated `XPub`
    ///
    /// ```
    /// use cardano::hdwallet::{XPrv, XPub, Seed};
    ///
    /// let seed = Seed::from_bytes([0;32]) ;
    /// let xprv = XPrv::generate_from_seed(&seed);
    ///
    /// let xpub = xprv.public();
    /// ```
    pub fn public(&self) -> XPub {
        let pk = mk_public_key(&self.as_ref()[0..64]);
        let mut out = [0u8; XPUB_SIZE];
        out[0..32].clone_from_slice(&pk);
        out[32..64].clone_from_slice(&self.as_ref()[64..]);
        XPub::from_bytes(out)
    }

    /// sign the given message with the `XPrv`.
    ///
    /// ```
    /// use cardano::hdwallet::{XPrv, XPub, Signature, Seed};
    ///
    /// let seed = Seed::from_bytes([0;32]) ;
    /// let xprv = XPrv::generate_from_seed(&seed);
    /// let msg = b"Some message...";
    ///
    /// let signature : Signature<String> = xprv.sign(msg);
    /// assert!(xprv.verify(msg, &signature));
    /// ```
    pub fn sign<T>(&self, message: &[u8]) -> Signature<T> {
        Signature::from_bytes(signature_extended(message, &self.as_ref()[0..64]))
    }

    /// verify a given signature
    ///
    pub fn verify<T>(&self, message: &[u8], signature: &Signature<T>) -> bool {
        let xpub = self.public();
        xpub.verify(message, signature)
    }

    pub fn derive(&self, scheme: DerivationScheme, index: DerivationIndex) -> Self {
        derive_private(self, index, scheme)
    }
}
impl PartialEq for XPrv {
    fn eq(&self, rhs: &XPrv) -> bool { fixed_time_eq(self.as_ref(), rhs.as_ref()) }
}
impl Eq for XPrv {}
impl Clone for XPrv {
    fn clone(&self) -> Self { Self::from_slice(self.as_ref()).expect("it is already a safely constructed XPrv") }
}
impl fmt::Debug for XPrv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}
impl fmt::Display for XPrv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}
impl AsRef<[u8]> for XPrv {
    fn as_ref(&self) -> &[u8] { &self.0 }
}
impl Drop for XPrv {
    fn drop(&mut self) {
        securemem::zero(&mut self.0);
    }
}

/// Extended Public Key (Point + ChainCode)
#[derive(Clone, Copy)]
pub struct XPub([u8; XPUB_SIZE]);
impl XPub {
    /// create a `XPub` by taking ownership of the given array
    pub fn from_bytes(bytes: [u8;XPUB_SIZE]) -> Self { XPub(bytes) }

    /// verify a signature
    ///
    /// ```
    /// use cardano::hdwallet::{XPrv, XPub, Seed, Signature};
    ///
    /// let seed = Seed::from_bytes([0;32]);
    /// let xprv = XPrv::generate_from_seed(&seed);
    /// let xpub = xprv.public();
    /// let msg = b"Some message...";
    ///
    /// let signature : Signature<String> = xprv.sign(msg);
    /// assert!(xpub.verify(msg, &signature));
    /// ```
    pub fn verify<T>(&self, message: &[u8], signature: &Signature<T>) -> bool {
        ed25519::verify(message, &self.as_ref()[0..32], signature.as_ref())
    }
}
impl PartialEq for XPub {
    fn eq(&self, rhs: &XPub) -> bool { fixed_time_eq(self.as_ref(), rhs.as_ref()) }
}
impl Eq for XPub {}
impl Hash for XPub {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0)
    }
}
impl fmt::Display for XPub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}
impl fmt::Debug for XPub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}
impl AsRef<[u8]> for XPub {
    fn as_ref(&self) -> &[u8] { &self.0 }
}

/// a signature with an associated type tag
///
#[derive(Clone)]
pub struct Signature<T> {
    bytes: [u8; SIGNATURE_SIZE],
    _phantom: PhantomData<T>,
}
impl<T> Signature<T> {
    pub fn from_bytes(bytes: [u8;SIGNATURE_SIZE]) -> Self {
        Signature { bytes: bytes, _phantom: PhantomData }
    }
}
impl<T> PartialEq for Signature<T> {
    fn eq(&self, rhs: &Signature<T>) -> bool { fixed_time_eq(self.as_ref(), rhs.as_ref()) }
}
impl<T> Eq for Signature<T> {}
impl<T> fmt::Display for Signature<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}
impl<T> fmt::Debug for Signature<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}
impl<T> AsRef<[u8]> for Signature<T> {
    fn as_ref(&self) -> &[u8] { &self.bytes }
}

pub type DerivationIndex = u32;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "generic-serialization", derive(Serialize, Deserialize))]
enum DerivationType {
    Soft(u32),
    Hard(u32),
}

fn to_type(index: DerivationIndex) -> DerivationType {
    if index >= 0x80000000 {
        DerivationType::Hard(index)
    } else {
        DerivationType::Soft(index)
    }
}

fn mk_ed25519_extended(extended_out: &mut [u8], secret: &[u8]) {
    assert!(extended_out.len() == 64);
    assert!(secret.len() == 32);
    let mut hasher = Sha512::new();
    hasher.input(secret);
    hasher.result(extended_out);
    extended_out[0] &= 248;
    extended_out[31] &= 63;
    extended_out[31] |= 64;
}

fn le32(i: u32) -> [u8; 4] {
    [i as u8, (i >> 8) as u8, (i >> 16) as u8, (i >> 24) as u8]
}

fn serialize_index(i: u32, derivation_scheme: DerivationScheme) -> [u8; 4] {
    match derivation_scheme {
        DerivationScheme::V2 => le32(i),
    }
}

fn mk_xprv(out: &mut [u8; XPRV_SIZE], kl: &[u8], kr: &[u8], cc: &[u8]) {
    assert!(kl.len() == 32);
    assert!(kr.len() == 32);
    assert!(cc.len() == CHAIN_CODE_SIZE);

    out[0..32].clone_from_slice(kl);
    out[32..64].clone_from_slice(kr);
    out[64..96].clone_from_slice(cc);
}

fn add_256bits_v2(x: &[u8], y: &[u8]) -> [u8; 32] {
    assert!(x.len() == 32);
    assert!(y.len() == 32);

    let mut carry: u16 = 0;
    let mut out = [0u8; 32];
    for i in 0..32 {
        let r = (x[i] as u16) + (y[i] as u16) + carry;
        out[i] = r as u8;
        carry = r >> 8;
    }
    out
}

fn add_256bits(x: &[u8], y: &[u8], scheme: DerivationScheme) -> [u8; 32] {
    match scheme {
        DerivationScheme::V2 => add_256bits_v2(x, y),
    }
}

fn add_28_mul8_v2(x: &[u8], y: &[u8]) -> [u8; 32] {
    assert!(x.len() == 32);
    assert!(y.len() == 32);

    let mut carry: u16 = 0;
    let mut out = [0u8; 32];

    for i in 0..28 {
        let r = x[i] as u16 + ((y[i] as u16) << 3) + carry;
        out[i] = (r & 0xff) as u8;
        carry = r >> 8;
    }
    for i in 28..32 {
        let r = x[i] as u16 + carry;
        out[i] = (r & 0xff) as u8;
        carry = r >> 8;
    }
    out
}

fn add_28_mul8(x: &[u8], y: &[u8], scheme: DerivationScheme) -> [u8; 32] {
    match scheme {
        DerivationScheme::V2 => add_28_mul8_v2(x, y),
    }
}

fn derive_private(xprv: &XPrv, index: DerivationIndex, scheme: DerivationScheme) -> XPrv {
    /*
     * If so (hardened child):
     *    let Z = HMAC-SHA512(Key = cpar, Data = 0x00 || ser256(left(kpar)) || ser32(i)).
     *    let I = HMAC-SHA512(Key = cpar, Data = 0x01 || ser256(left(kpar)) || ser32(i)).
     * If not (normal child):
     *    let Z = HMAC-SHA512(Key = cpar, Data = 0x02 || serP(point(kpar)) || ser32(i)).
     *    let I = HMAC-SHA512(Key = cpar, Data = 0x03 || serP(point(kpar)) || ser32(i)).
     **/

    let ekey = &xprv.as_ref()[0..64];
    let kl = &ekey[0..32];
    let kr = &ekey[32..64];
    let chaincode = &xprv.as_ref()[64..96];

    let mut zmac = Hmac::new(Sha512::new(), &chaincode);
    let mut imac = Hmac::new(Sha512::new(), &chaincode);
    let seri = serialize_index(index, scheme);
    match to_type(index) {
        DerivationType::Soft(_) => {
            let pk = mk_public_key(ekey);
            zmac.input(&[0x2]);
            zmac.input(&pk);
            zmac.input(&seri);
            imac.input(&[0x3]);
            imac.input(&pk);
            imac.input(&seri);
        }
        DerivationType::Hard(_) => {
            zmac.input(&[0x0]);
            zmac.input(ekey);
            zmac.input(&seri);
            imac.input(&[0x1]);
            imac.input(ekey);
            imac.input(&seri);
        }
    };

    let mut zout = [0u8; 64];
    zmac.raw_result(&mut zout);
    let zl = &zout[0..32];
    let zr = &zout[32..64];

    // left = kl + 8 * trunc28(zl)
    let left = add_28_mul8(kl, zl, scheme);
    // right = zr + kr
    let right = add_256bits(kr, zr, scheme);

    let mut iout = [0u8; 64];
    imac.raw_result(&mut iout);
    let cc = &iout[32..];

    let mut out = [0u8; XPRV_SIZE];
    mk_xprv(&mut out, &left, &right, cc);

    imac.reset();
    zmac.reset();

    XPrv::from_bytes(out)
}

fn mk_public_key(extended_secret: &[u8]) -> [u8; PUBLIC_KEY_SIZE] {
    assert!(extended_secret.len() == 64);
    ed25519::to_public(extended_secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    const D1: [u8;XPRV_SIZE] =
        [0xf8, 0xa2, 0x92, 0x31, 0xee, 0x38, 0xd6, 0xc5, 0xbf, 0x71, 0x5d, 0x5b, 0xac, 0x21, 0xc7,
         0x50, 0x57, 0x7a, 0xa3, 0x79, 0x8b, 0x22, 0xd7, 0x9d, 0x65, 0xbf, 0x97, 0xd6, 0xfa, 0xde,
         0xa1, 0x5a, 0xdc, 0xd1, 0xee, 0x1a, 0xbd, 0xf7, 0x8b, 0xd4, 0xbe, 0x64, 0x73, 0x1a, 0x12,
         0xde, 0xb9, 0x4d, 0x36, 0x71, 0x78, 0x41, 0x12, 0xeb, 0x6f, 0x36, 0x4b, 0x87, 0x18, 0x51,
         0xfd, 0x1c, 0x9a, 0x24, 0x73, 0x84, 0xdb, 0x9a, 0xd6, 0x00, 0x3b, 0xbd, 0x08, 0xb3, 0xb1,
         0xdd, 0xc0, 0xd0, 0x7a, 0x59, 0x72, 0x93, 0xff, 0x85, 0xe9, 0x61, 0xbf, 0x25, 0x2b, 0x33,
         0x12, 0x62, 0xed, 0xdf, 0xad, 0x0d];

    const D1_H0: [u8;XPRV_SIZE] =
        [0x60, 0xd3, 0x99, 0xda, 0x83, 0xef, 0x80, 0xd8, 0xd4, 0xf8, 0xd2, 0x23, 0x23, 0x9e, 0xfd,
         0xc2, 0xb8, 0xfe, 0xf3, 0x87, 0xe1, 0xb5, 0x21, 0x91, 0x37, 0xff, 0xb4, 0xe8, 0xfb, 0xde,
         0xa1, 0x5a, 0xdc, 0x93, 0x66, 0xb7, 0xd0, 0x03, 0xaf, 0x37, 0xc1, 0x13, 0x96, 0xde, 0x9a,
         0x83, 0x73, 0x4e, 0x30, 0xe0, 0x5e, 0x85, 0x1e, 0xfa, 0x32, 0x74, 0x5c, 0x9c, 0xd7, 0xb4,
         0x27, 0x12, 0xc8, 0x90, 0x60, 0x87, 0x63, 0x77, 0x0e, 0xdd, 0xf7, 0x72, 0x48, 0xab, 0x65,
         0x29, 0x84, 0xb2, 0x1b, 0x84, 0x97, 0x60, 0xd1, 0xda, 0x74, 0xa6, 0xf5, 0xbd, 0x63, 0x3c,
         0xe4, 0x1a, 0xdc, 0xee, 0xf0, 0x7a];

    const MSG: &'static [u8] = b"Hello World";

    const D1_H0_SIGNATURE: [u8; 64] =
        [0x90, 0x19, 0x4d, 0x57, 0xcd, 0xe4, 0xfd, 0xad, 0xd0, 0x1e, 0xb7, 0xcf, 0x16, 0x17, 0x80,
         0xc2, 0x77, 0xe1, 0x29, 0xfc, 0x71, 0x35, 0xb9, 0x77, 0x79, 0xa3, 0x26, 0x88, 0x37, 0xe4,
         0xcd, 0x2e, 0x94, 0x44, 0xb9, 0xbb, 0x91, 0xc0, 0xe8, 0x4d, 0x23, 0xbb, 0xa8, 0x70, 0xdf,
         0x3c, 0x4b, 0xda, 0x91, 0xa1, 0x10, 0xef, 0x73, 0x56, 0x38, 0xfa, 0x7a, 0x34, 0xea, 0x20,
         0x46, 0xd4, 0xbe, 0x04];

    fn compare_xprv(xprv: &[u8], expected_xprv: &[u8]) {
        assert_eq!(xprv[64..].to_vec(),
                   expected_xprv[64..].to_vec(),
                   "chain code");
        assert_eq!(xprv[..64].to_vec(),
                   expected_xprv[..64].to_vec(),
                   "extended key");
    }

    fn derive_xprv_eq(parent_xprv: &XPrv, idx: DerivationIndex, expected_xprv: [u8; 96]) {
        let child_xprv = derive_private(parent_xprv, idx, DerivationScheme::V2);
        compare_xprv(child_xprv.as_ref(), &expected_xprv);
    }

    #[test]
    fn xprv_derive() {
        let prv = XPrv::from_bytes_verified(D1).unwrap();
        derive_xprv_eq(&prv, 0x80000000, D1_H0);
    }

    fn do_sign(xprv: &XPrv, expected_signature: &[u8]) {
        let signature : Signature<Vec<u8>> = xprv.sign(MSG);
        assert_eq!(signature.as_ref(), expected_signature);
    }

    #[test]
    fn xprv_sign() {
        let prv = XPrv::from_bytes_verified(D1_H0).unwrap();
        do_sign(&prv, &D1_H0_SIGNATURE);
    }
}

#[cfg(test)]
#[cfg(feature = "with-bench")]
mod bench {
    use super::*;
    use test;

    #[bench]
    fn derivate_hard_v2(b: &mut test::Bencher) {
        let seed = Seed::from_bytes([0;SEED_SIZE]);
        let sk = XPrv::generate_from_seed(&seed);
        b.iter(|| {
            let _ = sk.derive(DerivationScheme::V2, 0x80000000);
        })
    }

    #[bench]
    fn derivate_soft_v2_xprv(b: &mut test::Bencher) {
        let seed = Seed::from_bytes([0;SEED_SIZE]);
        let sk = XPrv::generate_from_seed(&seed);
        b.iter(|| {
            let _ = sk.derive(DerivationScheme::V2, 0);
        })
    }
    #[bench]
    fn derivate_soft_v2_xpub(b: &mut test::Bencher) {
        let seed = Seed::from_bytes([0;SEED_SIZE]);
        let sk = XPrv::generate_from_seed(&seed);
        let pk = sk.public();
        b.iter(|| {
            let _ = pk.derive(DerivationScheme::V2, 0);
        })
    }
}