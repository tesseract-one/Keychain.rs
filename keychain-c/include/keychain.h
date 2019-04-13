/* Generated with cbindgen:0.8.3 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  Err_KeychainManager,
  Ok_KeychainManager,
} CResult_KeychainManager_Tag;

typedef struct {
  KeychainManager _0;
} Ok_Body_KeychainManager;

typedef struct {
  CResult_KeychainManager_Tag tag;
  union {
    Ok_Body_KeychainManager ok;
  };
} CResult_KeychainManager;

typedef struct {
  uint32_t _0;
} Network;

extern const Network NETWORK_BITCOIN;

extern const Network NETWORK_CARDANO;

extern const Network NETWORK_ETHEREUM;

CResult_KeychainManager keychain_manager_new(void);
