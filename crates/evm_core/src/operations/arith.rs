use alloy::primitives::{I256, U256};

use crate::{Evm, ProgramExitStatus};

pub fn stop(evm: &mut Evm) {
    evm.status = ProgramExitStatus::Success;
}

pub fn add(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a.wrapping_add(b)).unwrap();
    evm.pc += 1;
}

pub fn mul(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a.wrapping_mul(b)).unwrap();
    evm.pc += 1;
}

pub fn sub(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a.wrapping_sub(b)).unwrap();
    evm.pc += 1;
}

pub fn div(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a / b).unwrap();
    }
    evm.pc += 1;
}

pub fn sdiv(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let a_int = I256::from_limbs(*a.as_limbs());
    let b_int = I256::from_limbs(*b.as_limbs());
    if b_int == I256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let result = a_int.wrapping_div(b_int);
        evm.stack.push(U256::from_limbs(*result.as_limbs())).unwrap();
    }
    evm.pc += 1;
}

pub fn modulo(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a % b).unwrap();
    }
    evm.pc += 1;
}

pub fn smod(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let a_int = I256::from_limbs(*a.as_limbs());
    let b_int = I256::from_limbs(*b.as_limbs());
    if b_int == I256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let result = a_int.wrapping_rem(b_int);
        evm.stack.push(U256::from_limbs(*result.as_limbs())).unwrap();
    }
    evm.pc += 1;
}

pub fn addmod(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let n = evm.stack.pop().unwrap();
    if n == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a.add_mod(b, n)).unwrap();
    }
    evm.pc += 1;
}

pub fn mulmod(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let n = evm.stack.pop().unwrap();
    if n == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a.mul_mod(b, n)).unwrap();
    }
    evm.pc += 1;
}

pub fn exp(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a.pow(b)).unwrap();
    evm.pc += 1;
}

pub fn signextend(evm: &mut Evm) {
    let b = evm.stack.pop().unwrap(); // byte index (0 = least significant byte)
    let x = evm.stack.pop().unwrap(); // value to sign-extend

    if b < U256::from(31u8) {
        let byte_index = b.as_limbs()[0] as usize;
        let bit_index = byte_index * 8 + 7;
        let x_bytes = x.to_be_bytes::<32>();
        // In big-endian layout, byte_index 0 is the least significant byte at position 31
        let sign_byte = x_bytes[31 - byte_index];
        let sign_bit = (sign_byte >> 7) & 1;
        if sign_bit == 1 {
            let mask = U256::MAX << U256::from(bit_index + 1);
            evm.stack.push(x | mask).unwrap();
        } else {
            let mask = (U256::from(1u8) << U256::from(bit_index + 1)) - U256::from(1u8);
            evm.stack.push(x & mask).unwrap();
        }
    } else {
        evm.stack.push(x).unwrap();
    }
    evm.pc += 1;
}
