use std::fmt::{Debug, Formatter, Result as FResult};
use std::ops::Range;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
	start: usize,
	end: usize,
}

impl Debug for Span {
	fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
		f.write_str(&format!("[{}, {})", self.start, self.end))
	}
}

impl Span {
	pub fn new_raw(start: usize, end: usize) -> Self {
		Span {
			start,
			end,
		}
	}

	pub fn range(&self) -> Range<usize> {
		self.start..self.end
	}

	pub fn merge(&self, other: Span) -> Self {
		let start = self.start.min(other.start);
		let end = self.end.max(other.end);
		Span {
			start,
			end,
		}
	}
}

impl crate::Span for Span {
	fn new_raw(start: usize, end: usize) -> Self {
		Span::new_raw(start, end)
	}

	fn range(&self) -> Range<usize> {
		self.range()
	}
}
