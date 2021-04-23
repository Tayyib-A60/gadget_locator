use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_lets_users_make_claims() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(LocatorModule::claim(Origin::signed(1), hash.clone()));
		assert_eq!(<Proofs<Test>>::get(hash), (1,1));
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(
// 			TemplateModule::cause_error(Origin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }
