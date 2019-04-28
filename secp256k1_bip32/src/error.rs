use secp256k1::Error as SecError;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum KeyError {
  InvalidSignature(usize, usize),
  InvalidPublicKey,
  InvalidSecretKey,
  InvalidRecoveryId,
  InvalidMessage,
  InvalidInputLength,
  TweakOutOfRange,
  InvalidDataSize(usize, usize),
  InvalidEntropySize(usize),
  DeriveDepthTooBig,
  InternalError
}

impl fmt::Display for KeyError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &KeyError::InvalidSignature(bad, good) => {
        write!(f, "Invalid signature {}, expected {}", bad, good)
      }
      &KeyError::InvalidPublicKey => write!(f, "Invalid public key"),
      &KeyError::InvalidSecretKey => write!(f, "Invalid secret key"),
      &KeyError::InvalidRecoveryId => write!(f, "Invalid recovery id"),
      &KeyError::InvalidMessage => write!(f, "Invalid message"),
      &KeyError::InvalidInputLength => write!(f, "Invalid input length"),
      &KeyError::TweakOutOfRange => write!(f, "Tweak out of range"),
      &KeyError::InvalidDataSize(bad, good) => {
        write!(f, "Invalid key data size {}, expected {}", bad, good)
      }
      &KeyError::InvalidEntropySize(size) => write!(f, "Invalid entropy size {}", size),
      &KeyError::InternalError => write!(f, "Unknown internal error"),
      &KeyError::DeriveDepthTooBig => write!(f, "Derive depth is too big")
    }
  }
}

impl From<SecError> for KeyError {
  fn from(err: SecError) -> Self {
    match err {
      SecError::InvalidSignature => KeyError::InvalidSignature(0, secp256k1::util::SIGNATURE_SIZE),
      SecError::InvalidPublicKey => KeyError::InvalidPublicKey,
      SecError::InvalidSecretKey => KeyError::InvalidSecretKey,
      SecError::InvalidRecoveryId => KeyError::InvalidRecoveryId,
      SecError::InvalidMessage => KeyError::InvalidMessage,
      SecError::InvalidInputLength => KeyError::InvalidInputLength,
      SecError::TweakOutOfRange => KeyError::TweakOutOfRange
    }
  }
}

impl error::Error for KeyError {}
