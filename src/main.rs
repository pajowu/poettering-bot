extern crate egg_mode;
#[macro_use(object)] extern crate json;

mod config;

use egg_mode::tweet::DraftTweet;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};

use config::Config;

const PRIME: usize = 109847;

fn send_tweet(config: &Config, status: String) {
    let draft = DraftTweet::new(&status);
    let result = draft.send(
        &config.con_token.as_ref().unwrap(),
        &config.access_token.as_ref().unwrap()
    );
    match result {
        Err(e) => println!("{}", e),
        Ok(_) => (),
    }
}

fn get_next_word(counter: usize) -> Option<String> {

    let f_ = File::open(&Path::new("wordlist"));

    match f_ {
        Err(_) => None,
        Ok(mut f) => {
            let mut s = String::new();
            let res = f.read_to_string(&mut s);
            match res {
                Err(_) => None,
                Ok(_) => {
                    let mut lines = s.lines();
                    let limit = lines.clone().count();
                    let line_num = counter * PRIME % limit;
                    let line = lines.nth(line_num);
                    match line {
                        None => None,
                        Some(word) => return Some(word.to_string())
                    }
                }
            }
        }
    }
}

fn main() {
    let mut config = Config::load("config.json");

    let valid = config.validate();

    if valid {
        if let Some(word) = get_next_word(config.counter.unwrap()) {
            send_tweet(&config, format!("Poettering reinvents {}", &word));
        } else {
            println!("Couldn't get next word");
            return;
        }
        config.counter = Some(config.counter.unwrap() + 1);
    }

    config.save("config.json")
}
