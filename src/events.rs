use std::time::Duration;
use serenity::model::voice::VoiceState;
use serenity::http::CacheHttp;
use serenity::model::prelude::Activity;
use serenity::framework::standard::{Args, Delimiter};

use crate::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {} in {} servers.", ready.user.name, ready.guilds.len());
        ctx.set_activity(Activity::playing("Kuchi Kuchi | $help")).await;
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let user_id = reaction.user_id.unwrap();
        let msg_r = reaction.message(&ctx).await.unwrap();
        if user_id == ctx.cache.current_user_id() {return};
        let prefix = read_config("PREFIX");
        let content = &msg_r.content;
        if !content.is_empty() && content.chars().collect::<Vec<char>>()[0] != prefix.parse::<char>().unwrap() {return};
        
        let reaction_emoji = reaction.emoji;
        if !reaction_emoji.unicode_eq("▶") && !reaction_emoji.unicode_eq("⏹") {return};
        
        let args = msg_r.content.trim().split_whitespace().map(|x| x.to_lowercase()).collect::<Vec<_>>();
        let args_string = args[1..].join(" ");
        let cmd_name = &args[0][1..];
        let mut msg = ctx.http.get_message(reaction.channel_id.0, reaction.message_id.0).await.unwrap();
        
        if cmd_name.eq("p") || cmd_name.eq("play") {
            if let Some(guild_id) = reaction.guild_id {
                msg.guild_id = Some(guild_id);
                if let Some(g) = ctx.cache.guild(guild_id) {
                    let guild = g;
                    let member = guild.member(&ctx, user_id).await.unwrap();
                    msg.author = member.user;
                }
            }

            if reaction_emoji.unicode_eq("▶") {
                let _ = play(&ctx, &msg, Args::new(&args_string,&[Delimiter::Single(' ')])).await;
            } else if reaction_emoji.unicode_eq("⏹") {
                let _ = stop(&ctx, &msg, Args::new(&args_string,&[Delimiter::Single(' ')])).await;
            }
        } 
    }

    async fn voice_state_update(
        &self,
        ctx: Context,
        _old: Option<VoiceState>,
        new: VoiceState,
    ) {
        match new.channel_id {
            Some(id) => {
                if id == 454119833681002506 || id == 312603720934227968 {
                    let member = new.member.unwrap();
                    let guild_id = new.guild_id.unwrap();
                    // 1800 secs = 30 mins
                    tokio::time::sleep(Duration::from_secs(1800)).await;

                    let guild = ctx.cache.guild(guild_id).unwrap();

                    for channel in guild.channels(ctx.http()).await.unwrap().values() {
                        if channel.id == id {
                            let members = channel.members(ctx.cache().unwrap()).await.unwrap();
                            for mmbr in members {
                                if mmbr.user.id == member.user.id {
                                    let _ = member.disconnect_from_voice(ctx.http()).await;
                                    return;
                                }
                            }
                        }
                    }
                }
            },
            None => {}
        }
    }
} 
