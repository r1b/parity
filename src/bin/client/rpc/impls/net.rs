//! Net rpc implementation.
use rpc::jsonrpc_core::*;
use rpc::Net;

pub struct NetClient;

impl NetClient {
	pub fn new() -> Self { NetClient }
}

impl Net for NetClient {
	fn version(&self, _: Params) -> Result<Value, Error> {
		Ok(Value::U64(63))
	}

	fn peer_count(&self, _params: Params) -> Result<Value, Error> {
		Ok(Value::U64(0))
	}
}
