#![cfg(feature = "derive")]

use tynavi::{Selector, selector, traits::AsSelector};

#[derive(Selector)]
struct Unit;

#[derive(Selector)]
struct TupleUnit();

#[derive(Selector)]
enum EmptyEnum {}

#[derive(Debug, Clone, Copy)]
pub enum Role {
	Admin,
	Member,
}

impl Role {
	fn is_admin(&self) -> bool {
		matches!(self, Self::Admin)
	}

	fn is_member(&self) -> bool {
		matches!(self, Self::Member)
	}
}

#[derive(Selector)]
#[selector(rename = "UserDataSelector")]
struct UserData {
	#[selector(rename = "id")]
	user_id: i64,
	#[selector(skip(inspect))]
	name: String,
	#[selector(variants(admin, member))]
	role: Role,
}

#[derive(Selector)]
struct Pair(i64, String);

#[derive(Selector)]
pub struct Message {
	text: String,
}

#[selector]
pub enum Event {
	Ready,
	Message(Message),

	#[selector(rename = "MovedPayload", derive(Debug, Clone, Copy))]
	Moved(i64, i64),

	#[selector(rename = "JoinedPayload", derive(Debug, Clone, Copy))]
	Joined {
		user_id: i64,
		group_id: i64,
	},
}

#[test]
fn empty_types_only_generate_selector_entrypoints() {
	let unit = Unit;
	assert!(unit.as_selector().is_matched());

	let tuple = TupleUnit();
	assert!(tuple.as_selector().is_matched());
}

#[test]
fn struct_field_methods_work() {
	let user = UserData {
		user_id: 1,
		name: "alice".to_string(),
		role: Role::Admin,
	};

	assert_eq!(user.as_selector().route_id().select(), Some(&1));
	assert!(user.as_selector().id_filter(|id, _| *id > 0).is_matched());
	assert_eq!(
		user.as_selector().name_extract(|name, _| name.len()),
		Some(5)
	);
	assert!(user.as_selector().is_admin().is_matched());
	assert!(!user.as_selector().is_member().is_matched());
	assert!(!user.as_selector().not_is_admin().is_matched());
	assert!(user.as_selector().cond_is_admin(false).is_matched());

	let parent = user.as_selector().route_id().up();
	assert!(parent.is_matched());
}

#[test]
fn tuple_struct_uses_index_field_names() {
	let pair = Pair(7, "hello".to_string());
	assert_eq!(pair.as_selector().route_field_1().select(), Some(&7));
	assert!(
		pair
			.as_selector()
			.route_field_2()
			.contains("hell")
			.is_matched()
	);
}

#[test]
fn enum_methods_and_routes_work() {
	let event = Event::Message(Message {
		text: "hello".to_string(),
	});

	assert!(event.is_message());
	assert!(!event.is_ready());

	assert!(
		event
			.as_selector()
			.route_message()
			.route_text()
			.contains("hello")
			.is_matched()
	);

	let moved = Event::Moved(MovedPayload(1, 2));
	assert_eq!(
		moved.as_selector().route_moved().route_field_1().select(),
		Some(&1)
	);

	let joined = Event::Joined(JoinedPayload {
		user_id: 3,
		group_id: 4,
	});
	assert_eq!(
		joined.as_selector().route_joined().route_user_id().select(),
		Some(&3)
	);
}
