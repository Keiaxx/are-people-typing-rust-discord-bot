#[macro_use]
extern crate lazy_static;

// Config
extern crate config;
use std::collections::HashMap;

// Discord library
use serenity::model::event::TypingStartEvent;
use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use serenity::model::user::User;
use serenity::{
    http::AttachmentType,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct Handler;

lazy_static! {
    static ref TYPING_NOTIFS_ENABLED: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

impl EventHandler for Handler {
    // Message recvd events
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "rustytyping" {
            if TYPING_NOTIFS_ENABLED.load(Ordering::SeqCst) {
                TYPING_NOTIFS_ENABLED.store(false, Ordering::SeqCst);

                if let Err(why) = msg.channel_id.say(
                    &ctx.http,
                    "Unnecessary typing notifications have been disabled",
                ) {
                    println!("Error sending message: {:?}", why);
                }
            } else {
                TYPING_NOTIFS_ENABLED.store(true, Ordering::SeqCst);

                if let Err(why) = msg.channel_id.say(
                    &ctx.http,
                    "Unnecessary typing notifications have been enabled",
                ) {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    fn typing_start(&self, ctx: Context, typing_event: TypingStartEvent) {
        // Ensure guild
        if typing_event.guild_id.is_some() && TYPING_NOTIFS_ENABLED.load(Ordering::SeqCst) {
            let guild: GuildId = typing_event.guild_id.unwrap();

            

            // Try and get user from guild
            let whotyping: User = typing_event.user_id.to_user(&ctx.http).unwrap(); // Returns Result(Member)

            match guild.member(&ctx.http, whotyping.id) {
                Ok(member) => {
                    let member: Member = member;
                    let membername = if member.nick.is_some() {
                        member.nick.unwrap()
                    } else {
                        whotyping.name
                    };

                    let msg = typing_event.channel_id.send_message(&ctx.http, |m| {
                        // m.content("Hello, World!");
                        m.embed(|e| {
                            e.title("Someone is typing!");
                            e.description(format!("You must check out what {} is typing!", membername));
                            // e.image("attachment://poggers_fish.gif");
                            e.image("https://cdn.discordapp.com/attachments/110419059232780288/704069207297622056/poggers_fish.gif");
                            e.fields(vec![
                                ("SoMeONe", ".", true),
                                (
                                    ".",
                                    "COULD IT BE A",
                                    true,
                                ),
                            ]);
                            e.field(
                                "IS T Y P I N G",
                                "ooOoooO ooooOO",
                                false,
                            );
                            e.fields(vec![
                                ("O N A Y NAY", ".", true),
                                (
                                    ".",
                                    "MyStERiouS MEssage/?!@?/1",
                                    true,
                                ),
                            ]);
                            e.footer(|f| {
                                f.text(format!("It seems to be that {} is typing OwO", membername));
                                f
                            });
                            e
                        });
                        // m.add_file(AttachmentType::Path(Path::new("./poggers_fish.gif")));
                        m
                    });

                    if let Err(why) = msg {
                        println!("Error sending message: {:?}", why);
                    }
                }
                Err(e) => println!("There was an error getting the member from the guild"),
            };
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Attempt load config from Settings.toml
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
    let settings_map: HashMap<String, String> =
        settings.try_into::<HashMap<String, String>>().unwrap();
    let discord_token = settings_map.get("discord_token");

    // Ensure token is there
    let token = discord_token.expect("Expected a token in the environment");

    // Create client
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Start bot
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
