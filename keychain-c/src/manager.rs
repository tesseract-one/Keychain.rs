use keychain::KeychainManager;
use std::mem;
use result::CResult;

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_new() -> CResult<KeychainManager> {
  match KeychainManager::new() {
    Err(_) => CResult::Err,
    Ok(manager) => CResult::Ok(manager)
  }
}