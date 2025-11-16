use simple_newtype::{simple_string_newtype, ParseError};
use std::str::FromStr;

simple_string_newtype!(ShortId(String));
impl FromStr for ShortId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 10 {
            return Err(ParseError::length_limit_exceeded("ShortId", s, 10));
        }
        Ok(ShortId(s.into()))
    }
}

fn main() {
    let shortid = ShortId::try_from("1234").expect("ok");
    println!("Hello {}", shortid);

    let invalid = String::from("this is wayyyyyyyy too long");
    match ShortId::try_from(invalid) {
        Ok(id) => println!("Hello {}", id),
        Err(e) => println!("Not a valid user id: {:?}", e),
    }
}
