use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
        Args,
    },
    utils::{parse_username}
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::{
    path::Path,
    thread,
    time
};

#[command]
#[only_in(guilds)]
#[aliases("e1")]
#[description = "bl3n the tagged user"]
#[usage = "[@user] [times] [sound-name] | el3n [@user] [times]"]
#[example = "@3min 5 zewzew | el3n @3min 5"]
async fn el3n(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_mention = match args.single::<String>() {
        Ok(sound) => sound,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "❌ You have to mention someone!").await?;
            return Ok(());
        }
    };
    let user_id_nr = match parse_username(&user_mention) {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "❌ You have to use @ to mention someone!").await?;
            return Ok(());
        }
    };

    let guild = msg.guild(&ctx.cache).unwrap();
    let user_id = UserId(user_id_nr);
    let member = guild.member(ctx, user_id).await.unwrap();

    let channel_id = match guild
        .voice_states.get(&member.user.id)
        .and_then(|voice| voice.channel_id) {
            Some(id) => id,
            None => {
                msg.channel_id.say(&ctx.http, "❌ Error! User must be in a channel.").await?;
                return Ok(());
            }
    };

    let args_vec = args.rest().split_whitespace().collect::<Vec<_>>();
    let mut times = 1i8;
    let mut sound_name = "brazil";
    if !args.rest().is_empty() {
        times = args_vec[0].parse::<i8>().unwrap();
        if times > 10 || times < 1 {
            msg.reply(ctx, "10 max, kuchi kuchi!").await?;
            return Ok(());
        } 
    }
    if args_vec.len() >= 2 {
        sound_name = args_vec[1];
    }

    msg.react(ctx, '👌').await?;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.");
    
    let _handler = manager.join(guild.id, channel_id).await;

    let path = Path::new(format!("./sounds/{}.mp3", sound_name).as_str()).to_owned();

    if let Some(handler_lock) = manager.get(guild.id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ffmpeg(path).await {
            Ok(src) => src,
            Err(why) => {
                println!("Err starting source: {:?}", why);
                msg.channel_id.say(&ctx.http, "❌ Error sourcing ffmpeg").await?;
                return Ok(());
            }
        };

        let track_handler = handler.play_source(source);
        while !track_handler.get_info().await.unwrap().playing.is_done(){}
    }
    let to1 = ChannelId(861751113916612648);
    let to2 = ChannelId(861751155000213544);
    
    for _ in 0..times {
        guild.move_member(&ctx.http, user_id, to1).await?;
        thread::sleep(time::Duration::from_millis(700));
        guild.move_member(&ctx.http, user_id, to2).await?;
        thread::sleep(time::Duration::from_millis(700));
    }
    guild.move_member(&ctx.http, user_id, channel_id).await?;
    manager.leave(guild.id).await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("e2")]
#[description = "bl3n the tagged user without sound"]
#[usage = "[@user] [times]"]
#[example = "@3min 5"]
async fn el3no(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_mention = match args.single::<String>() {
        Ok(sound) => sound,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "❌ You have to mention someone!").await?;
            return Ok(());
        }
    };
    let user_id_nr = match parse_username(&user_mention) {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "❌ You have to use @ to mention someone!").await?;
            return Ok(());
        }
    };

    let guild = msg.guild(&ctx.cache).unwrap();
    let user_id = UserId(user_id_nr);
    let member = guild.member(ctx, user_id).await.unwrap();

    let channel_id = match guild
        .voice_states.get(&member.user.id)
        .and_then(|voice| voice.channel_id) {
            Some(id) => id,
            None => {
                msg.channel_id.say(&ctx.http, "❌ Error! User must be in a channel.").await?;
                return Ok(());
            }
    };

    let args_vec = args.rest().split_whitespace().collect::<Vec<_>>();
    let mut times = 1i8;
    if !args.rest().is_empty() {
        times = args_vec[0].parse::<i8>().unwrap();
        if times > 10 || times < 1 {
            msg.reply(ctx, "10 max, kuchi kuchi!").await?;
            return Ok(());
        } 
    }

    msg.react(ctx, '👌').await?;

    let to1 = ChannelId(861751113916612648);
    let to2 = ChannelId(861751155000213544);
    
    for _ in 0..times {
        guild.move_member(&ctx.http, user_id, to1).await?;
        thread::sleep(time::Duration::from_millis(700));
        guild.move_member(&ctx.http, user_id, to2).await?;
        thread::sleep(time::Duration::from_millis(700));
    }
    guild.move_member(&ctx.http, user_id, channel_id).await?;

    Ok(())
}
