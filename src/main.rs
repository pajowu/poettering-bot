extern crate egg_mode;
extern crate rustc_serialize;

mod auth;

use egg_mode::tweet::DraftTweet;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};

fn send_tweet(config: auth::Config, status: String) {
    let draft = DraftTweet::new(&status);
    let result = draft.send(&config.con_token, &config.access_token);
    match result {
        Err(e) => println!("{}", e),
        Ok(_) => (),
    }
}

fn load_counter(p: &Path) -> usize {
    let f_ = File::open(p);
    match f_ {
        Err(_) => return 0,
        Ok(mut f) => {
            let mut s = String::new();
            let res = f.read_to_string(&mut s);
            match res {
                Err(_) => return 0,
                Ok(_) => {
                    let counter = s.parse::<usize>();
                    match counter {
                        Err(_) => return 0,
                        Ok(counter) => return counter

                    }
                }
            }
        }
    }
}

fn write_counter(path: &Path, counter: usize) {
    let counter_file = File::create(path);
    match counter_file {
        Err(e) => println!("{:?}", e),
        Ok(mut counter_file) => {
            let res = counter_file.write(counter.to_string().as_bytes());
            match res {
                Err(e) => println!("{:?}", e),
                Ok(_) => {}
            }
        }
    }
}

fn get_next_word(counter: usize) -> Option<String> {
    let prime = 109847;

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
                    let line_num = counter * prime % limit;
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
    let twitter_config = auth::Config::load().unwrap();
    let counter = load_counter(&Path::new("counter"));

    if let Some(word) = get_next_word(counter) {
        send_tweet(twitter_config, format!("Poettering reinvents {}", &word));
    } else {
        println!("Couldn't get next word");
        return;
    }

    write_counter(&Path::new("counter"), counter+1)
    
}
