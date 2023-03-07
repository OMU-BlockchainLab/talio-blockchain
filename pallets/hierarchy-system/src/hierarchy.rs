//use scale_info::prelude::*;
use super::*;

pub type AccountOf<T> = <T as frame_system::Config>::AccountId;

use pallet_utils::RoleId;
use crate::Config;

// #[derive(Encode, Decode, TypeInfo,RuntimeDebug)]
// pub struct Hierarchy<T: Config> {
//     node_list: Vec<AccountOf<T>>,
//     tree: HashMap<Parent<T>, Vec<Child<T>>>,
//     parents: HashMap<AccountOf<T>, Parent<T>>,

// }

// impl<T:Config> Hierarchy<T> {

//     // create a new structure
//     pub fn new() -> Hierarchy<T> {
//         Hierarchy {
//             node_list: Vec::default(),
//             parents: HashMap::default(),
//             tree: HashMap::default(),
//         }
//     }

// }

pub trait Hierarchy {
	type Account;
	fn add_root_node(root_node: Self::Account) -> Self::Account;
	fn add_sub_node(
		parent: Self::Account,
		node: &Self::Account,
	) -> Self::Account;
	fn attach_child(parent: Self::Account, child: Self::Account);
	fn is_parent(parent: Self::Account, child: &Self::Account) -> bool;

	fn iter_child(parent: &Self::Account) -> Vec<Self::Account>;
}

impl<T: Config> Hierarchy for Pallet<T> {
	type Account = RoleId;
	fn add_root_node(root_node: Self::Account) -> Self::Account {
		let mut node_list = Self::node_list();

		node_list.push(root_node.clone());
		<Node<T>>::put(node_list);
		// <RoleLayer<T>>::insert();
		root_node
	}

	fn add_sub_node(
		parent: Self::Account,
		node: &Self::Account,
	) -> Self::Account {
		let sub_node = Self::add_root_node(node.clone());

		let current_parent_layer = Self::role_layer(&parent);

		//let parents = Self::parents().unwrap();
		<Parents<T>>::insert(&sub_node, &parent);
		<RoleLayer<T>>::insert(&node, current_parent_layer + 1);
		Self::attach_child(parent, node.to_owned());
		sub_node
	}

	fn attach_child(parent: Self::Account, child: Self::Account) {
		//let tree = Self::tree(parent);
		<Tree<T>>::mutate(parent, |childs| {
			childs.push(child);
		});
	}

	fn is_parent(parent: Self::Account, child: &Self::Account) -> bool {
		if let Some(parent_of_child) = Self::parents(child) {
			if parent_of_child == parent {
				return true
			} else {
				return false
			}
		} else {
			return false
		}
	}

	fn iter_child(parent: &Self::Account) -> Vec<Self::Account> {
		let childs = Self::tree(parent);
		childs
	}
}
