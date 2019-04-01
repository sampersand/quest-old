#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
	NothingParsed,
	NoEnvironmentLeft
}