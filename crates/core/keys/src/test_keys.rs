//! Hardcoded test keys used by the `Default` genesis state and test code.

use once_cell::sync::Lazy;

use crate::{
    keys::{Bip44Path, SpendKey, WalletId},
    Address, FullViewingKey,
};

/// This address is for test purposes, allocations were added beginning with
/// the 062-Iapetus testnet.
/// Previously the test data was generated using BIP39 derivation starting with
/// the 016-Pandia testnet.
pub const SEED_PHRASE: &str = "comfort ten front cycle churn burger oak absent rice ice urge result art couple benefit cabbage frequent obscure hurry trick segment cool job debate";

/// The test account's spend key.
pub static SPEND_KEY: Lazy<SpendKey> = Lazy::new(|| {
    SpendKey::from_seed_phrase_bip44(
        SEED_PHRASE
            .parse()
            .expect("hardcoded test seed phrase should be valid"),
        &Bip44Path::new(0),
    )
});

/// The test account's full viewing key.
pub static FULL_VIEWING_KEY: Lazy<FullViewingKey> = Lazy::new(|| {
    SPEND_KEY.full_viewing_key().clone()
});

/// The test account's addresses, derived from the spend key.
pub static ADDRESS_0: Lazy<Address> = Lazy::new(|| {
    SPEND_KEY.full_viewing_key().payment_address(0u32.into()).0
});

pub static ADDRESS_1: Lazy<Address> = Lazy::new(|| {
    SPEND_KEY.full_viewing_key().payment_address(1u32.into()).0
});

pub static WALLET_ID: Lazy<WalletId> = Lazy::new(|| FULL_VIEWING_KEY.wallet_id());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fvk_matches() {
        assert_eq!(*FULL_VIEWING_KEY, *SPEND_KEY.full_viewing_key());
    }
}