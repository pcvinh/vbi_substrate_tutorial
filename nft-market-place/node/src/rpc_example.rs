use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc]
pub trait ExampleRpc {
	#[rpc(name = "my_rpc")]
	fn get_value(&self) -> Result<u32>;
}

pub struct Example;

impl ExampleRpc for Example {
	fn get_value(&self) -> Result<u32> {
		Ok(5)
	}
}
