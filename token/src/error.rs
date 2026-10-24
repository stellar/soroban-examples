use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenError {
    DecimalTooLarge = 1,
    NegativeAmount = 2,
    InsufficientBalance = 3,
    InsufficientAllowance = 4,
    InvalidLiveUntilLedger = 5,
}

pub type Error = TokenError;
