mod commands;
mod events;
use commands::{
    general::*,
    player::*,
    owner::*,
    fun::*
};

use serenity::{
    prelude::*,
    async_trait,
    http::Http,
    client::{
        Client,
        Context,
        EventHandler
    },
    framework::standard::{
        StandardFramework,
        macros::{
            group,
        }
    },
    model::{
        channel::Reaction,
        gateway::Ready
    },
};
use songbird::SerenityInit;// register_songbird()

use serde_json;
use std::fs;

#[group]
#[commands(ping)]
struct General;

#[group]
#[commands(join, leave, play, stop, list, upload)]
#[summary = "Commands for playing sounds"]
#[allowed_roles("♠CLUB HOUSE♠")]
struct Player;

#[group]
#[commands(el3n, el3no)]
#[summary = "For fun"]
struct Fun;

#[group]
#[commands(delete, rename)]
#[summary = "Commands for owner"]
#[owners_only]
struct Owner;

#[tokio::main]
async fn main() {
    let prefix = read_config("PREFIX");
    let token = read_config("TOKEN");

    let http = Http::new(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = std::collections::HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&prefix).owners(owners).on_mention(Some(bot_id)))
        .group(&GENERAL_GROUP)
        .group(&PLAYER_GROUP)
        .group(&FUN_GROUP)
        .group(&OWNER_GROUP)
        .help(&MY_HELP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
            .event_handler(events::Handler)
            .framework(framework)
            .register_songbird()
            .await
            .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error while running the client: {:?}", why);
    }
}

fn read_config(config: &str) -> String {
    let data = fs::read_to_string("./config.json").expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    format!("{}", res[config].as_str().unwrap())
}
