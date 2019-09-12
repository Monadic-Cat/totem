extern crate serenity;

use mice::prelude::*;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{channel::Message, gateway::Ready};
use std::env;

struct Totem;

const ROLLP: &str = "!roll ";

impl EventHandler for Totem {
	fn ready(&self, _: Context, _: Ready) {
		println!("I'm alive!");
	}
	fn message(&self, ctx: Context, msg: Message) {
		if msg.content.starts_with(ROLLP) {
			match msg.channel_id.say(
				&ctx.http,
				match roll(&msg.content[ROLLP.len()..]) {
					Ok(x) => x.format(MiceFormat::new().total_right()),
					Err(x) => format!("{}", x),
				},
			) {
				Ok(_) => (),
				Err(x) => eprintln!("{}", x),
			}
		}
	}
}

fn main() {
	let token = env::var("TOTEM_AUTH").expect("How do you expect me to run without the token?");
	let mut client = Client::new(&token, Totem).expect("An unexpected error occured!");
	if let Err(why) = client.start() {
		println!("Fatal: {:?}", why);
	}
}
