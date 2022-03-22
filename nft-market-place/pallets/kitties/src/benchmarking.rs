//! Benchmarking setup for pallet-kitties

use super::*;

#[allow(unused)]
use crate::Pallet as SubstrateKitties;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
	/* verify {
		assert_eq!(Kitties::<T>::get(), Some(s));
	}*/

	impl_benchmark_test_suite!(SubstrateKitties, crate::mock::new_test_ext(), crate::mock::Test);
}
