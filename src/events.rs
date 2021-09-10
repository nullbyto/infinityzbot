use serenity::model::prelude::Activity;
use serenity::framework::standard::{Args, Delimiter};

use crate::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {} in {} servers.", ready.user.name, ready.guilds.len());
        ctx.set_activity(Activity::playing("Kuchi Kuchi Kuchi!")).await;
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let user_id = reaction.user_id.unwrap();
        let msg_r = reaction.message(&ctx).await.unwrap();
        if user_id == ctx.cache.current_user_id().await {return};
        let prefix = read_config("PREFIX");
        if msg_r.content.chars().collect::<Vec<char>>()[0] != prefix.parse::<char>().unwrap() {return};
        
        let reaction_emoji = reaction.emoji;
        if !reaction_emoji.unicode_eq("▶") && !reaction_emoji.unicode_eq("⏹") {return};
        
        let args = msg_r.content.trim().split_whitespace().map(|x| x.to_lowercase()).collect::<Vec<_>>();
        let args_string = args[1..].join(" ");
        let cmd_name = &args[0][1..];
        let mut msg = ctx.http.get_message(reaction.channel_id.0, reaction.message_id.0).await.unwrap();
        if let Some(guild_id) = reaction.guild_id {
            msg.guild_id = Some(guild_id);
        }
        if cmd_name.eq("p") || cmd_name.eq("play") {
            if reaction_emoji.unicode_eq("▶") {
                let _ = play(&ctx, &msg, Args::new(&args_string,&[Delimiter::Single(' ')])).await;
            } else if reaction_emoji.unicode_eq("⏹") {
                let _ = stop(&ctx, &msg, Args::new(&args_string,&[Delimiter::Single(' ')])).await;
            }
        } 
    }
} 