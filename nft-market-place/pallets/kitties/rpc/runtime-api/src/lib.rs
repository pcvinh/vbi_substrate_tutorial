#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	pub trait TestRuntimeApi
	{
		fn api_get_kitties_cnt() -> u64;
		//fn api_get_kitty() -> Kitty<>;
		//fn api_create_kitty() -> Hash;
	}
}
