use parse::{Parsable, Stream, Token, Precedence};
use env::{Environment, Peeker};
use obj::{AnyShared, types::IntoObject};
use std::fmt::{self, Display, Formatter};

macro_rules! define_opers {
	($stream:ident $env:ident $iter:ident; $($name:ident($sym:tt $prec:ident $check_block:block $block:block);)*) => {
		$(
			#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
			pub struct $name; // foo;

			impl Display for $name {
				fn fmt(&self, f: &mut Formatter) -> fmt::Result {
					Display::fmt($sym, f)
				}
			}

			impl Parsable for $name {
				fn parse($stream: &mut Stream) -> Option<Token> {
					if !$stream.as_str().starts_with($sym) {
						return None;
					}
					$check_block;
					Some(Token::new(
						$stream.offset_by($sym),
						Precedence::$prec,
						|$env: &Environment, $iter: &mut Peeker| {
							macro_rules! binary_oper {
								() => (binary_oper!(false));
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
	stream env iter;
	// math opers
	Add("+" AddSub {} { binary_oper!() }); AddAug("+=" AssignAug {} { binary_oper!() });
	Sub("-" AddSub {} { binary_oper!() }); SubAug("-=" AssignAug {} { binary_oper!() });
	Mul("*" MulDivMod {} { binary_oper!() }); MulAug("*=" AssignAug {} { binary_oper!() });
	Div("/" MulDivMod {} { binary_oper!() }); DivAug("/=" AssignAug {} { binary_oper!() });
	Mod("%" MulDivMod {} { binary_oper!() }); ModAug("%=" AssignAug {} { binary_oper!() });
	Pow("^" MulDivMod {} { binary_oper!(true) }); PowAug("^=" AssignAug {} { binary_oper!() });

	Lt("<" Ordering {} { binary_oper!() }); Gt(">" Ordering {} { binary_oper!() });
	Le("<=" Ordering {} { binary_oper!() }); Ge(">=" Ordering {} { binary_oper!() });
	Eq("==" Ordering {} { binary_oper!() }); Ne("!=" Ordering {} { binary_oper!() });
	Cmp("<=>" Ordering {} { binary_oper!() });

	And("and" And {
		stream.as_str().len() == 3 || !stream.chars().nth(2).unwrap().is_alphanumeric()
	} { binary_oper!() });
	Or("or" Or {
		stream.as_str().len() == 2 || !stream.chars().nth(1).unwrap().is_alphanumeric()
	} { binary_oper!() });

	Endline(";" Endline {} { env.pop(); Ok(()) });
	Comma("," Endline {} { Ok(()) });
	Accessor("." Accessor {} { binary_oper!() });
	Assign("=" Assign {} { binary_oper!(true) });
}



