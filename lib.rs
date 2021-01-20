use std::char;
use rand::Rng;

pub fn get_session_key() -> String {
    let (mut result, mut rng) = (String::new(), rand::thread_rng());
    for _i in 0..10 {
        let ch = char::from_digit(rng.gen_range(1, 10), 10).unwrap();
        result.push(ch);
    }
    return result;
}

pub fn get_hash_str() -> String {
    let (mut li, mut rng) = (String::new(), rand::thread_rng());
    for _i in 0..5 {
        let ch = char::from_digit(rng.gen_range(1, 7), 10).unwrap();
        li.push(ch);
    }
    return li;
}

pub fn next_session_key(hash_str: &str, session_key: &str) -> String {
    let mut result = 0;
    if hash_str.is_empty() {return "Hash is empty".to_string()}
    for idx in hash_str.chars() {
        if !idx.is_ascii_digit() {return "Hash contains non-digit letter".to_string()}
    }
    for idx in hash_str.chars() {
        let l = idx.to_string();
        result += calc_hash(session_key.to_string(), l.parse::<u64>().unwrap()).parse::<u64>().unwrap();
    }
    return result.to_string();
}

fn calc_hash(session_key: String, val: u64) -> String {
    match val {
        1 => {
            let result = "00".to_string() + &(session_key[0..5].parse::<u64>().unwrap() % 97).to_string();
            return result[result.len() - 2..result.len()].to_string()
        }
        2 => {
            let result = session_key.chars().rev().collect::<String>();
            return result + &session_key.chars().nth(0).unwrap().to_string()
        }
        3 => {
            return session_key[session_key.len() - 5..session_key.len()].to_string() + &session_key[0..5].to_string()
        }
        4 => {
            let mut result = 0;
            for _i in 1..9 {
                result += session_key.chars().nth(_i).unwrap().to_digit(10).unwrap() as u64 + 41;
            }
            return result.to_string()
        }
        5 => {
            let mut ch: char;
            let mut result = 0;
            for _i in 0..session_key.len() {
                ch = ((session_key.chars().nth(_i).unwrap() as u8) ^ 43) as char;
                if !ch.is_ascii_digit() {
                    ch = (ch as u8) as char;
                }
                result += ch as u64;
            }
            return result.to_string()
        }
        _ => {
            return (session_key.parse::<u64>().unwrap() + val).to_string()
        }
    }
}