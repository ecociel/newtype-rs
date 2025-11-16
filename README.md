# simple-newtype

A very simple newtype macro for Rust applications

# Usage

~~~
use simple_newtype::{simple_string_newtype, ParseError};
use std::str::FromStr;
use uuid::Uuid;

simple_string_newtype!(UserId(String));
impl UserId {
    pub fn new() -> Self {
        UserId(Uuid::new_v4().into())
    }
}
impl FromStr for UserId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match uuid::Uuid::parse_str(s) {
            Ok(uuid) => Ok(UserId(uuid.into())),
            Err(_) => Err(ParseError::invalid_syntax(
                "UserId",
                s,
                "not a valid user ID",
            )),
        }
    }
}

fn main() {
    let user_id = UserId::new();
    println!("Hello {}", user_id);
    let not_a_uuid = String::from("not a valid uuid");

    match UserId::try_from(not_a_uuid) {
        Ok(uid) => println!("Hello {}", uid),
        Err(e) => println!("Not a valid user id: {:?}", e),
    }
}
~~~
