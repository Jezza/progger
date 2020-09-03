#[macro_export]
macro_rules! new_parser {
(
	ctx = $ctx_lifetime:lifetime;
	Context = $ctx:path;
	TokenKind = $token_kind:path;
	Span = $span:path;
) => {
		pub type Token = ::progger::parser::Token<$token_kind, $span>;

		type Inner<$ctx_lifetime> = ::progger::parser::Parser<$ctx_lifetime, $ctx, $token_kind, $span>;

		pub fn new<$ctx_lifetime>(ctx: &$ctx_lifetime mut $ctx, name: &str, input: &str) -> Parser<$ctx_lifetime> {
			let inner = ::progger::parser::Parser::<$ctx_lifetime, $ctx, $token_kind, $span>::new(ctx, name, input);
			Parser(inner)
		}

		pub struct Parser<$ctx_lifetime>(Inner<$ctx_lifetime>);

		impl<$ctx_lifetime> ::std::ops::Deref for Parser<$ctx_lifetime> {
			type Target = Inner<$ctx_lifetime>;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl<$ctx_lifetime> ::std::ops::DerefMut for Parser<$ctx_lifetime> {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}

	};
}

pub type Token<T, S> = (T, S);

pub struct Parser<'ctx, Ctx, TokenKind, Span> {
	pub ctx: &'ctx mut Ctx,
	name: String,
	input: String,
	tokens: Vec<Token<TokenKind, Span>>,
	index: usize,
}

impl<'ctx, Ctx: 'ctx, Tok, Span: crate::Span> Parser<'ctx, Ctx, Tok, Span> {
	pub fn new<'source, TokenKind>(ctx: &'ctx mut Ctx, name: &str, input: &'source str) -> Parser<'ctx, Ctx, TokenKind, Span>
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
			name: name.into(),
			input: input.into(),
			tokens,
			index: 0,
		}
	}
}

impl<'ctx, Ctx: 'ctx, TokenKind, Span> Parser<'ctx, Ctx, TokenKind, Span> {
	fn name(&self) -> &str {
		&self.name
	}

	fn input(&self) -> &str {
		&self.input
	}
}


impl<'ctx, Ctx: 'ctx, TokenKind: Copy + Clone + PartialEq, Span: Copy + Clone> Parser<'ctx, Ctx, TokenKind, Span> {
	fn next(&mut self) -> Option<Token<TokenKind, Span>> {
		let index = self.index;
		let token = self.tokens.get(index).cloned();
		self.index = index + 1;
		token
	}

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
