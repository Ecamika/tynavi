#![cfg(feature = "tungstenite")]

use tungstenite::Bytes;
use tungstenite::protocol::Message;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::frame::{CloseFrame, Utf8Bytes};
use tynavi::traits::AsSelector;

#[test]
fn message_type_checks() {
	assert!(
		Message::Text("hello".into())
			.as_selector()
			.is_text()
			.is_matched()
	);
	assert!(
		Message::Binary(vec![1, 2, 3].into())
			.as_selector()
			.is_binary()
			.is_matched()
	);
	assert!(
		Message::Ping(vec![].into())
			.as_selector()
			.is_ping()
			.is_matched()
	);
	assert!(
		Message::Pong(vec![].into())
			.as_selector()
			.is_pong()
			.is_matched()
	);
	assert!(Message::Close(None).as_selector().is_close().is_matched());
	assert!(
		!Message::Text("hello".into())
			.as_selector()
			.is_binary()
			.is_matched()
	);
}

#[test]
fn message_not_type_checks() {
	assert!(
		Message::Text("hello".into())
			.as_selector()
			.not_is_binary()
			.is_matched()
	);
	assert!(
		!Message::Text("hello".into())
			.as_selector()
			.not_is_text()
			.is_matched()
	);
}

#[test]
fn message_cond_type_checks() {
	assert!(
		Message::Text("hello".into())
			.as_selector()
			.cond_is_text(true)
			.is_matched()
	);
	assert!(
		Message::Text("hello".into())
			.as_selector()
			.cond_is_text(false)
			.is_matched()
	);
	assert!(
		!Message::Text("hello".into())
			.as_selector()
			.cond_is_binary(true)
			.is_matched()
	);
	assert!(
		!Message::Text("hello".into())
			.as_selector()
			.cond_not_is_text(true)
			.is_matched()
	);
}

#[test]
fn message_empty() {
	assert!(Message::Text("".into()).as_selector().empty().is_matched());
	assert!(
		!Message::Text("hello".into())
			.as_selector()
			.empty()
			.is_matched()
	);
	assert!(
		Message::Text("hello".into())
			.as_selector()
			.not_empty()
			.is_matched()
	);
	assert!(
		!Message::Text("".into())
			.as_selector()
			.not_empty()
			.is_matched()
	);
}

#[test]
fn message_routing() {
	let text_msg = Message::Text("hello".into());
	assert_eq!(
		text_msg
			.as_selector()
			.as_text()
			.select()
			.map(|v| v.as_str()),
		Some("hello")
	);
	assert!(!text_msg.as_selector().as_binary().is_matched());

	let bin_msg = Message::Binary(vec![1, 2, 3].into());
	assert_eq!(
		bin_msg.as_selector().as_binary().select().map(|v| v.len()),
		Some(3)
	);

	let close_msg = Message::Close(Some(CloseFrame {
		code: CloseCode::Normal,
		reason: "bye".into(),
	}));
	assert!(close_msg.as_selector().as_close().is_matched());

	// close() returns Selector<Option<CloseFrame>, Self>, flatten it
	let close_frame = close_msg.as_selector().as_close().flatten().select();
	assert!(close_frame.is_some());
	assert_eq!(close_frame.unwrap().code, CloseCode::Normal);
}

#[test]
fn utf8bytes_string_methods() {
	let utf8 = Utf8Bytes::from_static("hello world");
	assert!(utf8.as_selector().contains("world").is_matched());
	assert!(!utf8.as_selector().contains("foo").is_matched());
	assert!(utf8.as_selector().starts_with("hello").is_matched());
	assert!(utf8.as_selector().ends_with("world").is_matched());
	assert!(!utf8.as_selector().empty().is_matched());
	assert!(utf8.as_selector().not_empty().is_matched());
	assert!(
		Utf8Bytes::from_static("")
			.as_selector()
			.empty()
			.is_matched()
	);
}

#[test]
fn closeframe_routing() {
	let frame = CloseFrame {
		code: CloseCode::Away,
		reason: Utf8Bytes::from_static("going away"),
	};
	assert_eq!(frame.as_selector().code().select(), Some(&CloseCode::Away));
	assert_eq!(
		frame.as_selector().reason().select(),
		Some(&Utf8Bytes::from_static("going away"))
	);
	assert!(frame.as_selector().reason().contains("going").is_matched());
}

#[test]
fn closecode_allowed() {
	assert!(CloseCode::Normal.as_selector().is_allowed().is_matched());
	assert!(!CloseCode::Abnormal.as_selector().is_allowed().is_matched());
	assert!(
		CloseCode::Abnormal
			.as_selector()
			.not_is_allowed()
			.is_matched()
	);
	assert!(
		CloseCode::Normal
			.as_selector()
			.cond_is_allowed(true)
			.is_matched()
	);
	assert!(
		CloseCode::Normal
			.as_selector()
			.cond_is_allowed(false)
			.is_matched()
	);
}

#[test]
fn bytes_empty() {
	assert!(Bytes::from_static(b"").as_selector().empty().is_matched());
	assert!(
		!Bytes::from_static(b"hello")
			.as_selector()
			.empty()
			.is_matched()
	);
	assert!(
		Bytes::from_static(b"hello")
			.as_selector()
			.not_empty()
			.is_matched()
	);
}

#[test]
fn message_chaining() {
	let msg = Message::Close(Some(CloseFrame {
		code: CloseCode::Normal,
		reason: Utf8Bytes::from_static("done"),
	}));
	assert!(
		msg
			.as_selector()
			.is_close()
			.as_close()
			.flatten()
			.code()
			.eq(&CloseCode::Normal)
			.is_matched()
	);

	let msg2 = Message::Text("hello world".into());
	assert!(
		msg2
			.as_selector()
			.is_text()
			.as_text()
			.contains("world")
			.is_matched()
	);
}
