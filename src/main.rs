use std::{thread, time::Duration};

use scanf::scanf;

fn main() -> Result<(), u64> {
    print!("Password: ");
    let input = match get_input() {
        Some(x) => x,
        None => {
            return Ok(());
        }
    };
    let flag = check_pw(input);
    match flag {
        true => Ok(()),
        false => Err(1),
    }
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    if scanf!("{input}").is_ok() {
        return Some(input.trim_end().to_string());
    }
    None
}

fn check_pw(s: String) -> bool {
    const SECRET: &[u8] = b"club";
    const SLEEPMS: u64 = 100;

    if s.len() != SECRET.len() {
        return false;
    }

    for (i, c) in s.into_bytes().iter().enumerate() {
        if *c != SECRET[i] {
            thread::sleep(Duration::from_millis(SLEEPMS));
            return false;
        }
        thread::sleep(Duration::from_millis(SLEEPMS));
    }

    true
}
