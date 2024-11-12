//! An example for creating tokens that can only be minted by a specific key
//! pair.
//!
//! This example can be run through:
//!
//! ```
//! cargo run --bin create-token
//! ```

use radix_transactions::prelude::*;
use resource_creator::*;
use scrypto::prelude::*;

pub fn main() {
    // The public key of the key-pair that controls this token and that can mint
    // it.
    let public_key = {
        let public_key_hex = "02b2c548ed5b3130b7193d2001c9f3faefbf3bff0921cbbb251adcdaef8e1b5608";
        Secp256k1PublicKey::from_str(public_key_hex).unwrap()
    };

    // Creating an "AccessRule" or a definition of the rule for minting and
    // controlling the resource. Our rule will require that a signature from
    // the private key corresponding to this public key for the minting.
    let access_rule = rule!(require(NonFungibleGlobalId::from_public_key(&public_key)));

    // Constructing the manifest that creates the resource and sets the metadata
    // on it.
    let manifest = ManifestBuilder::new()
        // We're on a testnet so we lock a fee from the faucet. If you're on
        // mainnet then you'd lock a fee from an actual account.
        .lock_fee_from_faucet()
        // Creating the resource or the token with the rules and configuration
        // that we want.
        .create_fungible_resource(
            // This defines who owns the resource and therefore who will be able
            // to update and change the resource's metadata in the future. We've
            // set this to be the public key seen above and we've set it to be
            // "Updatable" which means that it can be changed later.
            OwnerRole::Updatable(access_rule.clone()),
            // Controls whether the total supply of the resource is available on
            // ledger or not. This is just an optimization and most resources do
            // set it to be `true`.
            true,
            // The divisibility of the resource or how many decimal places the
            // resource has. The constant I'm using here will make the resource
            // have a divisibility of 18.
            DIVISIBILITY_MAXIMUM,
            // The "resource roles" define the behaviors of the resource as well
            // as who can perform them. This is where the minting and burning
            // behavior is defined.
            FungibleResourceRoles {
                // This controls who can mint the resource. We will set it to be
                // our public key defined above.
                mint_roles: mint_roles! {
                    minter => access_rule.clone();
                    minter_updater => access_rule.clone();
                },
                // This controls who can burn the resource. We will set it to be
                // our public key defined above.
                burn_roles: burn_roles! {
                    burner => access_rule.clone();
                    burner_updater => access_rule.clone();
                },
                // When we set this to `None` it means that we do not want to
                // override the default. By default freezing the resource is not
                // allowed by anyone.
                freeze_roles: None,
                // When we set this to `None` it means that we do not want to
                // override the default. By default recalling the resource is
                // not allowed by anyone.
                recall_roles: None,
                // When we set this to `None` it means that we do not want to
                // override the default. By default anyone can withdraw the
                // resource from THEIR account if they have it.
                withdraw_roles: None,
                // When we set this to `None` it means that we do not want to
                // override the default. By default anyone can deposit the
                // resource into THEIR account if they have it.
                deposit_roles: None,
            },
            metadata! {
                init {
                    "name" => "Chainge Wrapped USDC", locked;
                    "symbol" => "c.USDC", locked;
                }
            },
            // This controls whether we want to mint some initial supply or not
            // which we do not want at the moment.
            None,
        )
        .build();
    print_manifest(&manifest);
}
