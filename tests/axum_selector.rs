#![cfg(feature = "axum")]

use axum::extract::{Json, Path, Query, State};
use tynavi::traits::AsSelector;

#[test]
fn path_extractor_routing() {
	let path = Path("hello".to_string());
	assert_eq!(
		path.as_selector().inner().select(),
		Some(&"hello".to_string())
	);
}

#[test]
fn query_extractor_routing() {
	let query = Query(vec![("key", "value")]);
	assert_eq!(
		query.as_selector().inner().select(),
		Some(&vec![("key", "value")])
	);
}

#[test]
fn json_extractor_routing() {
	let json = Json(42i32);
	assert_eq!(json.as_selector().inner().select(), Some(&42));
	assert!(json.as_selector().inner().eq(&42).is_matched());
}

#[test]
fn state_extractor_routing() {
	let state = State("app_state".to_string());
	assert_eq!(
		state.as_selector().inner().select(),
		Some(&"app_state".to_string())
	);
	assert!(state.as_selector().inner().starts_with("app").is_matched());
}

#[test]
fn extractor_chaining() {
	let json = Json(vec![1, 2, 3]);
	assert!(
		json
			.as_selector()
			.inner()
			.filter(|v, _| !v.is_empty())
			.is_matched()
	);

	let path = Path(100u32);
	assert!(path.as_selector().inner().gt(&50).is_matched());
}
