use std::vec::Vec;

#[derive(Debug)]
pub enum Token {
    Number(usize),
    Delimiter(char),
}

pub fn parse(str: String) -> Vec<Token> {
    let mut list = Vec::new();
    let mut num_start: Option<usize> = None;
    for (i, c) in str.chars().enumerate() {
        match c {
            '0'..='9' => {
                if num_start == None {
                    num_start = Some(i);
                }
            },
            _ => {
                if num_start.is_some() {
                    let slice = &str[num_start.unwrap()..i];
                    list.push(Token::Number(slice.parse::<usize>().unwrap()));
                    num_start = None;
                }
                match c {
                    ',' | '.' | '-' => list.push(Token::Delimiter(c)),
                    _ => (),
                }
            },
        }
    }
    return list;
}
