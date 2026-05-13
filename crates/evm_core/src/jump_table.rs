use crate::{
    Evm,
    opcodes::Opcode,
    operations::{
        arith::*,
        bitwise::*,
        comparison::*,
        control::*,
        environment::*,
        memory_ops::*,
        stack_ops::*,
        storage::*,
    },
};

pub type OpcodeFN = fn(&mut Evm);

fn noop(_evm: &mut Evm) {}

pub fn build_jump_table() -> [OpcodeFN; 256] {
    let mut t = [noop as OpcodeFN; 256];

    // 0x00 — Stop and arithmetic
    t[Opcode::STOP as usize] = stop;
    t[Opcode::ADD as usize] = add;
    t[Opcode::MUL as usize] = mul;
    t[Opcode::SUB as usize] = sub;
    t[Opcode::DIV as usize] = div;
    t[Opcode::SDIV as usize] = sdiv;
    t[Opcode::MOD as usize] = modulo;
    t[Opcode::SMOD as usize] = smod;
    t[Opcode::ADDMOD as usize] = addmod;
    t[Opcode::MULMOD as usize] = mulmod;
    t[Opcode::EXP as usize] = exp;
    t[Opcode::SIGNEXTEND as usize] = signextend;

    // 0x10 — Comparison
    t[Opcode::LT as usize] = lt;
    t[Opcode::GT as usize] = gt;
    t[Opcode::SLT as usize] = slt;
    t[Opcode::SGT as usize] = sgt;
    t[Opcode::EQ as usize] = eq;
    t[Opcode::ISZERO as usize] = iszero;

    // 0x16 — Bitwise
    t[Opcode::AND as usize] = and;
    t[Opcode::OR as usize] = or;
    t[Opcode::XOR as usize] = xor;
    t[Opcode::NOT as usize] = not;
    t[Opcode::BYTE as usize] = byte;
    t[Opcode::SHL as usize] = shl;
    t[Opcode::SHR as usize] = shr;
    t[Opcode::SAR as usize] = sar;

    // 0x30 — Environment
    t[Opcode::ADDRESS as usize] = address;
    t[Opcode::ORIGIN as usize] = origin;
    t[Opcode::CALLER as usize] = caller;
    t[Opcode::CALLVALUE as usize] = callvalue;
    t[Opcode::CALLDATALOAD as usize] = calldataload;
    t[Opcode::CALLDATASIZE as usize] = calldatasize;
    t[Opcode::CALLDATACOPY as usize] = calldatacopy;
    t[Opcode::CODESIZE as usize] = codesize;
    t[Opcode::GASPRICE as usize] = gasprice;

    // 0x40 — Block environment
    t[Opcode::BLOCKHASH as usize] = blockhash;
    t[Opcode::COINBASE as usize] = coinbase;
    t[Opcode::TIMESTAMP as usize] = timestamp;
    t[Opcode::NUMBER as usize] = blocknumber;
    t[Opcode::DIFFICULTY as usize] = difficulty;
    t[Opcode::GASLIMIT as usize] = gaslimit;
    t[Opcode::CHAINID as usize] = chainid;
    t[Opcode::SELFBALANCE as usize] = selfbalance;
    t[Opcode::BASEFEE as usize] = basefee;

    // 0x50 — Stack, memory, storage, flow
    t[Opcode::POP as usize] = pop;
    t[Opcode::MLOAD as usize] = mload;
    t[Opcode::MSTORE as usize] = mstore;
    t[Opcode::MSTORE8 as usize] = mstore8;
    t[Opcode::SLOAD as usize] = sload;
    t[Opcode::SSTORE as usize] = sstore;
    t[Opcode::JUMP as usize] = jump;
    t[Opcode::JUMPI as usize] = jumpi;
    t[Opcode::PC as usize] = pc;
    t[Opcode::MSIZE as usize] = msize;
    t[Opcode::GAS as usize] = gas;
    t[Opcode::JUMPDEST as usize] = jumpdest;

    // 0x5F..0x7F — PUSH
    t[Opcode::PUSH0 as usize] = push0;
    t[Opcode::PUSH1 as usize] = push1;
    t[Opcode::PUSH2 as usize] = push2;
    t[Opcode::PUSH3 as usize] = push3;
    t[Opcode::PUSH4 as usize] = push4;
    t[Opcode::PUSH5 as usize] = push5;
    t[Opcode::PUSH6 as usize] = push6;
    t[Opcode::PUSH7 as usize] = push7;
    t[Opcode::PUSH8 as usize] = push8;
    t[Opcode::PUSH9 as usize] = push9;
    t[Opcode::PUSH10 as usize] = push10;
    t[Opcode::PUSH11 as usize] = push11;
    t[Opcode::PUSH12 as usize] = push12;
    t[Opcode::PUSH13 as usize] = push13;
    t[Opcode::PUSH14 as usize] = push14;
    t[Opcode::PUSH15 as usize] = push15;
    t[Opcode::PUSH16 as usize] = push16;
    t[Opcode::PUSH17 as usize] = push17;
    t[Opcode::PUSH18 as usize] = push18;
    t[Opcode::PUSH19 as usize] = push19;
    t[Opcode::PUSH20 as usize] = push20;
    t[Opcode::PUSH21 as usize] = push21;
    t[Opcode::PUSH22 as usize] = push22;
    t[Opcode::PUSH23 as usize] = push23;
    t[Opcode::PUSH24 as usize] = push24;
    t[Opcode::PUSH25 as usize] = push25;
    t[Opcode::PUSH26 as usize] = push26;
    t[Opcode::PUSH27 as usize] = push27;
    t[Opcode::PUSH28 as usize] = push28;
    t[Opcode::PUSH29 as usize] = push29;
    t[Opcode::PUSH30 as usize] = push30;
    t[Opcode::PUSH31 as usize] = push31;
    t[Opcode::PUSH32 as usize] = push32;

    // 0x80..0x8F — DUP
    t[Opcode::DUP1 as usize] = dup1;
    t[Opcode::DUP2 as usize] = dup2;
    t[Opcode::DUP3 as usize] = dup3;
    t[Opcode::DUP4 as usize] = dup4;
    t[Opcode::DUP5 as usize] = dup5;
    t[Opcode::DUP6 as usize] = dup6;
    t[Opcode::DUP7 as usize] = dup7;
    t[Opcode::DUP8 as usize] = dup8;
    t[Opcode::DUP9 as usize] = dup9;
    t[Opcode::DUP10 as usize] = dup10;
    t[Opcode::DUP11 as usize] = dup11;
    t[Opcode::DUP12 as usize] = dup12;
    t[Opcode::DUP13 as usize] = dup13;
    t[Opcode::DUP14 as usize] = dup14;
    t[Opcode::DUP15 as usize] = dup15;
    t[Opcode::DUP16 as usize] = dup16;

    // 0x90..0x9F — SWAP
    t[Opcode::SWAP1 as usize] = swap1;
    t[Opcode::SWAP2 as usize] = swap2;
    t[Opcode::SWAP3 as usize] = swap3;
    t[Opcode::SWAP4 as usize] = swap4;
    t[Opcode::SWAP5 as usize] = swap5;
    t[Opcode::SWAP6 as usize] = swap6;
    t[Opcode::SWAP7 as usize] = swap7;
    t[Opcode::SWAP8 as usize] = swap8;
    t[Opcode::SWAP9 as usize] = swap9;
    t[Opcode::SWAP10 as usize] = swap10;
    t[Opcode::SWAP11 as usize] = swap11;
    t[Opcode::SWAP12 as usize] = swap12;
    t[Opcode::SWAP13 as usize] = swap13;
    t[Opcode::SWAP14 as usize] = swap14;
    t[Opcode::SWAP15 as usize] = swap15;
    t[Opcode::SWAP16 as usize] = swap16;

    // 0xF3, 0xFD, 0xFE — Return, Revert, Invalid
    t[Opcode::RETURN as usize] = ret;
    t[Opcode::REVERT as usize] = revert;
    t[Opcode::INVALID as usize] = invalid;

    t
}
