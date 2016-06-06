extern crate discord;
extern crate rand;

use discord::Discord;
use discord::model::Event;
use rand::{thread_rng, Rng};
use std::env;

fn main() {
    let token = env::var("DICEID").expect("no $DICEID found");
    let discord = Discord::from_bot_token(&token).expect("login failed");
    let (mut connection, _) = discord.connect().expect("connect failed");

	println!("Ready.");
    let mut rng = thread_rng();
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
				println!("{} says: {}", message.author.name, message.content);
				if message.content == "!test" {
					let _ = discord.send_message(&message.channel_id, "This is a reply to the test.", "", false);
                } else if message.content == "!r" {
                    let _ = discord.send_message(&message.channel_id, &format!("rng[0,100): {}", rng.gen_range(0,100) ), "", false );
				} else if message.content == "!quit" {
					println!("Quitting.");
					break
				}
			}
			Ok(_) => {}
			Err(err) => {
                println!("Receive error: {:?}", err);
                break
            }
		}
	}

	// Log out from the API
	discord.logout().expect("logout failed");
}
