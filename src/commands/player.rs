use serenity::{
    Result as SerenityResult,
    framework::standard::{
        macros::command,
        CommandResult,
        Args,
    },
    utils::{Colour,}
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::{
    path::Path,
    fs
};
use std::{io, fs::File};

#[command]
#[only_in(guilds)]
#[aliases("j")]
#[description = "Joins your voice channel."]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    
    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice| voice.channel_id);
    
    let connect_to = match channel_id {
        Some(ch) => ch,
        None => {
            msg.reply(ctx, "❌ You are not in a voice channel!").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.");
    
    let _handler = manager.join(guild.id, connect_to).await;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("dc")]
#[description = "Leaves the voice channel."]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.");
    
    let handler = manager.remove(guild_id).await;
    if handler.is_err() {
        msg.reply(&ctx.http, "❌ I'm not in a voice channel!").await?;
    }
    msg.react(ctx, '👋').await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("p")]
#[description = "Play sounds in your voice channel. Use the `list` command to show all sounds."]
#[usage = "[soundname]"]
#[example = "zewzew"]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let sound_name = match args.single::<String>() {
        Ok(sound) => sound,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "❌ Must provide a sound name").await?;
            return Ok(());
        }
    };
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let path = Path::new(format!("./sounds/{}.mp3", sound_name).as_str()).to_owned();

    if !path.exists() {
        msg.reply(ctx, "❌ This sound name doesn't exist!").await?;
        return Ok(());
    }

    let manager = songbird::get(ctx).await
        .expect("Songbird voice client placed in at initialisation.").clone();
    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice| voice.channel_id);
    let connect_to = match channel_id {
        Some(ch) => ch,
        None => {
            msg.reply(ctx, "❌ You are not in a voice channel!").await?;
            return Ok(());
        }
    };
    let _handler = manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ffmpeg(path).await {
            Ok(src) => src,
            Err(why) => {
                println!("Err starting source: {:?}", why);
                msg.channel_id.say(&ctx.http, "❌ Error sourcing ffmpeg").await?;
                return Ok(());
            }
        };

        handler.stop();
        handler.play_source(source);

    }
    msg.react(&ctx.http, '▶').await?;
    msg.react(&ctx.http, '⏹').await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("s")]
pub async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        handler.stop();

    }

    Ok(())
}

#[command]
#[description = "Lists all sound names."]
#[aliases("l")]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    // use global variable MUTEX
    // use std::sync::Mutex;
    // static MY_VEC: Mutex<Vec<String>> = Mutex::new(vec![]); 
    let mut list_vec: Vec<String> = vec![];
    let paths = fs::read_dir("./sounds/").unwrap();
    for path in paths {
        let mut p = path.unwrap().path().to_str().unwrap().to_string();
        p = p.strip_suffix(".mp3").unwrap().to_string();
        p = p.strip_prefix("./sounds/").unwrap().to_string();
        list_vec.push(p);
    }

    list_vec.sort();
    let mut list = String::new();
    for chunk in list_vec.chunks(25) {
        list = format!("{}{}{}",
        list,
        chunk.join(", "),
        "\n---------------------------------------\
        ------------------------------------------------------\n");
    }

    let _msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("..:: Sound List ::..");
            e.description("All sounds are listed here: ");
            e.color(Colour::from(0xF83AAA));
            e.timestamp(chrono::Local::now());
            e.description(list);
            e
        });
        m
    }).await;
    
    Ok(())
}

#[command]
#[aliases("u")]
#[description = "Uploads the attached file. You have to attach the file and write the *command* as **comment**"]
#[usage = "[sound name]"]
async fn upload(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let file = match msg.attachments.first() {
        Some(a) => a,
        None => {
                msg.channel_id.say(&ctx.http, "❌ **You have to attach a file and use `upload` command as comment**").await?;
                return Ok(());
            }
    };
    if !file.filename.ends_with(".mp3") {
        msg.channel_id.say(&ctx.http, "❌ **The uploaded file must be an mp3 file!**").await?;
        return Ok(());
    }
    let sound_name = match args.single::<String>() {
        Ok(sound) => sound,
        Err(_) => {
            file.filename.split(".").collect::<Vec<_>>()[0].to_string()
        }
    };

    let resp = reqwest::get(&file.url).await.expect("Request failed").bytes().await.unwrap();
    let mut out = File::create(format!("./sounds/{}.mp3", &sound_name)).expect("Failed to create file");
    io::copy(&mut resp.as_ref(), &mut out).expect("Failed to copy file");

    if let Ok(_) = ctx.http.get_channel(msg.channel_id.0).await {
        msg.delete(&ctx.http).await?;
    }

    msg.channel_id.say(&ctx.http, format!("✅ **The sound `{}` has been uploaded you can find it in `list`**", &sound_name)).await?;

    Ok(())
}

#[allow(unused)]
/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
