
use substreams_entity_change::tables::Tables;
use substreams::prelude::DeltaInt64;
use substreams::key;
use substreams::scalar::BigInt;
use substreams::store::Deltas;

pub fn info_created_entity_change(tables: &mut Tables) {
  tables.create_row("ERC4337Info", "0x4337")
    .set("numFactory", BigInt::zero())
    .set("numSmartAccount", BigInt::zero())
    .set("numModule", BigInt::zero())
    .set("numOwner", BigInt::zero());
}

pub fn update_userop_sender(
  tables: &mut Tables, 
  user_op_sender_store: &Deltas<DeltaInt64>
) {
  for delta in user_op_sender_store.iter() {
    let key = key::first_segment(delta.get_key());
    let address = format!("0x{}", key::segment_at(delta.get_key(), 1));

    if key == "user_op_sender" {
      tables.update_row("SmartAccount", &address)
        .set("address", hex::decode(&address[2..]).unwrap())
        .set("numUserOp", delta.new_value);
    }
  }
}