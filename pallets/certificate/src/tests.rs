use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create_certificate() {
	new_test_ext().execute_with(|| {
		let cid = vec![1];
		assert_ok!(Certificate::create_certificate(
			Origin::signed(ALICE),
			cid.clone(),
			None,
			Some(BOB)
		));
		let cert = Certificate::certificate_by_id(cid).unwrap();
		assert_eq!(cert.org, Some(BOB));
		assert_eq!(cert.owner, ALICE);
		assert_eq!(cert.status, CerStatus::Unvalidated);
	});
}

#[test]
fn it_works_for_revoke_certificate() {
	new_test_ext().execute_with(|| {
		let cid = vec![1];
		assert_ok!(Certificate::create_certificate(
			Origin::signed(ALICE),
			cid.clone(),
			None,
			Some(BOB)
		));
		assert_ok!(Certificate::revoke_certificate(Origin::signed(ALICE), cid));
	});
}

#[test]
fn it_works_for_validate_certificate() {
	new_test_ext().execute_with(|| {
		let cid = vec![1];
		//create certificate
		assert_ok!(Certificate::create_certificate(
			Origin::signed(ALICE),
			cid.clone(),
			None,
			Some(CHARLIE)
		));

		let role_id = vec![1];
		// Register account
		assert_ok!(Account::register_account(Origin::root(), vec![1], Role::Organization));
		// validate organization role
		assert_ok!(Hierarchy::approve_org(Origin::signed(ALICE), role_id.clone(), CHARLIE));

		// validate certificate by organization
		assert_ok!(Certificate::validate_certificate(Origin::signed(CHARLIE), role_id, cid));
	});
}

#[test]
fn it_not_works_for_revoke_certificate() {
	new_test_ext().execute_with(|| {
		let cid = vec![1];
		assert_ok!(Certificate::create_certificate(
			Origin::signed(ALICE),
			cid.clone(),
			None,
			Some(BOB)
		));
		assert_noop!(
			Certificate::revoke_certificate(Origin::signed(CHARLIE), cid),
			Error::<Test>::NotOwnerCertificate
		);
	});
}
