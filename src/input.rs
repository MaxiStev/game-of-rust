use std::vec::Vec;

#[derive(Debug)]
enum Token {
    Number(usize),
    Delimiter(char),
}

#[derive(Debug)]
pub struct Range {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Expression {
    Range(Range),
    Integer(usize),
}

#[derive(Debug)]
pub struct Pair {
    x: Range,
    y: Range,
}

// lexical analysis
fn tokenize(str: String) -> Vec<Token> {
    let mut tokens = Vec::new();
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
                    tokens.push(Token::Number(slice.parse::<usize>().unwrap()));
                    num_start = None;
                }
                if let ',' | '.' | '-' = c { tokens.push(Token::Delimiter(c)) }
            },
        }
    }
    return tokens;
}

fn build_exprs(tokens: &mut Vec<Token>) -> Vec<Expression> {
    let mut exprs = Vec::new();
    let mut list = tokens.iter();
    let mut string: String = "".to_string();
    let mut is_range: bool = false;
    let mut range_start: usize = 0;
    let mut last_was_delim: bool = false;
    while let Some(token) = list.next() {
        match token {
            Token::Number(number) => {
                string.push_str(&number.to_string());
                last_was_delim = false;
            },
            _ => {
                if let Token::Delimiter(del) = token {
                    if last_was_delim { continue; }
                    last_was_delim = true;
                    if is_range {
                        let mut range_end: usize = string.parse().unwrap();
                        string = "".to_string();
                        if range_end < range_start {
                            std::mem::swap(&mut range_start, &mut range_end);
                        }
                        println!("\t\t\t\t{:?}", Expression::Range(Range{start: range_start, end: range_end}));
                        exprs.push(Expression::Range(Range{start: range_start, end: range_end}));
                        is_range = false;
                    } else if *del == '-' {
                        range_start = string.parse().unwrap();
                        string = "".to_string();
                        is_range = true;
                    } else {
                        println!("\t\t\t\t{:?}", Expression::Integer(string.parse().unwrap()));
                        exprs.push(Expression::Integer(string.parse().unwrap()));
                        string = "".to_string();
                    }
                }
            },
        }
        println!("\ntoken:\t\t\t{:?}\nstring:\t\t\t{string}\nis_range:\t\t{is_range}\nrange_start:\t\t{range_start}\nlast_was_delim:\t\t{last_was_delim}", token);
    }
    if is_range {
        let mut range_end: usize = string.parse().unwrap();
        if range_end < range_start {
            std::mem::swap(&mut range_start, &mut range_end);
        }
        println!("\t\t\t\t{:?}", Expression::Range(Range{start: range_start, end: range_end}));
        exprs.push(Expression::Range(Range{start: range_start, end: range_end}));
    }
    return exprs;
}

fn build_pairs(mut exprs: Vec<Expression>) -> Vec<Pair> {
    for e in &exprs {
        println!("{:?}", e);
    }
    let mut pairs: Vec<Pair> = Vec::new();
    while exprs.len() > 1 {
        println!("{}", exprs.len());
        let mut pair :Vec<Range> = Vec::new();
        for _ in 0..2 {
            let expr = exprs.pop().unwrap();
            match expr {
                Expression::Integer(num) => {
                    pair.push(Range{start: num, end: num + 1});
                },
                Expression::Range(mut range) => {
                    if range.start == range.end {
                        range.end += 1;
                    }
                    pair.push(range);
                },
            }
        }
        pairs.push(Pair{x: pair.pop().unwrap(), y: pair.pop().unwrap()});
    }
    pairs.reverse();
    return pairs;
}

pub fn parse(str: String) -> Vec<Pair> {
    let mut tokens = tokenize(str);
    let exprs = build_exprs(&mut tokens);
    build_pairs(exprs)
}

