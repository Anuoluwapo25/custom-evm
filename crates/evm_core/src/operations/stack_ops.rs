use alloy::primitives::U256;

use crate::Evm;

/// PUSH0: push literal 0 onto stack.
pub fn push0(evm: &mut Evm) {
    evm.stack.push(U256::ZERO).unwrap();
    evm.pc += 1;
}

/// PUSH1..PUSH32: read `n` immediate bytes after the opcode, push as big-endian U256.
/// The opcode byte is at evm.pc; data starts at evm.pc + 1.
fn push_n(evm: &mut Evm, n: usize) {
    let data_start = evm.pc + 1;
    let mut buf = [0u8; 32];
    for i in 0..n {
        buf[32 - n + i] = evm.memory.load_byte(data_start + i);
    }
    evm.stack.push(U256::from_be_bytes(buf)).unwrap();
    evm.pc += 1 + n; // opcode byte + n data bytes
}

pub fn push1(evm: &mut Evm) { push_n(evm, 1); }
pub fn push2(evm: &mut Evm) { push_n(evm, 2); }
pub fn push3(evm: &mut Evm) { push_n(evm, 3); }
pub fn push4(evm: &mut Evm) { push_n(evm, 4); }
pub fn push5(evm: &mut Evm) { push_n(evm, 5); }
pub fn push6(evm: &mut Evm) { push_n(evm, 6); }
pub fn push7(evm: &mut Evm) { push_n(evm, 7); }
pub fn push8(evm: &mut Evm) { push_n(evm, 8); }
pub fn push9(evm: &mut Evm) { push_n(evm, 9); }
pub fn push10(evm: &mut Evm) { push_n(evm, 10); }
pub fn push11(evm: &mut Evm) { push_n(evm, 11); }
pub fn push12(evm: &mut Evm) { push_n(evm, 12); }
pub fn push13(evm: &mut Evm) { push_n(evm, 13); }
pub fn push14(evm: &mut Evm) { push_n(evm, 14); }
pub fn push15(evm: &mut Evm) { push_n(evm, 15); }
pub fn push16(evm: &mut Evm) { push_n(evm, 16); }
pub fn push17(evm: &mut Evm) { push_n(evm, 17); }
pub fn push18(evm: &mut Evm) { push_n(evm, 18); }
pub fn push19(evm: &mut Evm) { push_n(evm, 19); }
pub fn push20(evm: &mut Evm) { push_n(evm, 20); }
pub fn push21(evm: &mut Evm) { push_n(evm, 21); }
pub fn push22(evm: &mut Evm) { push_n(evm, 22); }
pub fn push23(evm: &mut Evm) { push_n(evm, 23); }
pub fn push24(evm: &mut Evm) { push_n(evm, 24); }
pub fn push25(evm: &mut Evm) { push_n(evm, 25); }
pub fn push26(evm: &mut Evm) { push_n(evm, 26); }
pub fn push27(evm: &mut Evm) { push_n(evm, 27); }
pub fn push28(evm: &mut Evm) { push_n(evm, 28); }
pub fn push29(evm: &mut Evm) { push_n(evm, 29); }
pub fn push30(evm: &mut Evm) { push_n(evm, 30); }
pub fn push31(evm: &mut Evm) { push_n(evm, 31); }
pub fn push32(evm: &mut Evm) { push_n(evm, 32); }

/// DUP1..DUP16: copy the nth stack item (1 = top) to the top.
fn dup_n(evm: &mut Evm, n: usize) {
    let val = evm.stack.peek(n - 1).unwrap();
    evm.stack.push(val).unwrap();
    evm.pc += 1;
}

pub fn dup1(evm: &mut Evm) { dup_n(evm, 1); }
pub fn dup2(evm: &mut Evm) { dup_n(evm, 2); }
pub fn dup3(evm: &mut Evm) { dup_n(evm, 3); }
pub fn dup4(evm: &mut Evm) { dup_n(evm, 4); }
pub fn dup5(evm: &mut Evm) { dup_n(evm, 5); }
pub fn dup6(evm: &mut Evm) { dup_n(evm, 6); }
pub fn dup7(evm: &mut Evm) { dup_n(evm, 7); }
pub fn dup8(evm: &mut Evm) { dup_n(evm, 8); }
pub fn dup9(evm: &mut Evm) { dup_n(evm, 9); }
pub fn dup10(evm: &mut Evm) { dup_n(evm, 10); }
pub fn dup11(evm: &mut Evm) { dup_n(evm, 11); }
pub fn dup12(evm: &mut Evm) { dup_n(evm, 12); }
pub fn dup13(evm: &mut Evm) { dup_n(evm, 13); }
pub fn dup14(evm: &mut Evm) { dup_n(evm, 14); }
pub fn dup15(evm: &mut Evm) { dup_n(evm, 15); }
pub fn dup16(evm: &mut Evm) { dup_n(evm, 16); }

/// SWAP1..SWAP16: swap the top with the (n+1)th stack item.
fn swap_n(evm: &mut Evm, n: usize) {
    evm.stack.swap_top(n).unwrap();
    evm.pc += 1;
}

pub fn swap1(evm: &mut Evm) { swap_n(evm, 1); }
pub fn swap2(evm: &mut Evm) { swap_n(evm, 2); }
pub fn swap3(evm: &mut Evm) { swap_n(evm, 3); }
pub fn swap4(evm: &mut Evm) { swap_n(evm, 4); }
pub fn swap5(evm: &mut Evm) { swap_n(evm, 5); }
pub fn swap6(evm: &mut Evm) { swap_n(evm, 6); }
pub fn swap7(evm: &mut Evm) { swap_n(evm, 7); }
pub fn swap8(evm: &mut Evm) { swap_n(evm, 8); }
pub fn swap9(evm: &mut Evm) { swap_n(evm, 9); }
pub fn swap10(evm: &mut Evm) { swap_n(evm, 10); }
pub fn swap11(evm: &mut Evm) { swap_n(evm, 11); }
pub fn swap12(evm: &mut Evm) { swap_n(evm, 12); }
pub fn swap13(evm: &mut Evm) { swap_n(evm, 13); }
pub fn swap14(evm: &mut Evm) { swap_n(evm, 14); }
pub fn swap15(evm: &mut Evm) { swap_n(evm, 15); }
pub fn swap16(evm: &mut Evm) { swap_n(evm, 16); }
