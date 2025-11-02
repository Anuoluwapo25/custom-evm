//! Module for Errors on the evm implemention

#[derive(Debug)]
pub enum EvmErrors {
    StackOverFlow,
    StackTooDeep,
    StackUnderflow,
}
