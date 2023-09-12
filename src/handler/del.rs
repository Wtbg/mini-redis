use faststr::FastStr;
#[allow(unused_imports)]
use crate::S;
pub fn del(
    key: FastStr
) -> ::core::result::Result<volo_gen::mini_redis::Status, ::volo_thrift::AnyhowError> {
    let mut redis = super::model::REDIS.write();
    match redis.remove(&key.to_string()) {
        Some(_) => Ok(volo_gen::mini_redis::Status::Ok),
        None => {
            Err(
                ::volo_thrift::AnyhowError::new(
                    ::std::io::Error::new(::std::io::ErrorKind::NotFound, "key not exist")
                )
            )
        }
    }
}
