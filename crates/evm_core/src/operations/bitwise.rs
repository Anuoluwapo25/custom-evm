use alloy::primitives::{I256, U256};

use crate::Evm;

pub fn and(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a & b).unwrap();
    evm.pc += 1;
}

pub fn or(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a | b).unwrap();
    evm.pc += 1;
}

pub fn xor(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a ^ b).unwrap();
    evm.pc += 1;
}

pub fn not(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    evm.stack.push(!a).unwrap();
    evm.pc += 1;
}

pub fn byte(evm: &mut Evm) {
    let i = evm.stack.pop().unwrap(); // byte index from MSB (0 = most significant)
    let x = evm.stack.pop().unwrap();
    if i >= U256::from(32u8) {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let idx = i.as_limbs()[0] as usize;
        let result = x.to_be_bytes::<32>()[idx];
        evm.stack.push(U256::from(result)).unwrap();
    }
    evm.pc += 1;
}

pub fn shl(evm: &mut Evm) {
    let shift = evm.stack.pop().unwrap();
    let value = evm.stack.pop().unwrap();
    if shift >= U256::from(256u16) {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(value << shift).unwrap();
    }
    evm.pc += 1;
}

pub fn shr(evm: &mut Evm) {
    let shift = evm.stack.pop().unwrap();
    let value = evm.stack.pop().unwrap();
    if shift >= U256::from(256u16) {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(value >> shift).unwrap();
    }
    evm.pc += 1;
}

pub fn sar(evm: &mut Evm) {
    let shift = evm.stack.pop().unwrap();
    let value = evm.stack.pop().unwrap();
    let value_int = I256::from_limbs(*value.as_limbs());
    if shift >= U256::from(256u16) {
        let result = if value_int.is_negative() { I256::MINUS_ONE } else { I256::ZERO };
        evm.stack.push(U256::from_limbs(*result.as_limbs())).unwrap();
    } else {
        let result = value_int >> shift.as_limbs()[0] as usize;
        evm.stack.push(U256::from_limbs(*result.as_limbs())).unwrap();
    }
    evm.pc += 1;
}
