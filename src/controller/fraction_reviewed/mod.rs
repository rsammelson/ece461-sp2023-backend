#[cfg(test)]
mod tests;

use crate::controller::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Correctness();

#[async_trait]
impl Scorer for Reviewed {
    
}
