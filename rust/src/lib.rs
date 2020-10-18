/*!
Overlay API
===========

Examples
--------

```
use vision_overlay::*;
let pool = StringPool::new();
let mut draw = Vec::new();
draw.push(Draw::Text {
	x: 0.0,
	y: 0.0,
	string: pool.store(format!("Hello, {}!", "World")),
	font: "18px serif",
	max: None,
	fill: Some("white"),
	stroke: Some("black"),
});

let cmd = Command::Draw(&draw);
let s = serde_json::to_string(&cmd).unwrap();

// Send `s` over websocket to the overlay
```
*/

mod pool;
mod draw;

pub use self::pool::*;
pub use self::draw::*;

#[derive(Clone, Debug)]
pub struct Sprites<'a> {
	pub url: &'a str,
	pub sprites: &'a [Sprite<'a>],
}
impl<'a> serde::Serialize for Sprites<'a> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::*;
		let mut state = serializer.serialize_struct("Sprites", 2)?;
		state.serialize_field("url", self.url)?;
		state.serialize_field("sprites", self.sprites)?;
		state.end()
	}
}

#[derive(Clone, Debug)]
pub struct Sprite<'a> {
	pub id: &'a str,
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32,
}
impl<'a> serde::Serialize for Sprite<'a> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::*;
		let mut state = serializer.serialize_struct("Sprite", 5)?;
		state.serialize_field("id", &self.id)?;
		state.serialize_field("x", &self.x)?;
		state.serialize_field("y", &self.y)?;
		state.serialize_field("w", &self.w)?;
		state.serialize_field("h", &self.h)?;
		state.end()
	}
}

#[derive(Clone, Debug)]
pub enum Command<'a> {
	/// Debug string.
	///
	/// The contents are displayed as HTML on top of the overlay.
	/// Its purpose is to visualize state for debugging.
	Debug(&'a str),
	/// Status string.
	///
	/// Put some text at the bottom of the overlay.
	Status(&'a str),
	/// Draw instructions.
	///
	/// Paint the canvas with the given draw instructions.
	/// The canvas is cleared every time a new set of draw instructions are executed.
	Draw(&'a [Draw<'a>]),
	/// Display a list of names.
	Names(&'a [&'a str]),
	/// Preload sprites.
	Sprites(Sprites<'a>),
	/// Reload the overlay.
	Reload,
}
impl<'a> serde::Serialize for Command<'a> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::*;
		match self {
			Command::Debug(debug) => {
				let mut state = serializer.serialize_struct("Debug", 2)?;
				state.serialize_field("target", "debug")?;
				state.serialize_field("message", debug)?;
				state.end()
			},
			Command::Status(status) => {
				let mut state = serializer.serialize_struct("Status", 2)?;
				state.serialize_field("target", "status")?;
				state.serialize_field("message", status)?;
				state.end()
			},
			Command::Draw(draw) => {
				let mut state = serializer.serialize_struct("Draw", 2)?;
				state.serialize_field("target", "draw")?;
				state.serialize_field("message", draw)?;
				state.end()
			},
			Command::Names(names) => {
				let mut state = serializer.serialize_struct("Names", 2)?;
				state.serialize_field("target", "names")?;
				state.serialize_field("message", names)?;
				state.end()
			},
			Command::Sprites(sprites) => {
				let mut state = serializer.serialize_struct("Sprites", 2)?;
				state.serialize_field("target", "sprites")?;
				state.serialize_field("message", sprites)?;
				state.end()
			},
			Command::Reload => {
				let mut state = serializer.serialize_struct("Reload", 2)?;
				state.serialize_field("target", "reload")?;
				state.serialize_field("message", "")?;
				state.end()
			},
		}
	}
}
