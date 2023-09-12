use super::model::CHANNELS;
use faststr::FastStr;
pub fn publish(
    channel: FastStr,
    message: FastStr
) -> ::core::result::Result<volo_gen::mini_redis::Status, ::volo_thrift::AnyhowError> {
    let mut channels = CHANNELS.write();
    let messages = channels.entry(channel.to_string()).or_default();
    messages.push(message.into_string());
    Ok(Default::default())
}
