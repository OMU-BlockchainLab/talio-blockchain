// use super::*;
use crate::mock::*;
use frame_support::assert_ok;

// fn str2vec(s: &str) -> Vec<u8> {
// 	s.as_bytes().to_vec()
// }

#[test]
fn create_cv_should_work() {
	new_test_ext().execute_with(|| {
		let owner = [1;36];
		let cer1 = [0u8;16];
		let cv_id = [1u8;36];
		// Dispatch a signed extrinsic.
		assert_ok!(CvModule::create_item(Origin::signed(1), cv_id, owner, Some(vec![cer1]),));
	});
}

#[test]
fn update_cv_item_should_work() {
	new_test_ext().execute_with(|| {
		let owner = [1;36];
		let cv_id = [1u8;36];
		let cer1 = [1u8;16];

		// create item to test
		create_item();

		// update item
		let _ = CvModule::update_item(Origin::signed(1), cv_id, owner, Some(vec![cer1]));
	})
}

fn create_item() {
	let owner = [1;36];
	let cv_id = [1u8;36];
	let cer1 = [1u8;16];
	// Dispatch a signed extrinsic.
	assert_ok!(CvModule::create_item(Origin::signed(1), cv_id, owner, Some(vec![cer1]),));
}
