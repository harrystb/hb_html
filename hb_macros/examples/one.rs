use hb_macros::{context, context2};
use hb_parse::error::{ParseResult, ParseError};
struct eh;

impl eh {
    #[context("Something")]
    pub fn eh() -> ParseResult<()> {
        eh::b()?;
        Ok(())
    }

    #[context2("Better?")]
    pub fn hm() -> ParseResult<()> {
        Err(ParseError::with_msg("inner"))
    }

    #[context2("Better?")]
    pub fn ah() -> ParseResult<()> {
        Err(ParseError::from(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "This is an io error")))
    }


    pub fn b() -> ParseResult<()> {
        Err(ParseError::with_msg("inner"))
    }
}

fn main() {
    println!("{}", eh::eh().err().unwrap());
    println!("{}", eh::hm().err().unwrap());
    println!("{}", eh::ah().err().unwrap());
}