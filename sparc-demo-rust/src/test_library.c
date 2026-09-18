// test library
//
// C functions to be called from Rust to check ABI correctness

#include <stdint.h>

/// Testing the fix in https://github.com/rust-lang/rust/pull/160401
struct TwoU8s {
    uint8_t one;
    uint8_t two;
};

struct TwoU8s rust_dbg_extern_return_TwoU8s() {
    struct TwoU8s s;
    s.one = 10;
    s.two = 20;
    return s;
}

struct TwoU8s rust_dbg_extern_identity_TwoU8s(struct TwoU8s u) {
    return u;
}

