#![cfg(feature = "reqwest")]

use http::{Response as HttpResponse, Version};
use reqwest::{Body, Method, Request, Response, ResponseBuilderExt, StatusCode, Url};
use tynavi::traits::AsSelector;

#[test]
fn request_routing() {
	let mut req = Request::new(
		Method::POST,
		Url::parse("https://api.example.com:8443/v1/users?active=true").unwrap(),
	);
	req
		.headers_mut()
		.insert("x-token", "secret".parse().unwrap());
	*req.body_mut() = Some(Body::from("payload"));
	*req.version_mut() = Version::HTTP_2;

	assert_eq!(req.as_selector().method().select(), Some(&Method::POST));
	assert_eq!(
		req.as_selector().url().select().map(Url::as_str),
		Some("https://api.example.com:8443/v1/users?active=true")
	);
	assert!(
		req
			.as_selector()
			.headers()
			.contains_key("x-token")
			.is_matched()
	);
	assert!(req.as_selector().body().is_matched());
	assert!(req.as_selector().body().starts_with(b"pay").is_matched());
}

#[test]
fn request_version_filtering() {
	let mut req = Request::new(Method::GET, Url::parse("https://example.com").unwrap());
	*req.version_mut() = Version::HTTP_11;

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
			.cond_version_eq(false, Version::HTTP_2)
			.is_matched()
	);
	assert!(req.as_selector().version_gt(Version::HTTP_10).is_matched());
	assert!(req.as_selector().version_lt(Version::HTTP_2).is_matched());
}

#[test]
fn response_routing_and_status() {
	let url = Url::parse("https://example.com/items/1").unwrap();
	let res = Response::from(
		HttpResponse::builder()
			.status(StatusCode::CREATED)
			.version(Version::HTTP_2)
			.header("content-type", "application/json")
			.url(url.clone())
			.body("hello")
			.unwrap(),
	);

	assert_eq!(res.as_selector().url().select(), Some(&url));
	assert!(
		res
			.as_selector()
			.headers()
			.contains_key("content-type")
			.is_matched()
	);
	assert!(res.as_selector().is_success().is_matched());
	assert!(
		res
			.as_selector()
			.status_eq(StatusCode::CREATED)
			.is_matched()
	);
	assert!(res.as_selector().version_eq(Version::HTTP_2).is_matched());
}

#[test]
fn response_content_length_filtering() {
	let res = Response::from(
		HttpResponse::builder()
			.status(StatusCode::OK)
			.url(Url::parse("https://example.com/len").unwrap())
			.body("hello")
			.unwrap(),
	);
	let empty = Response::from(
		HttpResponse::builder()
			.status(StatusCode::NO_CONTENT)
			.url(Url::parse("https://example.com/empty").unwrap())
			.body("")
			.unwrap(),
	);

	assert!(res.as_selector().content_length_eq(5).is_matched());
	assert!(res.as_selector().content_length_gt(4).is_matched());
	assert!(res.as_selector().content_length_lt(6).is_matched());
	assert!(res.as_selector().content_length_ge(5).is_matched());
	assert!(res.as_selector().content_length_le(5).is_matched());
	assert!(res.as_selector().content_length_not_eq(6).is_matched());
	assert!(
		res
			.as_selector()
			.cond_content_length_eq(false, 999)
			.is_matched()
	);
	assert!(empty.as_selector().content_length_eq(0).is_matched());
}

#[test]
fn url_filters_and_routing() {
	let url = Url::parse("https://example.com:8080/api/v1/users?sort=desc").unwrap();
	let no_port = Url::parse("https://example.com/api/v1/users").unwrap();

	assert!(url.as_selector().starts_with("https://").is_matched());
	assert!(url.as_selector().ends_with("sort=desc").is_matched());
	assert!(url.as_selector().contains("/api/").is_matched());
	assert!(!url.as_selector().contains("/admin/").is_matched());
	assert_eq!(url.as_selector().path().select(), Some("/api/v1/users"));
	assert_eq!(url.as_selector().query().select(), Some("sort=desc"));
	assert_eq!(url.as_selector().host().select(), Some("example.com"));
	assert_eq!(url.as_selector().scheme().select(), Some("https"));
	assert!(url.as_selector().port_eq(8080).is_matched());
	assert!(url.as_selector().port_gt(8000).is_matched());
	assert!(url.as_selector().port_lt(9000).is_matched());
	assert!(!no_port.as_selector().port_eq(8080).is_matched());
	assert!(no_port.as_selector().port_not_eq(8080).is_matched());
	assert!(no_port.as_selector().cond_port_eq(false, 8080).is_matched());
}

#[test]
fn body_filters() {
	let body = Body::from("hello world");

	assert!(!body.as_selector().empty().is_matched());
	assert!(body.as_selector().not_empty().is_matched());
	assert!(body.as_selector().starts_with(b"hello").is_matched());
	assert!(body.as_selector().ends_with(b"world").is_matched());
	assert!(body.as_selector().contains(b"lo wo").is_matched());
	assert!(!body.as_selector().contains(b"abc").is_matched());
	assert!(body.as_selector().not_contains(b"abc").is_matched());
	assert!(
		body
			.as_selector()
			.cond_starts_with(false, b"abc")
			.is_matched()
	);
}

#[test]
fn backtrack_and_up_propagate() {
	let mut req = Request::new(
		Method::GET,
		Url::parse("https://example.com/users").unwrap(),
	);
	req.headers_mut().insert("x-id", "42".parse().unwrap());

	let url_selector = req.as_selector().url();
	let failed = url_selector.host().starts_with("missing.");

	assert!(url_selector.backtrack().select().is_some());
	assert_eq!(
		url_selector
			.backtrack()
			.select()
			.map(|request| request.url().as_str()),
		Some("https://example.com/users")
	);
	assert!(!failed.is_matched());
	assert!(!failed.up().is_matched());
	assert_eq!(url_selector.host().backtrack().select(), Some(req.url()));
}
