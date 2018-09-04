use parse::{Parsable, Stream, Token, Precedence};
use env::{Environment, Peeker};
use obj::{AnyShared, types::IntoObject};
use std::fmt::{self, Display, Formatter};

macro_rules! define_opers {
	($env:ident $iter:ident; $($name:ident($sym:tt $prec:ident $block:block);)*) => {
		$(
			#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
			pub struct $name; // foo;

			impl Display for $name {
				fn fmt(&self, f: &mut Formatter) -> fmt::Result {
					Display::fmt($sym, f)
				}
			}

			impl Parsable for $name {
				fn parse(stream: &mut Stream) -> Option<Token> {
					if !stream.as_str().starts_with($sym) {
						return None;
					}
					Some(Token::new(
						stream.offset_by($sym),
						Precedence::$prec,
						|$env: &Environment, $iter: &mut Peeker| {
							macro_rules! binary_oper {
								($rassoc:expr) => {{
									let lhs = $env.pop().expect("no lhs");

									let mut items = Vec::new();
									let cmp = |token: &Token| if $rassoc {
										token.prec() <= Precedence::$prec
									} else {
										token.prec() < Precedence::$prec
									};

									while $iter.peek().map(cmp).unwrap_or(false) {
										items.push($iter.next().unwrap())
									}

									$env.execute(items.into_iter())?;
									let rhs = $env.pop().expect(concat!("no rhs for binary oper `", $sym, "found"));
									$env.push(lhs.read_call(&($sym.into_object() as _), &[rhs], $env)?);
									Ok(())
								}}
							}

							$block
						}
					))
				}
			}

		)*
	}
}

// 1 * 2 ^ 3 + 4;

define_opers! {
	env iter;
	// math opers
	Add("+" AddSub { binary_oper!(false) });
	Mul("*" MulDivMod { binary_oper!(false) });

	Endline(";" Endline { env.pop(); Ok(()) });
	Comma("," Endline { Ok(()) });
	Accessor("." Accessor { binary_oper!(false) });
	Assign("=" Assign { binary_oper!(true) });
}



