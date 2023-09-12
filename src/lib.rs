#![feature(impl_trait_in_assoc_type)]
mod handler;
pub struct S;
#[volo::async_trait]
impl volo_gen::mini_redis::MiniRedisService for S {
    async fn ping(
        &self
    ) -> ::core::result::Result<volo_gen::mini_redis::Status, ::volo_thrift::AnyhowError> {
        Ok(Default::default())
    }
    async fn set(
        &self,
        _request: volo_gen::mini_redis::SetRequest
    ) -> ::core::result::Result<volo_gen::mini_redis::Status, ::volo_thrift::AnyhowError> {
        handler::set::set(_request.key, _request.value)
    }
    async fn get(
        &self,
        _request: volo_gen::mini_redis::GetRequest
    ) -> ::core::result::Result<volo_gen::mini_redis::ValueResponse, ::volo_thrift::AnyhowError> {
        handler::get::get(_request.key)
    }
    async fn del(
        &self,
        _request: volo_gen::mini_redis::DelRequest
    ) -> ::core::result::Result<volo_gen::mini_redis::Status, ::volo_thrift::AnyhowError> {
        handler::del::del(_request.key)
    }
}

