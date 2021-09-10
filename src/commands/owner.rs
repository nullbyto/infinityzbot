use serenity::{
    framework::standard::{
        macros::command,
        CommandResult,
        Args,
    },
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::{
    fs,
};

#[command]
#[description = "Deletes a sound."]
async fn delete(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let sound_name = match args.single::<String>() {
        Ok(sound) => sound,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "❌ Must provide a sound name").await?;
            return Ok(());
        }
    };
    let path = format!("./sounds/{}.mp3", sound_name);
    if let Err(_) = fs::remove_file(path) {
        msg.channel_id.say(ctx, "❌ Sound name doesn't exist!").await?;
    } else {
        msg.channel_id.say(ctx, format!("✅ **{}** was deleted!", sound_name)).await?;
    }
    
    Ok(())
}

#[command]
#[description = "Renames a sound name to a new name."]
#[usage = "[from] [to]"]
#[example = "zewzew zewzew1"]
async fn rename(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let sound_name = match args.single::<String>() {
        Ok(sound) => sound,
        Err(_) => {
            msg.channel_id.say(&ctx, "❌ Error! Must provide an existing sound name!").await?;
            return Ok(());
        }
    };
    let rest = args.rest().split_whitespace().collect::<Vec<_>>();
    if rest.is_empty(){
        msg.channel_id.say(&ctx, "❌ Error! Must provide a new sound name!").await?;
        return Ok(());
    }
    let from = format!("./sounds/{}.mp3", &sound_name);
    let to = format!("./sounds/{}.mp3", &rest[0]);
    if let Err(_) = fs::rename(&from, &to) {
        msg.channel_id.say(&ctx, "❌ Error! Sound doesn't exist!").await?;
    } else {
        msg.channel_id.say(&ctx, format!("✅ Sound name **{}** has been changed to **{}**.", &sound_name, &rest[0])).await?;
    }

    Ok(())
}

#[command]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    //let sys = System::new();
    let memory_usage = "".to_string();
    let cpu_usage = "".to_string();
    

    msg.channel_id.say(&ctx,
        format!("```= STATISTICS =\
        • Mem Usage  :: {}\n\
        • CPU Usage  :: {}\
        ```", memory_usage, cpu_usage)).await?;

    Ok(())
}