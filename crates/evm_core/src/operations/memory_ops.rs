use alloy::primitives::U256;

use crate::Evm;

pub fn pop(evm: &mut Evm) {
    evm.stack.pop().unwrap();
}

pub fn mload(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap().as_limbs()[0] as usize;
    let value = evm.memory.load_word(offset);
    evm.stack.push(value).unwrap();
}

pub fn mstore(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap().as_limbs()[0] as usize;
    let value = evm.stack.pop().unwrap();
    evm.memory.store_word(offset, value);
}

pub fn mstore8(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap().as_limbs()[0] as usize;
    let value = evm.stack.pop().unwrap();
    let byte = value.to_be_bytes::<32>()[31];
    evm.memory.store_byte(offset, byte);
}

pub fn msize(evm: &mut Evm) {
    let size = evm.memory.len();
    evm.stack.push(U256::from(size)).unwrap();
}
