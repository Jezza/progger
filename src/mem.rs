pub use arena::Arena;
pub use arena::ArenaId;
pub use interner::Interner;
pub use interner::InternId;

mod arena {
	use std::fmt::Formatter;
	use std::marker::PhantomData;

	use bumpalo::Bump;
	use bumpalo::collections::Vec as BumpVec;

	#[derive(Ord, PartialOrd, Eq, PartialEq)]
	pub struct ArenaId<T>(u16, PhantomData<*const T>);

	impl<T> Clone for ArenaId<T> {
		fn clone(&self) -> Self {
			*self
		}
	}

	impl<T> Copy for ArenaId<T> {}

	impl<T> std::fmt::Debug for ArenaId<T> {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f, "ArenaId({})", self.0)
		}
	}

	pub struct Arena<'arena, T: 'arena> {
		_bump: Box<Bump>,
		arena: BumpVec<'arena, T>,
	}

	impl<'a, T: 'a> Default for Arena<'a, T> {
		fn default() -> Self {
			Self::new()
		}
	}

	impl<'a, T: 'a> Arena<'a, T> {
		pub fn new() -> Arena<'a, T> {
			let bump = Bump::new();
			let bump = Box::new(bump);

			let arena_ref: &'a Bump = unsafe {
				std::mem::transmute(bump.as_ref())
			};

			let arena = BumpVec::new_in(arena_ref);

			Arena {
				_bump: bump,
				arena,
			}
		}

		pub fn alloc(&mut self, data: T) -> ArenaId<T> {
			let id = self.arena.len();
			self.arena.push(data);
			ArenaId(id as u16, PhantomData)
		}

		pub fn get(&self, id: ArenaId<T>) -> &T {
			&self.arena[id.0 as usize]
		}

		pub fn get_mut(&mut self, id: ArenaId<T>) -> &mut T {
			&mut self.arena[id.0 as usize]
		}
	}
}

mod interner {
	use crate::Span;

	#[derive(Debug)]
	#[derive(Ord, PartialOrd, Eq, PartialEq)]
	pub struct InternId(u16);

	impl Clone for InternId {
		fn clone(&self) -> Self {
			*self
		}
	}

	impl Copy for InternId {}

	pub struct Interner<T> {
		spans: Vec<T>,
		arena: String,
	}

	impl<T> Default for Interner<T> {
		fn default() -> Self {
			Self::new()
		}
	}

	impl<T> Interner<T> {
		pub fn new() -> Interner<T> {
			let spans = vec![];
			let arena = String::new();

			Interner {
				spans,
				arena,
			}
		}
	}

	impl<T: Span> Interner<T> {
		pub fn get(&self, id: InternId) -> &str {
			let span = self.spans.get(id.0 as usize).unwrap();
			&self.arena[span.range()]
		}

		pub fn lookup(&self, value: &str) -> Option<InternId> {
			for (i, span) in self.spans.iter().enumerate() {
				if &self.arena[span.range()] == value {
					return Some(InternId(i as u16));
				}
			}
			None
		}

		pub fn insert(&mut self, value: &str) -> InternId {
			if let Some(value) = self.lookup(value) {
				return value;
			}

			let start = self.arena.len();
			self.arena.push_str(value);
			let end = self.arena.len();

			let index = self.spans.len();
			self.spans.push(Span::new_raw(start, end));
			InternId(index as u16)
		}
	}
}
