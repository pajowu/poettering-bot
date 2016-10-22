extern crate egg_mode;
extern crate rustc_serialize;

mod auth;

use egg_mode::tweet::DraftTweet;
use egg_mode::Response;

fn send_tweet(config: auth::Config, status: &str) {
	let draft = DraftTweet::new(status);
	let result = draft.send(&config.con_token, &config.access_token);
	match result {
        Err(e) => println!("{}", e),
        Ok(r) => (),
    }
}

fn get_next_word(){

}
fn main() {
    let twitter_config = auth::Config::load().unwrap();
    //send_tweet(twitter_config, "This is an example status!");
}
