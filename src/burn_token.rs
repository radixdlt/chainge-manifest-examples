//! An example for burning tokens.
//!
//! This example can be run through:
//!
//! ```
//! cargo run --bin burn-token
//! ```

use radix_transactions::prelude::*;
use resource_creator::*;
use scrypto::prelude::*;

pub fn main() {
    // The resource address of the resource we're minting. I just obtain an
    // example resource address.
    let resource_address = example_fungible_resource(1);

    // The account that we're minting this resource for and that we will deposit
    // the resources into.
    let account_address = example_account(1);

    // Constructing the manifest.
    let manifest = ManifestBuilder::new()
        // We're on a testnet so we lock a fee from the faucet. If you're on
        // mainnet then you'd lock a fee from an actual account.
        .lock_fee_from_faucet()
        // I am assuming that the tokens are in your account and that you can
        // withdraw them from your account. This step doesn't really matter, all
        // what matters is that you obtain the tokens somehow, and a simple way
        // is an account withdraw.
        .withdraw_from_account(account_address, resource_address, dec!(10_000))
        // Burning all of the resource that is available in the worktop which is
        // the entire amount that we withdrew from the account.
        .burn_all_from_worktop(resource_address)
        .build();
    print_manifest(&manifest);
}
