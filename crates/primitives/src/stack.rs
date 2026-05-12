//! This is an impl of the "Stack" DS
//! - push and
//! - pop
use crate::{constants::MAX_STACK_DEPTH, errors::EvmErrors};
use alloy::primitives::U256;

#[derive(Debug, Default, Clone)]
pub struct Stack {
    data: Vec<U256>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(MAX_STACK_DEPTH as usize),
        }
    }

    /// Function intoduces a new item to the top of the stack
    pub fn push(&mut self, item: U256) -> Result<(), EvmErrors> {
        if self.data.len() >= MAX_STACK_DEPTH as usize {
            return Err(EvmErrors::StackOverFlow);
        }
        self.data.push(item);

        Ok(())
    }

    /// Function removes the last item in the stack and returns this item
    pub fn pop(&mut self) -> Option<U256> {
        return self.data.pop();
    }

    /// Peek at the item `depth` from the top (0 = top).
    pub fn peek(&self, depth: usize) -> Option<U256> {
        let len = self.data.len();
        if depth >= len {
            return None;
        }
        Some(self.data[len - 1 - depth])
    }

    /// Swap the top item with the item `depth` positions from the top (1 = second item).
    pub fn swap_top(&mut self, depth: usize) -> Result<(), EvmErrors> {
        let len = self.data.len();
        if depth == 0 || depth >= len {
            return Err(EvmErrors::StackUnderflow);
        }
        self.data.swap(len - 1, len - 1 - depth);
        Ok(())
    }
}
