use super::model::CHANNELS;
use faststr::FastStr;
pub fn subscribe(
    channel: FastStr,
    code: i32
) -> ::core::result::Result<::volo_gen::mini_redis::MessageResponse, ::volo_thrift::AnyhowError> {
    let channels = CHANNELS.read();
    let messages = channels.get(&channel.to_string()).cloned();
    let mut response = ::volo_gen::mini_redis::MessageResponse {
        message: None,
        code: None,
    };
    if let Some(messages) = messages {
        if code < (messages.len() as i32) {
            response.message = Some(messages[code as usize].clone().into());
            response.code = Some(code + 1);
        }
    }
    Ok(response)
}
