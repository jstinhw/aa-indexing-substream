use crate::abi;
use substreams::{log,Hex};
use substreams_ethereum::Event;
use substreams_ethereum::pb::eth::v2::Log;
use crate::events::UserOpSender;

pub fn extract_user_operation(
  user_op_sender: &mut Vec<UserOpSender>,
  log: &Log,
) {
  if let Some(user_op) = abi::entrypoint::events::UserOperationEvent::match_and_decode(log) {
    let sender_hex = Hex(user_op.sender.clone()).to_string();
    log::info!("extract_user_operation: sender: {:?}", sender_hex);
    user_op_sender.push(UserOpSender {
      log_ordinal: log.ordinal,
      sender: sender_hex
    });
  }
}
