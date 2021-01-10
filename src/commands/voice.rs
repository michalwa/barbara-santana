use serenity::{
    client::Context,
    model::channel::Message,
    framework::standard::{ macros::*, * },
};

static BARBARA: &'static str = "https://www.youtube.com/watch?v=NZaCSX7pvmw";

#[group]
#[only_in(guilds)]
#[commands(speak, leave)]
struct Voice;

#[command]
async fn speak(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|s| s.channel_id);

    let connect_to = match channel_id {
        Some(id) => id,
        None => {
            msg.reply(ctx, "wejdz na kanal glosowy to pogadamy").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await.unwrap().clone();
    let (call_lock, joined) = manager.join(guild.id, connect_to).await;

    if let Err(_) = joined {
        msg.reply(ctx, "nie moge dolaczyc cos nie wyszlo przepraszam").await?;
    }

    let mut call = call_lock.lock().await;
    call.play_source(match songbird::ytdl(BARBARA).await {
        Ok(source) => source,
        Err(_) => {
            msg.reply(ctx, "nie moglam zaladowac nagrania przepraszam").await?;
            return Ok(());
        }
    });

    Ok(())
}

#[command]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    let manager = songbird::get(ctx).await.unwrap().clone();

    if manager.get(guild.id).is_some() {
        if let Err(_) = manager.remove(guild.id).await {
            msg.reply(ctx, "nie moge wyjsc pomocy").await?;
        }
    } else {
        msg.reply(ctx, "nic nie mowie o co ci chodzi").await?;
    }

    Ok(())
}
