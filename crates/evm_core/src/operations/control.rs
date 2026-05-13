use alloy::primitives::U256;

use crate::{Evm, ProgramExitStatus};

pub fn jump(evm: &mut Evm) {
    let dest = evm.stack.pop().unwrap().as_limbs()[0] as usize;
    // JUMPDEST (0x5B) must be at the destination
    assert_eq!(evm.memory.load_byte(dest), 0x5B, "JUMP to invalid destination");
    evm.pc = dest;
}

pub fn jumpi(evm: &mut Evm) {
    let dest = evm.stack.pop().unwrap().as_limbs()[0] as usize;
    let cond = evm.stack.pop().unwrap();
    if cond != U256::ZERO {
        assert_eq!(evm.memory.load_byte(dest), 0x5B, "JUMPI to invalid destination");
        evm.pc = dest;
    } else {
        evm.pc += 1;
    }
}

pub fn jumpdest(evm: &mut Evm) {
    // No-op marker; just advance pc
    evm.pc += 1;
}

pub fn pc(evm: &mut Evm) {
    evm.stack.push(U256::from(evm.pc)).unwrap();
    evm.pc += 1;
}

pub fn gas(evm: &mut Evm) {
    // Gas tracking not implemented; push max to avoid reverts
    evm.stack.push(U256::MAX).unwrap();
    evm.pc += 1;
}

pub fn revert(evm: &mut Evm) {
    evm.stack.pop(); // offset
    evm.stack.pop(); // size
    evm.status = ProgramExitStatus::Failure;
}

pub fn ret(evm: &mut Evm) {
    evm.stack.pop(); // offset
    evm.stack.pop(); // size
    evm.status = ProgramExitStatus::Success;
}

pub fn invalid(evm: &mut Evm) {
    evm.status = ProgramExitStatus::Failure;
}
