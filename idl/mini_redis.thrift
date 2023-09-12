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


service MiniRedisService {
  Status ping()
  Status set(1: SetRequest request)
  ValueResponse get(1: GetRequest request)
  Status del(1: DelRequest request)
}
