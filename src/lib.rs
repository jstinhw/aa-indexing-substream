mod pb;
pub mod abi;
mod filtering;
mod db;

use crate::pb::aa::Events;
use pb::aa::events;
use substreams::log;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth;
use substreams::store::StoreAddInt64;
use substreams::store::StoreNew;
use substreams::store::StoreAdd;
use substreams::prelude::*;
use substreams::pb::substreams::Clock;

#[substreams::handlers::map]
fn map_contract(block: eth::v2::Block) -> Result<Events, substreams::errors::Error> {
    use abi::entrypoint::events::UserOperationEvent;

    let mut events = Events::default();
    let mut user_op_sender: Vec<events::UserOpSender> = vec![];
    
    for tx in block.transactions() {
        tx.receipt().logs().for_each(|log| {
            if UserOperationEvent::match_log(log.as_ref()) {
                log::info!("UserOp: sender {:?}", user_op_sender);
                filtering::extract_user_operation(
                    &mut user_op_sender,
                    log.as_ref(),
                )
            }
        });
    }

    events.user_op_sender = user_op_sender;
    Ok(events)
}

#[substreams::handlers::store]
pub fn store_user_operation_sender_num(aa_events: Events, store: StoreAddInt64) {
    log::info!("store_user_operation_sender_num: {:?}", aa_events.user_op_sender);
    aa_events.user_op_sender.iter().for_each(|uo_sender| {
        store.add(
            0, 
            format!("user_op_sender:{}", uo_sender.sender.clone()),
            1
        );
    });
}

#[substreams::handlers::map]
pub fn graph_out(
    clock: Clock,
    derived_userop_sender_deltas: Deltas<DeltaInt64>,
) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    if clock.number == 93635673 {
        db::info_created_entity_change(&mut tables);
    }
    db::update_userop_sender(
        &mut tables,
        &derived_userop_sender_deltas
    );

    Ok(tables.to_entity_changes())
}
