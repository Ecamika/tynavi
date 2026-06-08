#![cfg(feature = "http")]

use http::header::{HeaderMap, HeaderValue};
use http::{Method, Request, Response, StatusCode, Uri, Version};
use tynavi::traits::AsSelector;

#[test]
fn request_routing() {
	let req = Request::builder()
		.method(Method::GET)
		.uri("https://example.com/path?query=1")
		.version(Version::HTTP_11)
		.header("content-type", "application/json")
		.body("body")
		.unwrap();

	assert_eq!(req.as_selector().method().select(), Some(&Method::GET));
	assert_eq!(
		req.as_selector().uri().select(),
		Some(&"https://example.com/path?query=1".parse::<Uri>().unwrap())
	);
	assert!(
		req
			.as_selector()
			.headers()
			.contains_key("content-type")
			.is_matched()
	);
	assert_eq!(req.as_selector().body().select(), Some(&"body"));
}

#[test]
fn request_version_filtering() {
	let req = Request::builder()
		.method(Method::GET)
		.uri("/")
		.version(Version::HTTP_11)
		.body(())
		.unwrap();

	assert!(req.as_selector().version_eq(Version::HTTP_11).is_matched());
	assert!(!req.as_selector().version_eq(Version::HTTP_2).is_matched());
	assert!(
		req
			.as_selector()
			.version_not_eq(Version::HTTP_2)
			.is_matched()
	);
	assert!(
		req
			.as_selector()
			.cond_version_eq(true, Version::HTTP_11)
			.is_matched()
	);
	assert!(
		req
			.as_selector()
			.cond_version_eq(false, Version::HTTP_2)
			.is_matched()
	);
	assert!(req.as_selector().version_gt(Version::HTTP_10).is_matched());
	assert!(req.as_selector().version_lt(Version::HTTP_2).is_matched());
}

#[test]
fn response_routing() {
	let res = Response::builder()
		.status(StatusCode::OK)
		.version(Version::HTTP_2)
		.header("x-custom", "value")
		.body("body")
		.unwrap();

	assert!(
		res
			.as_selector()
			.headers()
			.contains_key("x-custom")
			.is_matched()
	);
	assert_eq!(res.as_selector().body().select(), Some(&"body"));
}

#[test]
fn response_status_category() {
	let res_200 = Response::builder().status(200).body(()).unwrap();
	let res_404 = Response::builder().status(404).body(()).unwrap();
	let res_500 = Response::builder().status(500).body(()).unwrap();
	let res_301 = Response::builder().status(301).body(()).unwrap();
	let res_100 = Response::builder().status(100).body(()).unwrap();

	assert!(res_200.as_selector().is_success().is_matched());
	assert!(res_404.as_selector().is_client_error().is_matched());
	assert!(res_500.as_selector().is_server_error().is_matched());
	assert!(res_301.as_selector().is_redirection().is_matched());
	assert!(res_100.as_selector().is_informational().is_matched());

	assert!(!res_200.as_selector().is_client_error().is_matched());
	assert!(res_200.as_selector().not_is_client_error().is_matched());
}

#[test]
fn response_status_cond_category() {
	let res = Response::builder().status(200).body(()).unwrap();
	assert!(res.as_selector().cond_is_success(true).is_matched());
	assert!(res.as_selector().cond_is_success(false).is_matched());
	assert!(!res.as_selector().cond_is_client_error(true).is_matched());
}

#[test]
fn response_status_comparison() {
	let res = Response::builder().status(404).body(()).unwrap();
	assert!(
		res
			.as_selector()
			.status_eq(StatusCode::NOT_FOUND)
			.is_matched()
	);
	assert!(!res.as_selector().status_eq(StatusCode::OK).is_matched());
	assert!(res.as_selector().status_not_eq(StatusCode::OK).is_matched());
	assert!(
		res
			.as_selector()
			.status_gt(StatusCode::BAD_REQUEST)
			.is_matched()
	);
	assert!(
		res
			.as_selector()
			.status_lt(StatusCode::INTERNAL_SERVER_ERROR)
			.is_matched()
	);
}

#[test]
fn response_version_filtering() {
	let res = Response::builder()
		.status(200)
		.version(Version::HTTP_2)
		.body(())
		.unwrap();

	assert!(res.as_selector().version_eq(Version::HTTP_2).is_matched());
	assert!(!res.as_selector().version_eq(Version::HTTP_11).is_matched());
}

#[test]
fn method_comparison() {
	assert!(Method::GET.as_selector().eq(&Method::GET).is_matched());
	assert!(!Method::GET.as_selector().eq(&Method::POST).is_matched());
	assert!(Method::POST.as_selector().not_eq(&Method::GET).is_matched());
}

#[test]
fn status_code_category() {
	assert!(
		StatusCode::CONTINUE
			.as_selector()
			.is_informational()
			.is_matched()
	);
	assert!(StatusCode::OK.as_selector().is_success().is_matched());
	assert!(
		StatusCode::MOVED_PERMANENTLY
			.as_selector()
			.is_redirection()
			.is_matched()
	);
	assert!(
		StatusCode::BAD_REQUEST
			.as_selector()
			.is_client_error()
			.is_matched()
	);
	assert!(
		StatusCode::INTERNAL_SERVER_ERROR
			.as_selector()
			.is_server_error()
			.is_matched()
	);

	assert!(!StatusCode::OK.as_selector().is_client_error().is_matched());
	assert!(
		StatusCode::OK
			.as_selector()
			.not_is_client_error()
			.is_matched()
	);
}

#[test]
fn status_code_cond_category() {
	assert!(
		StatusCode::OK
			.as_selector()
			.cond_is_success(true)
			.is_matched()
	);
	assert!(
		StatusCode::OK
			.as_selector()
			.cond_is_success(false)
			.is_matched()
	);
	assert!(
		!StatusCode::OK
			.as_selector()
			.cond_is_client_error(true)
			.is_matched()
	);
}

#[test]
fn status_code_comparison() {
	assert!(
		StatusCode::OK
			.as_selector()
			.eq(&StatusCode::OK)
			.is_matched()
	);
	assert!(
		StatusCode::NOT_FOUND
			.as_selector()
			.gt(&StatusCode::BAD_REQUEST)
			.is_matched()
	);
	assert!(
		StatusCode::NOT_FOUND
			.as_selector()
			.lt(&StatusCode::INTERNAL_SERVER_ERROR)
			.is_matched()
	);
}

#[test]
fn version_comparison() {
	assert!(
		Version::HTTP_11
			.as_selector()
			.eq(&Version::HTTP_11)
			.is_matched()
	);
	assert!(
		!Version::HTTP_11
			.as_selector()
			.eq(&Version::HTTP_2)
			.is_matched()
	);
}

#[test]
fn uri_string_filter() {
	let uri: Uri = "https://example.com/api/v1/users".parse().unwrap();
	assert!(uri.as_selector().starts_with("https://").is_matched());
	assert!(uri.as_selector().ends_with("/users").is_matched());
	assert!(uri.as_selector().contains("/api/").is_matched());
	assert!(!uri.as_selector().contains("/admin/").is_matched());
	assert!(uri.as_selector().not_contains("/admin/").is_matched());
}

#[test]
fn uri_cond_string_filter() {
	let uri: Uri = "https://example.com/api".parse().unwrap();
	assert!(
		uri
			.as_selector()
			.cond_starts_with(true, "https://")
			.is_matched()
	);
	assert!(
		uri
			.as_selector()
			.cond_starts_with(false, "http://")
			.is_matched()
	);
}

#[test]
fn uri_routing() {
	let uri: Uri = "https://example.com:8080/path?key=value".parse().unwrap();

	assert_eq!(uri.as_selector().path().select(), Some("/path"));
	assert_eq!(uri.as_selector().query().select(), Some("key=value"));
	assert_eq!(uri.as_selector().host().select(), Some("example.com"));
	assert_eq!(uri.as_selector().scheme().select(), Some("https"));
}

#[test]
fn uri_routing_missing() {
	let uri: Uri = "/path".parse().unwrap();
	assert!(!uri.as_selector().query().is_matched());
	assert!(!uri.as_selector().host().is_matched());
	assert!(!uri.as_selector().scheme().is_matched());
}

#[test]
fn uri_port_filtering() {
	let uri: Uri = "https://example.com:8080/path".parse().unwrap();
	let uri_no_port: Uri = "https://example.com/path".parse().unwrap();

	assert!(uri.as_selector().port_eq(8080).is_matched());
	assert!(!uri.as_selector().port_eq(80).is_matched());
	assert!(uri.as_selector().port_not_eq(80).is_matched());
	assert!(uri.as_selector().port_gt(1000).is_matched());
	assert!(uri.as_selector().port_lt(9000).is_matched());

	assert!(!uri_no_port.as_selector().port_eq(8080).is_matched());
	assert!(
		uri_no_port
			.as_selector()
			.cond_port_eq(false, 8080)
			.is_matched()
	);
}

#[test]
fn header_map_operations() {
	let mut map = HeaderMap::new();
	map.insert("content-type", HeaderValue::from_static("application/json"));
	map.insert("x-custom", HeaderValue::from_static("value"));

	assert!(!map.as_selector().empty().is_matched());
	assert!(map.as_selector().not_empty().is_matched());
	assert!(map.as_selector().contains_key("content-type").is_matched());
	assert!(!map.as_selector().contains_key("authorization").is_matched());
	assert!(
		map
			.as_selector()
			.not_contains_key("authorization")
			.is_matched()
	);

	assert_eq!(
		map
			.as_selector()
			.keyof("content-type")
			.select()
			.map(|v| v.to_str().unwrap()),
		Some("application/json")
	);
	assert!(!map.as_selector().keyof("missing").is_matched());
}

#[test]
fn header_value_string_filter() {
	let val = HeaderValue::from_static("application/json");
	assert!(val.as_selector().starts_with("application/").is_matched());
	assert!(val.as_selector().ends_with("/json").is_matched());
	assert!(val.as_selector().contains("json").is_matched());
	assert!(!val.as_selector().contains("xml").is_matched());
	assert!(!val.as_selector().empty().is_matched());
	assert!(val.as_selector().not_empty().is_matched());
}

#[test]
fn header_value_cond_string_filter() {
	let val = HeaderValue::from_static("text/plain");
	assert!(val.as_selector().cond_contains(true, "text").is_matched());
	assert!(val.as_selector().cond_contains(false, "html").is_matched());
}

#[test]
fn request_chaining() {
	let req = Request::builder()
		.method(Method::POST)
		.uri("https://api.example.com/v1/data")
		.header("authorization", "Bearer token123")
		.body(())
		.unwrap();

	assert!(req.as_selector().method().eq(&Method::POST).is_matched());

	assert!(
		req
			.as_selector()
			.uri()
			.path()
			.starts_with("/v1/")
			.is_matched()
	);

	assert!(
		req
			.as_selector()
			.headers()
			.contains_key("authorization")
			.is_matched()
	);

	assert_eq!(
		req
			.as_selector()
			.headers()
			.keyof("authorization")
			.select()
			.and_then(|v| v.to_str().ok()),
		Some("Bearer token123")
	);
}

#[test]
fn response_chaining() {
	let res = Response::builder()
		.status(StatusCode::NOT_FOUND)
		.body(())
		.unwrap();

	assert!(res.as_selector().is_client_error().is_matched());
	assert!(
		res
			.as_selector()
			.status_eq(StatusCode::NOT_FOUND)
			.is_matched()
	);
	assert!(
		res
			.as_selector()
			.status_gt(StatusCode::BAD_REQUEST)
			.is_matched()
	);
}
