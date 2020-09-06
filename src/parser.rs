#[macro_export]
macro_rules! new_parser {
(
	Context = $ctx:path;
	TokenKind = $token_kind:path;
	Span = $span:path;
) => {
		pub type Token = ::progger::parser::Token<$token_kind, $span>;

		type Inner = ::progger::parser::Parser<$ctx, $token_kind, $span>;

		pub fn new(ctx: $ctx, input: &str) -> Parser<> {
			let inner = ::progger::parser::Parser::<$ctx, $token_kind, $span>::new(ctx, input);
			Parser(inner)
		}

		pub struct Parser(Inner);

		impl Parser {
			pub fn into_ctx(self) -> $ctx {
				self.0.into_ctx()
			}
		}

		impl ::std::ops::Deref for Parser {
			type Target = Inner;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl ::std::ops::DerefMut for Parser {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}

	};
}

pub type Token<T, S> = (T, S);

pub struct Parser<Ctx, TokenKind, Span> {
	pub ctx: Ctx,
	tokens: Vec<Token<TokenKind, Span>>,
	index: usize,
}

impl<Ctx, Tok, Span: crate::Span> Parser<Ctx, Tok, Span> {
	pub fn new<'source, TokenKind>(ctx: Ctx, input: &'source str) -> Parser<Ctx, TokenKind, Span>
		where TokenKind: logos::Logos<'source, Source = str>
	{
		let tokens: Vec<Token<TokenKind, Span>> = TokenKind::lexer(input)
			.spanned()
			.map(|(kind, span)| {
				let logos::Span {
					start,
					end,
				} = span;

				(kind, Span::new_raw(start, end))
			})
			.collect();

		// println!("{:#?}", tokens);

		Parser {
			ctx,
			tokens,
			index: 0,
		}
	}
}

impl<Ctx, TokenKind, Span> Parser<Ctx, TokenKind, Span> {
	pub fn into_ctx(self) -> Ctx {
		let Parser {
			ctx,
			..
		} = self;

		ctx
	}
}

impl<Ctx, TokenKind: Copy + Clone + PartialEq, Span: Copy + Clone> Iterator for Parser<Ctx, TokenKind, Span> {
	type Item = Token<TokenKind, Span>;

	fn next(&mut self) -> Option<Self::Item> {
		let index = self.index;
		let token = self.tokens.get(index).cloned();
		self.index = index + 1;
		token
	}
}

impl<Ctx, TokenKind: Copy + Clone + PartialEq, Span: Copy + Clone> Parser<Ctx, TokenKind, Span> {
	pub fn peek_token(&mut self) -> Option<Token<TokenKind, Span>> {
		self.tokens.get(self.index)
			.cloned()
	}

	pub fn peek_kind(&mut self) -> Option<TokenKind> {
		self.tokens.get(self.index)
			.map(|(kind, _)| *kind)
	}

	pub fn is(&mut self, token: TokenKind) -> bool {
		self.peek_kind()
			.map(|kind| kind == token)
			.unwrap_or(false)
	}

	pub fn not(&mut self, token: TokenKind) -> bool {
		self.peek_kind()
			.map(|kind| kind != token)
			.unwrap_or(false)
	}

	pub fn at(&mut self, token: TokenKind) -> bool {
		if self.is(token) {
			self.index += 1;
			true
		} else {
			false
		}
	}

	pub fn take_token(&mut self) -> Option<Token<TokenKind, Span>> {
		self.next()
	}

	pub fn take_kind(&mut self) -> Option<TokenKind> {
		self.take_token().map(|(kind, _)| kind)
	}

	pub fn expect(&mut self, kind: TokenKind) -> Result<Token<TokenKind, Span>, Option<Token<TokenKind, Span>>> {
		let token = self.peek_token()
			.ok_or(None)?;
		if token.0 == kind {
			self.index += 1;
			Ok(token)
		} else {
			Err(Some(token))
		}
	}
}
