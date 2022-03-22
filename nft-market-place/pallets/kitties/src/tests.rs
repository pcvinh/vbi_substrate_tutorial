use crate::{mock::*, Error, *};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create_kitty() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(1)));
		// Read pallet storage and assert an expected result.
		// assert_eq!(SubstrateKitties::something(), Some(42));
	});
}

/*#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(SubstrateKitties::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}*/


#[test]
fn should_working_set_price_kitty(){
	new_test_ext().execute_with(|| {
		//
		System::set_block_number(1);

		SubstrateKitties::create_kitty(Origin::signed(1));

		let kitty_id = SubstrateKitties::kitties_owned(1)[0];
		let new_price = Some(100u128);
		assert_ok!(SubstrateKitties::set_price(Origin::signed(1), kitty_id, new_price));
	});
}

#[test]
fn should_working_transfer_kitty(){
	new_test_ext().execute_with(|| {
		//
		System::set_block_number(1);

		SubstrateKitties::create_kitty(Origin::signed(1));

		let kitty_id = SubstrateKitties::kitties_owned(1)[0];
		assert_ok!(SubstrateKitties::transfer(Origin::signed(1), 2, kitty_id));
	});
}

#[test]
fn should_working_breed_kitty(){
	new_test_ext().execute_with(|| {
		//
		System::set_block_number(1);

		let mom_dna = Some([0u8; 16]);
		let dad_dna = Some([1u8; 16]);
		let created_time = &18u64;
		assert_ok!(SubstrateKitties::mint(&1, mom_dna, Some(Gender::Female)));
		assert_ok!(SubstrateKitties::mint(&1, dad_dna, Some(Gender::Male)));
		let mom_id = SubstrateKitties::kitties_owned(1)[0];
		let dad_id = SubstrateKitties::kitties_owned(1)[1];
		assert_ok!(SubstrateKitties::breed_kitty(Origin::signed(1), mom_id, dad_id));
	});
}

#[test]
fn should_working_buy_kitty(){
	new_test_ext().execute_with(|| {
		//
		System::set_block_number(1);

		SubstrateKitties::create_kitty(Origin::signed(1));

		let kitty_id = SubstrateKitties::kitties_owned(1)[0];
		let new_price = Some(0u128);
		SubstrateKitties::set_price(Origin::signed(1), kitty_id, new_price);

		let bid_price = 0u128;
		assert_ok!(SubstrateKitties::buy_kitty(Origin::signed(2), kitty_id, bid_price));

	});
}
