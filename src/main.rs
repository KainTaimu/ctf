#[cfg(debug_assertions)]
use std::time::Instant;
use std::{thread, time::Duration};

use scanf::scanf;

const SECRET: &[u8] = b"clubeh";
const SLEEPMS: u64 = 67;

fn main() -> Result<(), i32> {
    print!("Password: ");
    let input = match get_input() {
        Some(x) => x,
        None => {
            return Ok(());
        }
    };

    let flag: bool;
    #[cfg(not(debug_assertions))]
    {
        flag = check_pw(input);
    }
    #[cfg(debug_assertions)]
    {
        let now = Instant::now();
        flag = check_pw(input);
        println!("{:#?}", now.elapsed());
    }

    if flag {
        win();
        Ok(())
    } else {
        Err(1)
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

fn win() {
    println!("You win!");
}
