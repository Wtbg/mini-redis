use faststr::FastStr;
pub fn set(
    key: FastStr,
    value: FastStr
) -> ::core::result::Result<volo_gen::mini_redis::Status, ::volo_thrift::AnyhowError> {
    let mut redis = super::model::REDIS.write();
    redis.insert(key.to_string(), value.to_string());
    Ok(Default::default())
}
