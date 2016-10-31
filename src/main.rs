extern crate egg_mode;
#[macro_use(object)] extern crate json;

mod config;

use egg_mode::tweet::DraftTweet;
use std::path::Path;
use std::fs::File;
use std::io::{Read};

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

fn is_bad_word(word: &str) -> Option<bool> {
    let f_ = File::open(&Path::new("blacklist"));

    match f_ {
        Err(_) => None,
        Ok(mut f) => {
            let mut s = String::new();
            let res = f.read_to_string(&mut s);
            match res {
                Err(_) => None,
                Ok(_) => {
                    let lines = s.lines();
                    for line in lines {
                        if line == word {
                            return Some(true)
                        }
                    }
                    return Some(false)
                }
            }
        }
    }
}
fn get_next_word(counter: usize) -> Result<String, String> {

    assert!(counter % PRIME != 0, format!("Wordfile cannot contain a multiple of {} number lines", PRIME));

    let f_ = File::open(&Path::new("wordlist"));

    match f_ {
        Err(_) => Err("Error opening file".to_string()),
        Ok(mut f) => {
            let mut s = String::new();
            let res = f.read_to_string(&mut s);
            match res {
                Err(_) => Err("Error reading file".to_string()),
                Ok(_) => {
                    let mut lines = s.lines();
                    let limit = lines.clone().count();
                    let line_num = counter * PRIME % limit;
                    let line = lines.nth(line_num);
                    match line {
                        None => Err("Can't read word".to_string()),
                        Some(word) =>  {
                            let ibw = is_bad_word(word);
                            match ibw {
                                Some(b) => {
                                    if !b {
                                        return Ok(word.to_string())
                                    } else {
                                        Err("Bad word".to_string())
                                    }
                                },
                                None => Err("Error when checking for bad word".to_string())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn generate_tweet(config: &mut Config) -> Option<String> {
    let mut counter = config.counter.unwrap();
    let mut tweet = "".to_string();
    while tweet == "" {
        match get_next_word(counter) {
            Ok(word) => tweet = format!("Poettering reinvents {}", &word),
            Err(err) => { 
                if err != "Bad word".to_string() {
                    println!("Couldn't get next word, Err: {}", err);
                    return None
                }
            },
        }
        counter += 1;
    }
    config.counter = Some(counter);
    
    Some(tweet)
}

fn main() {
    let mut config = Config::load("config.json");

    let valid = config.validate();

    if valid {
        let tweet = generate_tweet(&mut config);
        if tweet.is_some() {
            send_tweet(&config, tweet.unwrap().to_string());
        }
        
    }

    config.save("config.json")
}
