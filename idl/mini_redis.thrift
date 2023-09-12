namespace py MiniRedis

enum Status {
  OK = 0,
  ERROR = 1,
}

struct KeyValue {
  1: string key,
  2: string value,
}

struct SetRequest {
  1: required string key,
  2: required string value,
}
struct GetRequest {
  1: required string key,
}
struct DelRequest {
  1: required string key,
}
struct ValueResponse {
  1: string value,
}
struct MessageResponse {
  1: string message,
  2: i32 code,
}
struct MessageGet{
  1: required string channel
  2: required i32 code,
}
struct MessagePublish {
  1: required string message,
  2: required string channel,
}

service MiniRedisService {
  Status ping()
  Status set(1: SetRequest request)
  ValueResponse get(1: GetRequest request)
  Status del(1: DelRequest request)
  Status publish(1: MessagePublish request)
  MessageResponse subscribe(1: MessageGet channel)
}
