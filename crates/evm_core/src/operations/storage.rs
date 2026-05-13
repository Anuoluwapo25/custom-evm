use alloy::primitives::U256;

use crate::Evm;

pub fn sload(evm: &mut Evm) {
    let key = evm.stack.pop().unwrap();
    let value = evm
        .storage
        .data
        .get(&evm.tx.to)
        .and_then(|acct| acct.slots.get(&key))
        .copied()
        .unwrap_or(U256::ZERO);
    evm.stack.push(value).unwrap();
    evm.pc += 1;
}

pub fn sstore(evm: &mut Evm) {
    let key = evm.stack.pop().unwrap();
    let value = evm.stack.pop().unwrap();
    if let Some(acct) = evm.storage.data.get_mut(&evm.tx.to) {
        acct.slots.insert(key, value);
    }
    evm.pc += 1;
}
