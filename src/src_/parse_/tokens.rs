use std::cmp::Ordering;
use parse::{Tree, TokenMatch, funcs, MatchData, Stream};
use std::hash::{Hash, Hasher};
use env_::Environment__;
use obj_::{QObject_, QObject__, Result_, classes_::{self, QVar, QBlock, QText, QNull, QList}};
use regex::Regex;
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Binding {
	Literal = 0, Accessor, BlockStart, //# itself is ofr sigils
	Unary, Pow, UnaryNeg,
	MulDivMod, AddSub,
	BwShift, BwAnd, BwOrXor,
	Ordering, Equality,
	LogicalAnd, LogicalOr,

	TernaryElse,
	TernaryIf,
	Assignment, AssignmentMath, AssignmentBwShift, AssignmentBw,
	Comma,
	EndOfLine
}

impl Default for Binding {
	fn default() -> Binding { Binding::Literal }
}

#[derive(Clone)]
pub struct Token {
	name: &'static str,
	binding: Binding,
	r_assoc: bool,
	pub(super) match_fn: fn(&Stream, &Environment__) -> Option<MatchData>,
	qobj: Option<fn(&Tree, &Environment__) -> Result_>,
	to_qvar: Option<fn(&Tree, &Environment__) -> QObject_>,
}


impl PartialOrd for Token {
	fn partial_cmp(&self, rhs: &Token) -> Option<Ordering> {
		self.binding.partial_cmp(&rhs.binding)
	}
}

impl Ord for Token {
	fn cmp(&self, rhs: &Token) -> Ordering {
		self.binding.cmp(&rhs.binding)
	}
}


impl Hash for Token {
	fn hash<H: Hasher>(&self, h: &mut H) {
		(self as *const Token).hash(h)
	}
}

impl Default for Token {
	fn default() -> Token {
		Token {
			name: "",
			r_assoc: false, // unused
			binding: Binding::default(),
			match_fn: |_, _| None,
			qobj: None,
			to_qvar: None
		}
	}
}

impl Eq for Token {}
impl PartialEq for Token {
	fn eq(&self, other: &Token) -> bool {
		self as *const Token == other as *const Token
	}
}

impl Token {
	pub fn create_qobject(&self, tree: &Tree, env: &Environment__) -> Result_ {
		debug_assert!(self.qobj.is_some(), "Attempted to create a QObject_ out of a non-qobj token `{:?}`", self);
		self.qobj.expect("bad qobj token")(tree, env)
	}

	pub fn to_qvar(&self, tree: &Tree, env: &Environment__) -> Result_ {
		if let Some(to_qvar) = self.to_qvar {
			return Ok(to_qvar(tree, env));
		}

		assert!(tree.is_singular(), "Can only get qvar of singular trees for now (got: {:#?})", tree);

		if tree.oper.token == V_NORM.deref() {
			let text = tree.oper.try_as_str().expect(concat!("Non-Text match data found for QVar"));
			Ok(QObject_::Old(QVar::from_str(text).unwrap().into()))
		} else {
			tree.execute(env)
		}
	}
}

impl Debug for Token {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Token({:?})", self.name)
	}
}

macro_rules! tokens {
	($(token $token:ident $body:tt;)*) => {
		impl Token {
			pub fn default_tokens() -> &'static [&'static Token] { DEFAULT_TOKENS.deref() }
		}

		lazy_static!  {

			static ref DEFAULT_TOKENS: [&'static Token; argcount!($($token)*)] = [$($token.deref()),*];

			$(
				pub static ref $token: Token = create_token!($token @$body);
			)*
		}
	}
}

macro_rules! normal_match_fn {
	($regex:expr) => (|stream, _| regex_match!($regex, stream).map(|x| {let l = x.len(); MatchData::Text(x, l)}))
}

macro_rules! lazy_regex {
	($regex:expr) => ({
		lazy_static! { static ref REGEX: Regex = regex!($regex); }
		Deref::deref(&REGEX)
	})
}

macro_rules! regex_match {
	($regex:expr, $val:expr) => {
		if cfg!(debug_assertions) {
			if let Some(m) = $regex.find($val) {
				debug_assert_eq!(m.start(), 0, "Regex `{}` didn't match at 0 ({})", $regex.as_str(), m.as_str());
				Some(m.as_str().to_string())
			} else {
				None
			}
		} else {
			$regex.find($val).map(|x| x.as_str().to_string())
		}
	}
}

macro_rules! create_token {
	($name:ident @{$($body:tt)*}) => { Token { name: stringify!($name), $($body)* } };
	($name:ident @($($inner:tt)*)) => { create_token!($name $($inner)*) };
	($name:ident forced_eof $regex:tt) => {
		Token {
			name: stringify!($name),
			match_fn: |stream, _| regex_match!(lazy_regex!(concat!("\\A", $regex)), stream).map(|m| MatchData::Eof(m.len())),
			..Token::default()
		}
	};
	($name:ident no_token $regex:tt) => {
		Token {
			name: stringify!($name),
			match_fn: |stream, _| regex_match!(lazy_regex!(concat!("\\A", $regex)), stream).map(|m| MatchData::NoToken(m.len())),
			..Token::default()
		}
	};

	($name:ident block_start $paren:tt $opposite:tt $qobj:expr) => {
		Token {
			name: stringify!($name),
			binding: Binding::BlockStart,
			qobj: Some($qobj),
			match_fn: |stream, env| {
				if !stream.starts_with($paren) {
					return None
				}
				let start_len = stream.len();
				let mut stream = stream.clone();
				stream.offset_by(1);
				let matches = funcs::matches_until(&mut stream, env, |x| 
					x.try_as_str()
					 .and_then(|x| x.chars().next())
					 .map(|x| x == $opposite)
					 .unwrap_or(false));

				Some(MatchData::Block(matches, start_len - stream.len()))
			},
			..Token::default()
		}
	};
	($name:ident marker_token $marker:tt $binding:ident) => {
		Token {
			name: stringify!($name),
			binding: Binding::$binding,
			match_fn: |stream, _|
				if stream.starts_with($marker) {
					let s = $marker.to_string();
					let l = s.len();
					Some(MatchData::Text(s, l))
				} else {
					None
				},
			..Token::default()
		}
	};
	($name:ident text $quote:tt $object:ty, $func:tt) => {
		Token {
			name: stringify!($name),
			qobj: Some(|tree, _| {
				assert!(tree.is_singular(), concat!("Non-singular tree found for `", stringify!($object), "`: {:?}"), tree);
				assert_eq!(tree.oper.token, $name.deref());
				let text = tree.oper.try_as_str().expect(concat!("Non-Text match data found for ", stringify!($object)));

				Ok(QObject_::Old(<$object>::$func(text).into()))
			}),
			match_fn: |stream, _| get_text($quote, stream).map(|(s, len)| MatchData::Text(s, len)),
			..Token::default()
		}
	};
	($name:ident object $object:ty, $regex:path) => {{
		use obj_::classes_::*;
		Token {
			name: stringify!($name),
			match_fn: normal_match_fn!($regex),
			qobj: Some(|tree, _| Ok(object_from_tree::<$object, _>(tree))),
			..Token::default()
		}
	}};
	($name:ident object $object:ty, $regex:path, NEW) => {{
		use obj_::classes_::*;
		Token {
			name: stringify!($name),
			match_fn: normal_match_fn!($regex),
			qobj: Some(|tree, _| Ok(object_from_tree_new::<$object, _>(tree))),
			..Token::default()
		}
	}};
	($name:ident oper binary $oper:ty, $regex:path, $binding:expr) => {{
		use obj_::classes_::opers::*;
		use self::Binding::*;
		Token {
			name: stringify!($name),
			binding: $binding,
			match_fn: normal_match_fn!($regex),
			qobj: Some(|tree, env| QObject__::from(<$oper>::from_tree(tree)).call(&[], env) ),
			..Token::default()
		}
	}};
}

fn object_from_tree<T: FromStr<Err=E> + Into<QObject__>, E: Debug>(tree: &Tree) -> QObject_ {
	assert!(tree.is_singular(), "Non-singular tree found {:?}", tree);
	let text = tree.oper.try_as_str().expect("Non-Text match data found");

	match T::from_str(&text) {
		Ok(obj) => obj.into().old(),
		Err(err) => panic!("Bad sigil `{:?}` supplied : {:?}", text, err)
	}
}
fn object_from_tree_new<T: ::obj::classes::QuestClass + FromStr<Err=E>, E: Debug>(tree: &Tree) -> QObject_ {
	assert!(tree.is_singular(), "Non-singular tree found {:?}", tree);
	let text = tree.oper.try_as_str().expect("Non-Text match data found");

	match T::from_str(&text) {
		Ok(obj) => (::shared::Shared::new(::obj::__QObject::new(obj)) as ::obj::SharedObject).into(),
		Err(err) => panic!("Bad sigil `{:?}` supplied : {:?}", text, err)
	}
}

fn get_text(quote: char, stream: &Stream) -> Option<(String, usize)> {
	if !stream.starts_with(quote) {
		return None
	}

	let mut len = 1;
	let mut text = String::new();
	let mut stream = stream.clone();
	stream.offset_by(1);
	let mut chars = stream.chars();

	loop {
		let c = chars.next().expect("Unterminated string found");
		len += 1;
		if c == quote {
			break
		}
		text.push(if c != '\\' { c } else {
			len += 1;
			match chars.next().expect("dangling `\\` found") {
				'\n' => continue,
				'n' => '\n',
				't' => '\t',
				'r' => '\r',
				'x' | 'u' | 'U' => unimplemented!("todo: extended escapes"),
				other => other
			}
		});
	}
	Some((text, len))
}


tokens! {
	token W_EOF {
		match_fn: |stream, _| if stream.is_empty() { Some(MatchData::Eof(0)) } else { None },
		..Token::default()
	};
	token W_FORCED_EOF(forced_eof r"__(?:EOF|END)__\b");
	token W_COMMENT(no_token "(?m)#.*$");
	token W_WHITESPACE(no_token r"(?m)\s+");

	token P_LPAREN(block_start '(' ')' |tree, env| {
		debug_assert!(tree.rhs().is_none(), "got rhs of paren block");
		let block = Tree::try_from_vec(tree.oper.data.try_as_block().expect("MatchData isn't a block for paren?").to_vec());
		if let Some(lhs) = tree.lhs() { // this is a function call, eg `foo(XXX)`
			let lhs = lhs.execute(env)?.unwrap_old();

			if let Some(block) = block { // this is a function call with args, eg `foo(1, 2)`
				let block = block.execute(env)?.unwrap_old();
				if let Some(list) = block.try_cast_list() {
					let list = list.as_ref();
					lhs.call(&list.iter().collect::<Vec<_>>(), env)
				} else {
					lhs.call(&[&block], env)
				}
			} else { // function call with out args, eg `foo()`
				lhs.call(&[], env)
			}
		} else { // this is just a `()` without anything surrounding it, as in `4 * (5 + 3)`
			match block {
				Some(tree) => tree.execute(env),
				None => panic!("Empty paren body encountered") // this is analgous to `x = ()`;
			}
		}
	});
	// token P_LSQUAR(block_start '[' ']' |tree, env|QBlockArray);
	token P_LCURLY(block_start '{' '}' |tree, env| {
		debug_assert!(tree.rhs().is_none(), "got rhs of paren block");
		if let Some(lhs) = tree.lhs() {
			let block = Tree::try_from_vec(tree.oper.data.try_as_block().expect("MatchData isn't a block for paren?").to_vec());
			let lhs = lhs.execute(env)?.unwrap_old();

			if let Some(block) = block { // this is a function call with args, eg `foo(1, 2)`
				let block = block.execute(env)?.unwrap_old();
				if let Some(list) = block.try_cast_list() {
					let list = list.as_ref();
					lhs.call_local(&list.iter().collect::<Vec<_>>(), env)
				} else {
					lhs.call_local(&[&block], env)
				}
			} else { // function call with out args, eg `foo()`
				lhs.call_local(&[], env)
			}
		} else { // this is just a `()` without anything surrounding it, as in `4 * (5 + 3)`
			if let MatchData::Block(ref block, _) = tree.oper.data {
				Ok(QObject_::Old(QBlock::new(Tree::try_from_vec(block.to_owned())).into()))
			} else {
				panic!("Empty curly block body encountered") // this is analgous to `x = ()`;
			}
		}
	});

	token P_RPAREN(marker_token ')' Literal);
	token P_RSQUAR(marker_token ']' Literal);
	token P_RCURLY(marker_token '}' Literal);

	token N_DECI(object QNum, num::RE_DECI);
	token N_HEX(object QNum, num::RE_HEX);
	token N_BINARY(object QNum, num::RE_BINARY);
	token N_OCTAL(object QNum, num::RE_OCTAL);
	token N_NORM(object QNum, num::RE_FLOAT);

	token B_TRUE(object bool, bool::RE_TRUE, NEW);
	token B_FALSE(object QBool, bool::RE_FALSE);
	token B_NULL(object QNull, null::RE_NULL);

	// token V_LITERAL(text "`");

	token S_SINGLE(text '\'' QText, new);
	token S_DOUBLE(text '\"' QText, new);
	token S_GRAVE(text '`' QVar, from_nonstatic_str);


	// token O_NEG(oper unary true QNeg, neg::REGEX, UnaryNeg);
	token O_ADD(oper binary QAdd, add::REGEX, AddSub);
	token O_SUB(oper binary QSub, sub::REGEX, AddSub);
	token O_MUL(oper binary QMul, mul::REGEX, MulDivMod);
	token O_DIV(oper binary QDiv, div::REGEX, MulDivMod);
	token O_POW(oper binary QPow, pow::REGEX, Pow);
	token O_MOD(oper binary QMod, mod_::REGEX, MulDivMod);

	token O_ADDI(oper binary QAddI, addi::REGEX, AssignmentMath);
	token O_SUBI(oper binary QSubI, subi::REGEX, AssignmentMath);
	token O_MULI(oper binary QMulI, muli::REGEX, AssignmentMath);
	token O_DIVI(oper binary QDivI, divi::REGEX, AssignmentMath);
	token O_POWI(oper binary QPowI, powi::REGEX, AssignmentMath);
	token O_MODI(oper binary QModI, modi::REGEX, AssignmentMath);

	// token O_NOT(oper unary true QNot, noti::REGEX, Unary);
	token O_OR (oper binary  QOr,  or::REGEX, LogicalOr);
	token O_AND(oper binary QAnd, and::REGEX, LogicalAnd);

	token O_LT(oper binary QLt, lt::REGEX, Ordering);
	token O_GT(oper binary QGt, gt::REGEX, Ordering);
	token O_LE(oper binary QLe, le::REGEX, Ordering);
	token O_GE(oper binary QGe, ge::REGEX, Ordering);

	token O_CMP(oper binary QCmp, cmp::REGEX, Equality);
	token O_EQ(oper binary QEq, eq::REGEX, Equality);
	token O_NE(oper binary QNe, ne::REGEX, Equality);

	token O_ASSIGN(oper binary QAssign, assign::REGEX, Assignment);
	token O_ASSIGNL(oper binary QAssignL, assignl::REGEX, Assignment);
	token O_ASSIGNR(oper binary QAssignR, assignr::REGEX, Assignment);

	// token O_NOT(oper unary true QBwNeg, bw_neg::REGEX, Unary);
	token O_BWOR(oper binary QBwOr, bw_or::REGEX, BwOrXor);
	token O_BWAND(oper binary QBwAnd, bw_and::REGEX, BwAnd);
	token O_BWXOR(oper binary QBwXor, bw_xor::REGEX, BwOrXor);
	token O_BWLS(oper binary QBwLs, bw_ls::REGEX, BwShift);
	token O_BWRS(oper binary QBwRs, bw_rs::REGEX, BwShift);

	token O_BWORI(oper binary QBwOrI, bw_ori::REGEX, AssignmentBw);
	token O_BWANDI(oper binary QBwAndI, bw_andi::REGEX, AssignmentBw);
	token O_BWXORI(oper binary QBwXorI, bw_xori::REGEX, AssignmentBw);
	token O_BWLSI(oper binary QBwLsI, bw_lsi::REGEX, AssignmentBwShift);
	token O_BWRSI(oper binary QBwRsI, bw_rsi::REGEX, AssignmentBwShift);

	token O_ACCESSOR(oper binary QAccessor, accessor::REGEX, Accessor);
	token COMMA(oper binary QComma, comma::REGEX, Comma);
	token LINE_END(oper binary QLineEnd, line_end::REGEX, EndOfLine);

	token EXISTS(oper binary QQuestion, exists::REGEX, TernaryIf);

	token COLON(marker_token ':' TernaryElse);


	token V_NORM {
		match_fn: normal_match_fn!(classes_::var::REGEX),
		qobj: Some(|tree, env| {
			assert!(tree.is_singular(), concat!("Non-singular tree found for Var: {:?}"), tree);
			assert_eq!(tree.oper.token, V_NORM.deref(), "non-vnorm passed to vnorm");
			env.get(&tree.to_qvar(env)?.unwrap_old())
		}),
		..Token::default()
	};
}
