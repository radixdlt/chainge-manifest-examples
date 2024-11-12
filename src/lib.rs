use radix_transactions::manifest::decompile;
use scrypto_test::prelude::*;

pub fn print_manifest(manifest: &TransactionManifestV1) {
    let manifest = decompile(&manifest.instructions, &NetworkDefinition::mainnet()).unwrap();
    println!("{manifest}");
}

pub fn example_account(n: usize) -> ComponentAddress {
    node_id(n, EntityType::GlobalAccount).try_into().unwrap()
}

pub fn example_fungible_resource(n: usize) -> ResourceAddress {
    node_id(n, EntityType::GlobalFungibleResourceManager)
        .try_into()
        .unwrap()
}

pub fn node_id(n: usize, entity_type: EntityType) -> NodeId {
    let hash = hash(n.to_le_bytes());
    let bytes = hash.0;
    let mut node_id = NodeId(bytes[0..30].try_into().unwrap());
    node_id.0[0] = entity_type as u8;
    node_id
}
