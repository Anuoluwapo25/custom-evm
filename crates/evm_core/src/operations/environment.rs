use alloy::primitives::U256;

use crate::Evm;

pub fn address(evm: &mut Evm) {
    let addr = U256::from_be_slice(evm.tx.to.as_slice());
    evm.stack.push(addr).unwrap();
    evm.pc += 1;
}

pub fn caller(evm: &mut Evm) {
    let addr = U256::from_be_slice(evm.tx.from.as_slice());
    evm.stack.push(addr).unwrap();
    evm.pc += 1;
}

pub fn callvalue(evm: &mut Evm) {
    evm.stack.push(evm.tx.value).unwrap();
    evm.pc += 1;
}

pub fn calldatasize(evm: &mut Evm) {
    evm.stack.push(U256::from(evm.tx.data.len())).unwrap();
    evm.pc += 1;
}

pub fn calldataload(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap().as_limbs()[0] as usize;
    let mut buf = [0u8; 32];
    let data = &evm.tx.data;
    for i in 0..32 {
        if offset + i < data.len() {
            buf[i] = data[offset + i];
        }
    }
    evm.stack.push(U256::from_be_bytes(buf)).unwrap();
    evm.pc += 1;
}

pub fn calldatacopy(evm: &mut Evm) {
    let mem_offset = evm.stack.pop().unwrap().as_limbs()[0] as usize;
    let data_offset = evm.stack.pop().unwrap().as_limbs()[0] as usize;
    let size = evm.stack.pop().unwrap().as_limbs()[0] as usize;
    let data = evm.tx.data.clone();
    for i in 0..size {
        let byte = if data_offset + i < data.len() { data[data_offset + i] } else { 0 };
        evm.memory.store_byte(mem_offset + i, byte);
    }
    evm.pc += 1;
}

pub fn codesize(evm: &mut Evm) {
    evm.stack.push(U256::from(evm.memory.len())).unwrap();
    evm.pc += 1;
}

pub fn origin(evm: &mut Evm) {
    let addr = U256::from_be_slice(evm.tx.from.as_slice());
    evm.stack.push(addr).unwrap();
    evm.pc += 1;
}

pub fn gasprice(evm: &mut Evm) {
    evm.stack.push(U256::ZERO).unwrap();
    evm.pc += 1;
}

pub fn blocknumber(evm: &mut Evm) {
    evm.stack.push(evm.block_env.number).unwrap();
    evm.pc += 1;
}

pub fn timestamp(evm: &mut Evm) {
    evm.stack.push(evm.block_env.timestamp).unwrap();
    evm.pc += 1;
}

pub fn coinbase(evm: &mut Evm) {
    let addr = U256::from_be_slice(evm.block_env.coinbase.as_slice());
    evm.stack.push(addr).unwrap();
    evm.pc += 1;
}

pub fn gaslimit(evm: &mut Evm) {
    evm.stack.push(evm.block_env.gas_limit).unwrap();
    evm.pc += 1;
}

pub fn chainid(evm: &mut Evm) {
    evm.stack.push(evm.block_env.chain_id).unwrap();
    evm.pc += 1;
}

pub fn basefee(evm: &mut Evm) {
    evm.stack.push(evm.block_env.base_fee).unwrap();
    evm.pc += 1;
}

pub fn selfbalance(evm: &mut Evm) {
    let balance = evm
        .storage
        .data
        .get(&evm.tx.to)
        .map(|acct| acct.balance)
        .unwrap_or(U256::ZERO);
    evm.stack.push(balance).unwrap();
    evm.pc += 1;
}

pub fn difficulty(evm: &mut Evm) {
    evm.stack.push(evm.block_env.difficulty).unwrap();
    evm.pc += 1;
}

pub fn blockhash(evm: &mut Evm) {
    evm.stack.pop(); // block number (simplified: always return 0)
    evm.stack.push(evm.block_env.block_hash).unwrap();
    evm.pc += 1;
}
