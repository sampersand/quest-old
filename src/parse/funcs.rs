use std::path::Path;
use std::{io, fs};
use parse::{Token, TokenMatch, MatchData, Stream, Tree};
use obj::QObject;
use env::Environment;

pub fn parse_file<P: AsRef<Path>>(file: P, env: &Environment) -> io::Result<QObject> {
	let file = file.as_ref();
	Ok(parse(&mut Stream::from_file(&file.to_string_lossy(), &fs::read_to_string(file)?), env))
}

pub fn parse_str(text: &str, env: &Environment) -> QObject {
	parse(&mut Stream::from_str(text), env)
}

fn parse(stream: &mut Stream, env: &Environment) -> QObject {
	let matches = matches_until(stream, env, |_| false);
	Tree::try_from_vec(matches).map(|x| x.execute(env)).unwrap_or_else(|| ().into())
}

pub fn matches_until(stream: &mut Stream, env: &Environment, until: fn(&TokenMatch) -> bool) -> Vec<TokenMatch> {
	let mut matches = Vec::new();
	let tokens = env.tokens.read();
	while !stream.is_empty() {
		let tokenmatch = tokens.iter()
			.filter_map(|token| (token.match_fn)(stream, env).map(|data| TokenMatch::new(data, token, stream.get_src())))
			.rev()
			.max_by(|x, y| x.data.cmp(&y.data))
			.unwrap_or_else(|| panic!("No tokens found for {:?}", stream));

		stream.offset_by(tokenmatch.data.len());

		if until(&tokenmatch) || tokenmatch.data.is_eof() {
			break
		}

		if !tokenmatch.data.is_notoken() {
			matches.push(tokenmatch)
		}
	}
	matches
}