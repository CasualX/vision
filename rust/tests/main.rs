use vision_overlay::*;

#[test]
fn main() {
	let container = StringPool::new();
	let a = container.store(String::from("rgba(0,0,255,0.5)"));
	let b = container.store(String::from("48px serif"));
	let draw = &[
		Draw::Rect { x: 10.0, y: 10.0, w: 100.0, h: 100.0, fill: Some(a), inset: None, stroke: None },
		Draw::Text { x: 10.0, y: 200.0, string: "Hello World", font: b, max: None, stroke: Some("black"), fill: Some("red") },
	];
	let cmd = Command::Draw(draw);
	let s = serde_json::to_string(&cmd).unwrap();
	println!("{}", s);
}
