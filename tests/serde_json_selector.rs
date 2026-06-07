#![cfg(feature = "serde_json")]

use serde_json::{Map, Value, json};
use tynavi::traits::AsSelector;

#[test]
fn value_type_checks() {
	assert!(json!(null).as_selector().is_null().is_matched());
	assert!(!json!(null).as_selector().not_is_null().is_matched());
	assert!(json!(true).as_selector().is_bool().is_matched());
	assert!(json!(42).as_selector().is_number().is_matched());
	assert!(json!("hello").as_selector().is_string().is_matched());
	assert!(json!([1, 2, 3]).as_selector().is_array().is_matched());
	assert!(
		json!({"key": "value"})
			.as_selector()
			.is_object()
			.is_matched()
	);
}

#[test]
fn value_cond_type_checks() {
	assert!(json!(42).as_selector().cond_is_number(true).is_matched());
	assert!(json!(42).as_selector().cond_is_number(false).is_matched());
	assert!(!json!(42).as_selector().cond_is_string(true).is_matched());
	assert!(
		!json!(42)
			.as_selector()
			.cond_not_is_number(true)
			.is_matched()
	);
	assert!(
		json!(42)
			.as_selector()
			.cond_not_is_string(true)
			.is_matched()
	);
}

#[test]
fn value_routing() {
	assert!(json!(null).as_selector().as_null().is_matched());
	assert_eq!(json!(true).as_selector().as_bool().select(), Some(&true));
	assert_eq!(
		json!(42)
			.as_selector()
			.as_number()
			.select()
			.map(|n| n.as_u64()),
		Some(Some(42))
	);
	assert_eq!(
		json!("hello").as_selector().as_str().select(),
		Some(&"hello".to_string())
	);
	assert_eq!(
		json!({"a": 1})
			.as_selector()
			.as_object()
			.select()
			.unwrap()
			.get("a"),
		Some(&json!(1))
	);
}

#[test]
fn value_navigation() {
	let json = json!({"users": [{"name": "alice"}]});
	assert!(
		json
			.as_selector()
			.keyof("users")
			.first()
			.keyof("name")
			.as_str()
			.contains("alice")
			.is_matched()
	);

	assert_eq!(
		json
			.as_selector()
			.keyof("users")
			.indexof(0)
			.keyof("name")
			.as_str()
			.select(),
		Some(&"alice".to_string())
	);

	assert_eq!(
		json
			.as_selector()
			.keyof("users")
			.first()
			.keyof("name")
			.select(),
		Some(&Value::String("alice".to_string()))
	);
}

#[test]
fn value_empty() {
	assert!(json!(null).as_selector().empty().is_matched());
	assert!(json!("").as_selector().empty().is_matched());
	assert!(json!([]).as_selector().empty().is_matched());
	assert!(json!({}).as_selector().empty().is_matched());
	assert!(!json!(42).as_selector().empty().is_matched());

	assert!(!json!(null).as_selector().not_empty().is_matched());
	assert!(!json!("").as_selector().not_empty().is_matched());
	assert!(!json!([]).as_selector().not_empty().is_matched());
	assert!(!json!({}).as_selector().not_empty().is_matched());
	assert!(json!(42).as_selector().not_empty().is_matched());

	assert!(json!("").as_selector().cond_empty(true).is_matched());
	assert!(json!("").as_selector().cond_empty(false).is_matched());
	assert!(!json!("x").as_selector().cond_empty(true).is_matched());
}

#[test]
fn map_operations() {
	let mut map = Map::new();
	map.insert("key".to_string(), json!("value"));

	assert_eq!(
		map.as_selector().keyof("key").select(),
		Some(&json!("value"))
	);
	assert!(map.as_selector().contains_key("key").is_matched());
	assert!(!map.as_selector().contains_key("missing").is_matched());
	assert!(map.as_selector().not_contains_key("missing").is_matched());
	assert!(map.as_selector().not_empty().is_matched());
	assert!(!map.as_selector().empty().is_matched());

	assert!(
		map
			.as_selector()
			.cond_contains_key(true, "key")
			.is_matched()
	);
	assert!(
		!map
			.as_selector()
			.cond_contains_key(true, "missing")
			.is_matched()
	);
	assert!(
		map
			.as_selector()
			.cond_contains_key(false, "missing")
			.is_matched()
	);
}
