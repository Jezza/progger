pub mod parser;
pub mod mem;
pub mod span;

pub trait Span {
	fn new_raw(start: usize, end: usize) -> Self;

	fn range(&self) -> std::ops::Range<usize>;
}



