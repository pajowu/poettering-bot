extern crate egg_mode;


use std;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use rustc_serialize::json;


pub struct Config {
    pub con_token: egg_mode::Token<'static>,
    pub access_token: egg_mode::Token<'static>,
    pub user_id: i64,
    pub screen_name: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct JsonConfig {
    pub access_key: String,
    pub access_secret: String,
    pub user_id: i64,
    pub screen_name: String,
}

impl Config {

    fn authenticate(token: &egg_mode::Token) {
        let request_token = egg_mode::request_token(&token, "oob").unwrap();

        println!("Go to the following URL, sign in, and give me the PIN that comes back:");
        println!("{}", egg_mode::authorize_url(&request_token));

        let mut pin = String::new();
        std::io::stdin().read_line(&mut pin).unwrap();
        println!("");
        let tok_result = egg_mode::access_token(&token, &request_token, pin).unwrap();

        let access_token = tok_result.0;
        let user_id = tok_result.1;
        let username = tok_result.2;

        let json_config = JsonConfig {
            access_key: access_token.key.into_owned(),
            access_secret: access_token.secret.into_owned(),
            user_id: user_id,
            screen_name: username
        };

        let encoded = json::encode(&json_config).unwrap();

        let mut config_file = File::create("twitter_token.json").unwrap();
        let res = config_file.write(encoded.as_bytes());

        match res {
            Err(e) => println!("{:?}", e),
            _ => {}
        }

    }

    fn load_config(mut config_file: &File, token: egg_mode::Token<'static>) -> Option<Config> {
        let mut config_string = String::new();
        let res = config_file.read_to_string(&mut config_string);

        match res {
            Err(_) => None,
            Ok(_) => {
                let tmp_config: JsonConfig = json::decode(&config_string).unwrap();

                Some(Config {
                    con_token: token,
                    access_token: egg_mode::Token::new(tmp_config.access_key, tmp_config.access_secret),
                    user_id: tmp_config.user_id,
                    screen_name: tmp_config.screen_name
                })
            }
        }
    }

    pub fn load() -> Option<Self> {
        let consumer_key = include_str!("consumer_key").trim();
        let consumer_secret = include_str!("consumer_secret").trim();

        let token: egg_mode::Token<'static> = egg_mode::Token::new(consumer_key, consumer_secret);

        if let Err(_) = File::open(&Path::new("twitter_token.json")) {
            Self::authenticate(&token);
        }

        let config_file = File::open(&Path::new("twitter_token.json")).unwrap();

        let config = Self::load_config(&config_file, token);

        if config.is_none() { return None }

        let mut config = config.unwrap();

        if let Err(err) = egg_mode::verify_tokens(&config.con_token, &config.access_token) {
            println!("We've hit an error using your old tokens: {:?}", err);
            println!("We'll have to reauthenticate before continuing.");
            std::fs::remove_file("twitter_token.json").unwrap();
            Self::authenticate(&config.con_token);
            config = Self::load_config(&config_file, config.con_token).unwrap();
        }

        Some(config)
    }

}