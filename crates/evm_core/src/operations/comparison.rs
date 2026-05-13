use alloy::primitives::{I256, U256};

use crate::Evm;

pub fn lt(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(if a < b { U256::from(1u8) } else { U256::ZERO }).unwrap();
    evm.pc += 1;
}

pub fn gt(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(if a > b { U256::from(1u8) } else { U256::ZERO }).unwrap();
    evm.pc += 1;
}

pub fn slt(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let a_int = I256::from_limbs(*a.as_limbs());
    let b_int = I256::from_limbs(*b.as_limbs());
    evm.stack.push(if a_int < b_int { U256::from(1u8) } else { U256::ZERO }).unwrap();
    evm.pc += 1;
}

pub fn sgt(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let a_int = I256::from_limbs(*a.as_limbs());
    let b_int = I256::from_limbs(*b.as_limbs());
    evm.stack.push(if a_int > b_int { U256::from(1u8) } else { U256::ZERO }).unwrap();
    evm.pc += 1;
}

pub fn eq(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(if a == b { U256::from(1u8) } else { U256::ZERO }).unwrap();
    evm.pc += 1;
}

pub fn iszero(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    evm.stack.push(if a == U256::ZERO { U256::from(1u8) } else { U256::ZERO }).unwrap();
    evm.pc += 1;
}
