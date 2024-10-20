// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let name = parts[0].trim();
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        let age_str = parts[1].trim();
        let age = age_str.parse::<usize>().map_err(ParsePersonError::ParseInt)?;

        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

// 计算字节长度的函数
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// 计算字符数量的函数
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);

    let s = "Hello, world!";
    println!("Byte count: {}", byte_counter(s));
    println!("Char count: {}", char_counter(s));

    let s = "こんにちは、世界！";
    println!("Byte count: {}", byte_counter(s));
    println!("Char count: {}", char_counter(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_counter() {
        assert_eq!(byte_counter("hello"), 5);
        assert_eq!(byte_counter("こんにちは"), 15);
    }

    #[test]
    fn test_char_counter() {
        assert_eq!(char_counter("hello"), 5);
        assert_eq!(char_counter("こんにちは"), 5);
    }
}
