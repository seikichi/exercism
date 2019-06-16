use std::borrow::ToOwned;
use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        if input.starts_with(':') {
            return self.eval_definition(input);
        }

        let mut input: Vec<_> = input.split(' ').map(str::to_uppercase).rev().collect();
        while let Some(word) = input.pop() {
            if let Ok(n) = word.parse::<Value>() {
                self.stack.push(n);
                continue;
            }
            if let Some(words) = self.words.get(&word) {
                input.extend(words.iter().rev().map(ToOwned::to_owned));
                continue;
            }

            match (&word[..], self.stack.pop(), self.stack.pop()) {
                // DivisionByZero
                ("/", Some(0), Some(_)) => return Err(Error::DivisionByZero),
                // Ok
                ("+", Some(rhs), Some(lhs)) => self.stack.push(lhs + rhs),
                ("-", Some(rhs), Some(lhs)) => self.stack.push(lhs - rhs),
                ("*", Some(rhs), Some(lhs)) => self.stack.push(lhs * rhs),
                ("/", Some(rhs), Some(lhs)) => self.stack.push(lhs / rhs),
                ("SWAP", Some(rhs), Some(lhs)) => self.stack.extend(&[rhs, lhs]),
                ("OVER", Some(rhs), Some(lhs)) => self.stack.extend(&[lhs, rhs, lhs]),
                ("DUP", Some(rhs), Some(lhs)) => self.stack.extend(&[lhs, rhs, rhs]),
                ("DUP", Some(rhs), None) => self.stack.extend(&[rhs, rhs]),
                ("DROP", Some(_), Some(lhs)) => self.stack.push(lhs),
                ("DROP", Some(_), None) => {}
                // StackUnderflow
                ("+", _, _)
                | ("-", _, _)
                | ("*", _, _)
                | ("/", _, _)
                | ("SWAP", _, _)
                | ("OVER", _, _)
                | ("DUP", _, _)
                | ("DROP", _, _) => return Err(Error::StackUnderflow),
                // UnknownWord
                (_, _, _) => return Err(Error::UnknownWord),
            }
        }
        Ok(())
    }

    fn eval_definition(&mut self, input: &str) -> ForthResult {
        let input: Vec<_> = input.split(' ').map(str::to_uppercase).collect();

        if input[input.len() - 1] != ";" || input[1].parse::<Value>().is_ok() {
            return Err(Error::InvalidWord);
        }

        let mut definition = vec![];
        for w in &input[2..input.len() - 1] {
            if let Some(words) = self.words.get(w) {
                definition.extend(words.iter().map(ToOwned::to_owned));
            } else {
                definition.push(w.clone());
            }
        }
        self.words.insert(input[1].to_string(), definition);
        Ok(())
    }
}
