use std::cell::Cell;
use std::mem::forget;

/// Dynamically constructed strings in the API.
#[derive(Default)]
pub struct StringPool {
	strings: Cell<Vec<String>>,
}
impl StringPool {
	pub const fn new() -> StringPool {
		StringPool { strings: Cell::new(Vec::new()) }
	}
	pub fn store(&self, s: String) -> &str {
		let mut strings = self.strings.replace(Vec::new());
		let s_ptr = s.as_str() as *const str;
		strings.push(s);
		forget(self.strings.replace(strings));
		unsafe { &*s_ptr }
	}
}
