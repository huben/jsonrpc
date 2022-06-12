use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::ServerBuilder;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use serde_json::{ json, from_value };
use serde_derive::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct AddParams {
	a: u64,
	b: u64,
}

#[rpc]
pub trait Rpc {
	#[rpc(name = "addWithMarco")]
	fn add(&self, a: u64, b: u64) -> Result<u64>;

	#[rpc(name = "addWithMarcoDesc")]
	fn add_params(&self, desc: String, add_params: AddParams) -> Result<Value>;
}

pub struct RpcImpl;
impl Rpc for RpcImpl {
	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}
	fn add_params(&self, desc: String, add_params: AddParams) -> Result<Value> {
		Ok(json!([desc, { "sum": add_params.a + add_params.b }]))
	}
}

fn main() {
	let mut io = IoHandler::default();

	io.add_method("say_hello", |_params: Params| async {
		Ok(Value::String("hello".to_owned()))
	});

	io.add_method("echo", |params: Params| async {
		Ok(params.into())
	});

	io.add_method("add", |params: Params| async {
		let parsed: [u64; 2] = params.parse().unwrap();
		let sum = parsed[0] + parsed[1];
		Ok(json!(sum))
	});

	io.add_method("addWithDesc", |params: Params| async {
		let parsed: [Value; 2] = params.parse().unwrap();
		let desc = &parsed[0];
		let second = &parsed[1];
		let add_params: AddParams = from_value(second.to_owned()).unwrap();
		Ok(json!([desc, { "sum": add_params.a + add_params.b }]))
	});

	io.extend_with(RpcImpl.to_delegate());

	let server = ServerBuilder::new(io)
		.threads(3)
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.unwrap();

	server.wait();
}
