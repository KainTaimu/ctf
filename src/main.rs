use crate::constants::SECRETL;
use scanf::scanf;
#[cfg(debug_assertions)]
use std::time::Instant;
use std::{thread, time::Duration};

mod constants;

const SLEEPMS: u64 = 10;

fn main() -> Result<(), i32> {
    #[cfg(debug_assertions)]
    println!("Secret len: {}", SECRETL.len());
    loop {
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
            return Ok(());
        } else {
            println!("Try again.");
            continue;
        }
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
    if s.len() != SECRETL.len() {
        return false;
    }

    let mut hasher = blake3::Hasher::new();
    for (i, c) in s.into_bytes().iter().enumerate() {
        let hash = hasher.update(&[*c]).finalize();

        #[cfg(debug_assertions)]
        println!("{:?} {:?}, {:#?}", c, hash, hasher.count());

        if hash.to_hex().as_bytes() != SECRETL[i] {
            thread::sleep(Duration::from_millis(SLEEPMS / 2));
            return false;
        }
        thread::sleep(Duration::from_millis(SLEEPMS));
    }

    true
}

fn win() {
    println!("You win!");
}
