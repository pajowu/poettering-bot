use std;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};

use json;
use egg_mode;
use egg_mode::Token;

pub struct Config<'a> {
    pub con_token: Option<Token<'a>>,
    pub access_token: Option<Token<'a>>,
    pub counter: Option<usize>
}

impl<'a> Config<'a> {

    fn parse_token(j: json::JsonValue) -> Option<Token<'a>> {
        let key = j["key"].as_str();
        let secret = j["secret"].as_str();

        if key.is_some() && secret.is_some() {
            let key = key.unwrap().to_string();
            let secret = secret.unwrap().to_string();
            Some(Token::new(key, secret))
        } else {
            None
        }

        
    }

    fn parse_config(s: &str) -> Option<Config<'a>>{
        let parsed = match json::parse(s) {
            Ok(p) => p,
            Err(_) => return None
        };

        let counter: usize = match parsed["counter"].as_number() {
            None => return None,
            Some(c) => c.into()
        };

        let consumer_token = parsed["con_token"].clone();
        let access_token = parsed["access_token"].clone();

        let config = Config {
            con_token: Self::parse_token(consumer_token),
            access_token: Self::parse_token(access_token),
            counter: Some(counter),
        };

        Some(config)
    }

    fn load_config<'b>(mut f: &File) -> Option<Config<'a>> {
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Err(_) => None,
            Ok(_) => Self::parse_config(&s)
        }
    }

    pub fn clone_token(token: &Token) -> Token<'a> {
        let key = token.key.clone().into_owned();
        let secret = token.secret.clone().into_owned();
        Token::new(key, secret)
    }

    pub fn clone_config(config: &Config) -> Config<'a> {
        let consumer_token = match config.con_token {
            Some(ref c) => Some(Self::clone_token(c)),
            None => None,
        };
        let access_token = match config.access_token {
            Some(ref c) => Some(Self::clone_token(c)),
            None => None,
        };
        let config: Config = Config {
            access_token: access_token,
            con_token: consumer_token,
            counter: config.counter
        };
        config

    }

    fn authenticate(config: &Config<'a>) -> Option<Config<'a>> {
        let consumer_token = match config.con_token {
            Some(ref c) => Self::clone_token(c),
            None => return None,
        };
        let request_token = egg_mode::request_token(&consumer_token, "oob").unwrap();

        println!("Go to the following URL, sign in, and give me the PIN that comes back:");
        println!("{}", egg_mode::authorize_url(&request_token));

        let mut pin = String::new();
        std::io::stdin().read_line(&mut pin).unwrap();
        println!("");
        let tok_result = egg_mode::access_token(&consumer_token, &request_token, pin).unwrap();

        let access_token = tok_result.0;

        let config: Config = Config {
            access_token: Some(access_token),
            con_token: Some(consumer_token),
            counter: config.counter
        };

        Some(config)
    }

    pub fn load(p: &str) -> Self {

        let path = &Path::new(p);

        let config = match File::open(path) {
            Err(_) => {
                println!("Couldn't open config file");
                None
            },
            Ok(config_file) => Self::load_config(&config_file)
        };

        match config {
            Some(c) => c,
            None => Config { access_token: None,
                    con_token: None,
                    counter: Some(0) }
        }
    }

    fn ensure_valid(config: &Config<'a>) -> Option<Config<'a>> {
        match egg_mode::verify_tokens(&config.con_token.as_ref().unwrap(), &config.access_token.as_ref().unwrap()) {
            Err(_) => {
                println!("Tokens Invalid, reauthenticating.");
                Self::authenticate(config)
            },
            Ok(_) => Some(Self::clone_config(config))
        }
    }

    fn set_self<'b>(slf: &mut Config<'b>, cfg: Config<'b>) {
        slf.con_token = cfg.con_token;
        slf.access_token = cfg.access_token;
        slf.counter = match cfg.counter {
            Some(c) => Some(c),
            None => Some(0)
        }
    }

    pub fn validate(&mut self) -> bool {
        let slf = Self::clone_config(self);
        match slf.con_token {
            None => {
                println!("Please add the consumer token to config.json");
                return false
            },
            Some(_) => {
                match slf.access_token {
                    None => {
                        let cfg = Self::authenticate(self).unwrap();
                        Self::set_self(self, cfg);

                    },
                    Some(_) => {
                        let cfg = Self::ensure_valid(self).unwrap();
                        Self::set_self(self, cfg);}
                }
                true
            }
        }
    }

    fn serialize_token(t_: Option<Token>) -> Option<json::JsonValue> {
        match t_ {
            None => None,
            Some(t) => {
                Some(object!{
                    "key" => t.key.into_owned(),
                    "secret" => t.secret.into_owned()
                })
            }
        }
    }

    fn serialize_config(config: &Config) -> String {
        let ct = match config.con_token {
            None => None,
            Some(ref c) => Some(Self::clone_token(c))
        };
        let at = match config.access_token {
            None => None,
            Some(ref c) => Some(Self::clone_token(c))
        };

        let con_token = Self::serialize_token(ct);
        let access_token = Self::serialize_token(at);

        let serialized_config = object!{
            "con_token" => con_token,
            "access_token" => access_token,
            "counter" => config.counter
        };
        serialized_config.to_string()
    }

    pub fn save(&self, p: &str) {
        let path = &Path::new(p);
        match File::create(path) {
            Err(_) => {
                println!("Couldn't open config file");
            },
            Ok(mut config_file) => {
                let encoded = Self::serialize_config(self);
                config_file.write(encoded.as_bytes());
            }
        }
        ()
    }
}