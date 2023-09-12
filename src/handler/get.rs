use faststr::FastStr;
pub fn get(
    key: FastStr
) -> ::core::result::Result<volo_gen::mini_redis::ValueResponse, ::volo_thrift::AnyhowError> {
    let redis = super::model::REDIS.read();
    let value = redis.get(&key.to_string()).map(|v| v.to_string());
    Ok(volo_gen::mini_redis::ValueResponse {
        value: value.map(|v| v.into()),
    })
}
