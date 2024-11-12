//! An example for minting tokens into some account.
//!
//! This example can be run through:
//!
//! ```
//! cargo run --bin mint-token
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
        // Minting the resource. We're minting 10,000 of it. We're not like ETH
        // where you need to worry about how many decimal places there are. You
        // specify your amount in a human readable way.
        .mint_fungible(resource_address, dec!(10_000))
        // Attempt to deposit the resources into the user's account. We abort
        // if the deposit fails.
        .try_deposit_entire_worktop_or_abort(account_address, None)
        .build();
    print_manifest(&manifest);
}
