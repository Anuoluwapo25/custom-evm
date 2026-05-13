use alloy::primitives::{Address, U256};
use evm_core::Evm;
use primitives::{
    evm_types::{BlockEnv, EvmAccount, EvmStorage, Transaction},
    memory::Memory,
    stack::Stack,
};

fn run_bytecode(bytecode: &[u8]) -> Evm {
    let mut storage = EvmStorage::default();
    let contract_addr = Address::repeat_byte(0x42);
    let caller_addr = Address::repeat_byte(0x01);

    storage.data.insert(
        contract_addr,
        EvmAccount {
            address: contract_addr,
            balance: U256::from(1000u64),
            nonce: U256::ZERO,
            code: bytecode.to_vec(),
            slots: Default::default(),
        },
    );

    let tx = Transaction {
        from: caller_addr,
        to: contract_addr,
        value: U256::from(100u64),
        gas_limit: U256::from(1_000_000u64),
        nonce: U256::ZERO,
        data: vec![],
    };

    let block_env = BlockEnv {
        number: U256::from(1u64),
        timestamp: U256::from(1_700_000_000u64),
        gas_limit: U256::from(30_000_000u64),
        base_fee: U256::from(7u64),
        coinbase: Address::repeat_byte(0xAA),
        difficulty: U256::ZERO,
        chain_id: U256::from(1u64),
        block_hash: U256::ZERO,
    };

    let mut evm = Evm::new(block_env, tx, Memory::new(), Stack::new(), storage);
    evm.setup();
    evm
}

fn main() {
    // Example 1: PUSH1 10, PUSH1 20, ADD, STOP  →  stack top should be 30
    println!("=== Example 1: 10 + 20 ===");
    let bytecode = [
        0x60, 0x0A, // PUSH1 10
        0x60, 0x14, // PUSH1 20
        0x01,       // ADD
        0x00,       // STOP
    ];
    let evm = run_bytecode(&bytecode);
    println!("Status : {:?}", evm.status);
    println!("Result : {}", evm.stack.peek(0).unwrap_or(U256::ZERO));
    println!();

    // Example 2: 7 mod 3 = 1
    // EVM MOD: a = top, b = second; result = a % b
    // Push 3 first (becomes second), push 7 (becomes top=a). Result = 7 % 3 = 1.
    println!("=== Example 2: 7 mod 3 ===");
    let bytecode = [
        0x60, 0x03, // PUSH1 3  (second / divisor)
        0x60, 0x07, // PUSH1 7  (top / dividend)
        0x06,       // MOD
        0x00,       // STOP
    ];
    let evm = run_bytecode(&bytecode);
    println!("Status : {:?}", evm.status);
    println!("Result : {}", evm.stack.peek(0).unwrap_or(U256::ZERO));
    println!();

    // Example 3: 2^10 = 1024
    // EVM EXP: a = top (base), b = second (exponent); result = a^b
    // Push 10 first (exponent), push 2 (base). Result = 2^10 = 1024.
    println!("=== Example 3: 2 ^ 10 ===");
    let bytecode = [
        0x60, 0x0A, // PUSH1 10  (exponent, second)
        0x60, 0x02, // PUSH1 2   (base, top)
        0x0A,       // EXP
        0x00,       // STOP
    ];
    let evm = run_bytecode(&bytecode);
    println!("Status : {:?}", evm.status);
    println!("Result : {}", evm.stack.peek(0).unwrap_or(U256::ZERO));
    println!();

    // Example 4: conditional jump — jumps over a PUSH, lands on JUMPDEST then STOP
    // EVM JUMPI: top = destination, second = condition
    // pc: 0  PUSH1 1    (condition, pushed first → second item)
    //     2  PUSH1 7    (destination, pushed second → top item)
    //     4  JUMPI      → pops dest=7, cond=1; 1≠0 so jump to pc=7
    //     5  PUSH1 0xFF (skipped)
    //     7  JUMPDEST
    //     8  STOP
    println!("=== Example 4: JUMPI (conditional) ===");
    let bytecode = [
        0x60, 0x01, // PUSH1 1   (condition, will be second)
        0x60, 0x07, // PUSH1 7   (destination, will be top)
        0x57,       // JUMPI
        0x60, 0xFF, // PUSH1 255 (should be skipped)
        0x5B,       // JUMPDEST  (pc=7)
        0x00,       // STOP
    ];
    let evm = run_bytecode(&bytecode);
    println!("Status : {:?}", evm.status);
    println!("Stack empty (0xFF skipped): {}", evm.stack.peek(0).is_none());

    println!("\nAll examples complete.");

    // Summary of what's implemented
    println!("\n--- Implemented opcodes ---");
    println!("Arithmetic : STOP ADD MUL SUB DIV SDIV MOD SMOD ADDMOD MULMOD EXP SIGNEXTEND");
    println!("Comparison : LT GT SLT SGT EQ ISZERO");
    println!("Bitwise    : AND OR XOR NOT BYTE SHL SHR SAR");
    println!("Memory     : MLOAD MSTORE MSTORE8 MSIZE");
    println!("Stack      : POP PUSH0..PUSH32 DUP1..DUP16 SWAP1..SWAP16");
    println!("Control    : JUMP JUMPI JUMPDEST PC GAS RETURN REVERT INVALID");
    println!("Env        : ADDRESS CALLER CALLVALUE CALLDATALOAD CALLDATASIZE CALLDATACOPY");
    println!("           : CODESIZE ORIGIN GASPRICE BLOCKHASH COINBASE TIMESTAMP");
    println!("           : NUMBER DIFFICULTY GASLIMIT CHAINID SELFBALANCE BASEFEE");
    println!("Storage    : SLOAD SSTORE");
}
