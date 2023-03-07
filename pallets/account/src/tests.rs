use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

use pallet_utils::{Role, RoleStatus};

#[test]
fn it_works_for_register_account() {
	new_test_ext().execute_with(|| {
		let id = [1;36];

		assert_ok!(Account::register_account(Origin::root(), id.clone(), Role::Organization));
		let account = Account::account_storage(id).unwrap();
		assert_eq!(account.status, RoleStatus::Pending);
	});
}

#[test]
fn it_works_for_update_role() {
	new_test_ext().execute_with(|| {
		let id =[1;36];

		assert_ok!(Account::register_account(Origin::root(), id.clone(), Role::Organization));
		let account = Account::account_storage(id.clone()).unwrap();
		assert_eq!(account.status, RoleStatus::Pending);
		assert_eq!(account.role, Role::Organization);
		assert_ok!(Account::update_role(Origin::root(), id.clone(), Role::SysMan));
		let account = Account::account_storage(id).unwrap();
		assert_eq!(account.role, Role::SysMan);
	});
}
