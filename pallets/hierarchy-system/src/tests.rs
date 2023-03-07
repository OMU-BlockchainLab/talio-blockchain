use crate::mock::*;
use frame_support::assert_ok;
use pallet_utils::RoleStatus;

#[test]
fn sysman_working_for_3_layer() {
	new_test_ext().execute_with(|| {
		// check initial hierarchy tree
		let sys_mans = Hierarchy::sys_role();
		let role_id_root = [0;36];
		assert_eq!(sys_mans, vec![role_id_root]);
		let role_id_alice = [1;36];
		let role_id_bob = [2;36];
		let role_id_eve = [3;36];
		assert_eq!(Hierarchy::role_layer(role_id_alice),0u8 );
		assert_eq!(Hierarchy::role_layer(role_id_bob),0u8 );
		assert_eq!(Hierarchy::node_list(), vec![role_id_root]);
		let empty_tree: Vec<[u8;36]> = Vec::new();
		assert_eq!(Hierarchy::tree(role_id_root), empty_tree);

		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_id_alice,
			Role::SysMan,
		));

		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_id_bob,
			Role::SysMan,
		));


		assert_ok!(Hierarchy::approve_sysman(Origin::signed(ROOT),role_id_root, vec![role_id_alice, role_id_bob]));
		// check parent for ALICE
		assert_eq!(Hierarchy::parents(role_id_alice).unwrap(), role_id_root);
		// check parent for BOB
		assert_eq!(Hierarchy::parents(role_id_bob).unwrap(), role_id_root);
		// check parent for EVE
		assert_eq!(Hierarchy::parents(role_id_eve), None);

		//let role_id_eve = [3;36];
		// Register EVE is normal user
		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_id_eve,
			Role::SysMan,
		));

		// Add Sys man for EVE
		assert_ok!(Hierarchy::approve_sysman(Origin::signed(ALICE),role_id_alice, vec![role_id_eve]));

		assert_eq!(Hierarchy::role_layer(role_id_eve),3u8 );
		assert_eq!(Hierarchy::node_list(), vec![role_id_root, role_id_alice, role_id_bob, role_id_eve]);
		assert_eq!(Hierarchy::tree(role_id_alice), vec![role_id_eve]);
		// check parent for EVE
		assert_eq!(Hierarchy::parents(role_id_eve).unwrap(), role_id_alice);

		let role_id_charlie = [4;36];
		// Register Charlie is normal user
		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_id_charlie,
			Role::SysMan,
		));

		// Add sysman for CHARLIE
		assert_ok!(Hierarchy::approve_sysman(Origin::signed(ALICE),role_id_alice, vec![role_id_charlie]));
		assert_eq!(Hierarchy::role_layer(role_id_charlie),3u8 );
		assert_eq!(Hierarchy::node_list(), vec![role_id_root, role_id_alice, role_id_bob, role_id_eve, role_id_charlie]);
		assert_eq!(Hierarchy::tree(role_id_alice), vec![role_id_eve, role_id_charlie]);
		// check parent for EVE
		assert_eq!(Hierarchy::parents(role_id_charlie).unwrap(), role_id_alice);

		// Revoke EVE out of sysman role
		assert_ok!(Hierarchy::revoke_sysman(Origin::signed(ALICE), role_id_alice,role_id_eve));

		assert_eq!(Hierarchy::node_list(), vec![role_id_root, role_id_alice, role_id_bob, role_id_charlie]);
		assert_eq!(Hierarchy::tree(role_id_alice), vec![role_id_charlie]);
		// check parent for CHARLIE
		assert_eq!(Hierarchy::parents(role_id_charlie).unwrap(), role_id_alice);
		// check parent for EVE
		assert_eq!(Hierarchy::parents(role_id_eve), None);

		// Revoke CHARLIE out of sysman role
		assert_ok!(Hierarchy::revoke_sysman(Origin::signed(ALICE),role_id_alice,role_id_charlie));

		assert_eq!(Hierarchy::node_list(), vec![role_id_root, role_id_alice, role_id_bob]);
		let childs_of_alice_empty: Vec<[u8;36]> = vec![];
		assert_eq!(Hierarchy::tree(role_id_alice), childs_of_alice_empty);
		let childs_of_bob_empty: Vec<[u8;36]> = vec![];
		assert_eq!(Hierarchy::tree(role_id_bob), childs_of_bob_empty);

		// check parent for CHARLIE
		assert_eq!(Hierarchy::parents(role_id_charlie), None);
		// check parent for EVE
		assert_eq!(Hierarchy::parents(role_id_eve), None);
	})
}

#[test]
fn sysman_working_for_4_layer() {
	new_test_ext().execute_with(|| {
		// check initial hierarchy tree
		let role_id_root = [0;36];
		let sys_mans = Hierarchy::sys_role();
		assert_eq!(sys_mans, vec![role_id_root]);

		assert_eq!(Hierarchy::node_list(), vec![role_id_root]);
		let empty_tree:Vec<[u8;36]> = Vec::new();
		assert_eq!(Hierarchy::tree(role_id_root), empty_tree);

		let role_id_alice = [1;36];
		let role_id_bob = [2;36];

		let role_id_eve = [3;36];
		let role_id_charlie = [4;36];
		let role_id_steve  = [5;36];

		let role_id_mai = [6;36];

		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_id_alice,
			Role::SysMan,
		));

		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_id_bob,
			Role::SysMan,
		));
		assert_ok!(Hierarchy::approve_sysman(Origin::signed(ROOT),role_id_root, vec![role_id_alice, role_id_bob]));

		
		// Register EVE is normal user
		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_id_eve,
			Role::SysMan,
		));
		// Register Charlie is Sysman
		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_id_charlie,
			Role::SysMan,
		));

		// Add Sys man for EVE, CHARLIE
		assert_ok!(Hierarchy::approve_sysman(
			Origin::signed(ALICE),
			role_id_alice,
			vec![role_id_eve, role_id_charlie]
		));

		let sys_mans = Hierarchy::sys_role();
		assert_eq!(sys_mans, vec![role_id_root, role_id_alice, role_id_bob, role_id_eve, role_id_charlie]);

		// Register STEVE is SysMan
		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_id_steve,
			Role::SysMan,
		));

		// Add sysman for STEVE

		assert_ok!(Hierarchy::approve_sysman(Origin::signed(EVE),role_id_eve, vec![role_id_steve]));

		// Add sysman for MAI
		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_id_mai,
			Role::SysMan,
		));

		assert_ok!(Hierarchy::approve_sysman(Origin::signed(CHARLIE),role_id_charlie, vec![role_id_mai]));

		assert_eq!(Hierarchy::node_list(), vec![role_id_root, role_id_alice, role_id_bob, role_id_eve, role_id_charlie, role_id_steve, role_id_mai]);

		// Revoke EVE out of sysman role
		assert_ok!(Hierarchy::revoke_sysman(Origin::signed(ALICE), role_id_alice ,role_id_eve));

		assert_eq!(Hierarchy::node_list(), vec![role_id_root, role_id_alice, role_id_bob, role_id_charlie, role_id_mai]);

		// Revoke CHARLIE out of sysman role
		assert_ok!(Hierarchy::revoke_sysman(Origin::signed(ALICE), role_id_alice,role_id_charlie));

		assert_eq!(Hierarchy::node_list(), vec![role_id_root, role_id_alice, role_id_bob]);

		// check parent for CHARLIE
		assert_eq!(Hierarchy::parents(role_id_charlie), None);
		// check parent for EVE
		assert_eq!(Hierarchy::parents(role_id_eve), None);

		// Check approve organization
		let role_org_1 = [7;36];
		let role_org_2 = [8;36];

		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_org_1.clone(),
			Role::Organization,
		));

		assert_ok!(pallet_account::Pallet::<Test>::register_account(
			Origin::root(),
			role_org_2.clone(),
			Role::Organization,
		));

		let get_account_1 = pallet_account::Pallet::<Test>::account_storage(role_org_1.clone());
		assert_eq!(get_account_1.unwrap().status, RoleStatus::Pending);

		let get_account_2 = pallet_account::Pallet::<Test>::account_storage(role_org_2.clone());
		assert_eq!(get_account_2.unwrap().status, RoleStatus::Pending);

		assert_ok!(Hierarchy::approve_org(
			Origin::signed(ALICE),
			role_id_alice,
			vec![role_org_1.clone(), role_org_2.clone()]
		));

		let get_account_1 = pallet_account::Pallet::<Test>::account_storage(role_org_1.clone());
		assert_eq!(get_account_1.unwrap().status, RoleStatus::Active);

		let get_account_2 = pallet_account::Pallet::<Test>::account_storage(role_org_2.clone());
		assert_eq!(get_account_2.unwrap().status, RoleStatus::Active);
	})
}
