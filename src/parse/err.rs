#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {

}

pub type Result<T> = ::std::result::Result<Error, T>;