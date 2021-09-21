use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Outer<T> {
    v: private::Inner<T>,
}

mod private {
    use super::*;

    #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
    pub(crate) struct Inner<T> {
        val: T,
    }
}
