#[derive(Clone, Debug)]
pub enum Draw<'a> {
	/// Draws a rectangle.
	Rect {
		/// X coordinate.
		x: f32,
		/// Y coordinate.
		y: f32,
		/// Width.
		w: f32,
		/// Height.
		h: f32,
		/// Optional fill color. If absent the rectangle will not be filled.
		fill: Option<&'a str>,
		/// Optional inset color. Draws a second rectangle at an offset.
		inset: Option<&'a str>,
		/// Optional stroke color. If absent the rectangle will not be stroked.
		stroke: Option<&'a str>,
	},
	/// Draws a path.
	Path {
		/// SVG Path syntax.
		path: &'a str,
		/// Optional fill color. If absent the rectangle will not be filled.
		fill: Option<&'a str>,
		/// Optional stroke color. If absent the rectangle will not be stroked.
		stroke: Option<&'a str>,
	},
	/// Draws text.
	Text {
		/// X coordinate.
		x: f32,
		/// Y coordinate.
		y: f32,
		/// The text to draw.
		string: &'a str,
		/// The font description.
		font: &'a str,
		/// Maximum width the text should be squeezed in.
		max: Option<f32>,
		/// Optional fill color. If absent the rectangle will not be filled.
		fill: Option<&'a str>,
		/// Optional stroke color. If absent the rectangle will not be stroked.
		stroke: Option<&'a str>,
	},
	/// Draws a sprite.
	Sprite {
		/// Sprite identifier.
		id: &'a str,
		/// X coordinate.
		x: f32,
		/// Y coordinate.
		y: f32,
		/// Width.
		w: f32,
		/// Height.
		h: f32,
	},
}
impl<'a> serde::Serialize for Draw<'a> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::*;
		match self {
			Draw::Rect { x, y, w, h, fill, inset, stroke } => {
				let mut state = serializer.serialize_struct("Rect", 1)?;
				state.serialize_field("ty", "rect")?;
				state.serialize_field("x", x)?;
				state.serialize_field("y", y)?;
				state.serialize_field("w", w)?;
				state.serialize_field("h", h)?;
				if let Some(fill) = fill {
					state.serialize_field("fill", fill)?;
				}
				if let Some(inset) = inset {
					state.serialize_field("inset", inset)?;
				}
				if let Some(stroke) = stroke {
					state.serialize_field("stroke", stroke)?;
				}
				state.end()
			},
			Draw::Path { path, fill, stroke } => {
				let mut state = serializer.serialize_struct("Path", 1)?;
				state.serialize_field("ty", "path")?;
				state.serialize_field("path", path)?;
				if let Some(fill) = fill {
					state.serialize_field("fill", fill)?;
				}
				if let Some(stroke) = stroke {
					state.serialize_field("stroke", stroke)?;
				}
				state.end()
			},
			Draw::Text { string, font, x, y, max, fill, stroke } => {
				let mut state = serializer.serialize_struct("Text", 1)?;
				state.serialize_field("ty", "text")?;
				state.serialize_field("x", x)?;
				state.serialize_field("y", y)?;
				state.serialize_field("string", string)?;
				state.serialize_field("font", font)?;
				if let Some(max) = max {
					state.serialize_field("max", max)?;
				}
				if let Some(fill) = fill {
					state.serialize_field("fill", fill)?;
				}
				if let Some(stroke) = stroke {
					state.serialize_field("stroke", stroke)?;
				}
				state.end()
			},
			Draw::Sprite { id, x, y, w, h } => {
				let mut state = serializer.serialize_struct("Sprite", 6)?;
				state.serialize_field("ty", "sprite")?;
				state.serialize_field("id", id)?;
				state.serialize_field("x", x)?;
				state.serialize_field("y", y)?;
				state.serialize_field("w", w)?;
				state.serialize_field("h", h)?;
				state.end()
			},
		}
	}
}
