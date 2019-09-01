extern crate serenity;

use serenity::client::{Client, Context, EventHandler};
use serenity::model::gateway::Ready;
use std::env;

struct Totem;

impl EventHandler for Totem {
	fn ready(&self, _: Context, _: Ready) {
		println!("I'm alive!");
	}
}

fn main() {
	let token = env::var("TOTEM_AUTH").expect("How do you expect me to run without the token?");
	let mut client = Client::new(&token, Totem).expect("An unexpected error occured!");
	if let Err(why) = client.start() {
		println!("Fatal: {:?}", why);
	}
}
