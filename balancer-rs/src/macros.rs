// NB: If you use these macros, remember that all dependencies have
// to be in scope where they are used.

/// Simple conversion from a string slice to an Address
#[macro_export]
macro_rules! addr {
    ($address: expr) => {
        Address::from_str($address).unwrap()
    };
}

#[macro_export]
macro_rules! pool_id {
    ($id: expr) => {
        PoolId::from_str($id).unwrap()
    };
}

/// Simple conversion from &[str] or [usize] to a [`U256`](crate::U256).
#[macro_export]
macro_rules! u256 {
    ($t: expr) => {{
        {
            U256::from_dec_str(&$t.to_string()).unwrap()
        }
    }};
}

/// Simple conversion from &[str] or [usize] to a [`I256`](crate::I256).
#[macro_export]
macro_rules! i256 {
    ($t: expr) => {{
        {
            I256::from_dec_str(&$t.to_string()).unwrap()
        }
    }};
}

/// Simple conversion from &[str] or [usize] to a [`SwapFeePercentage`](crate::SwapFeePercentage).
#[macro_export]
macro_rules! swap_fee {
    ($t: expr) => {{
        {
            SwapFeePercentage::from_human_readable_str(&$t.to_string()).unwrap()
        }
    }};
}
