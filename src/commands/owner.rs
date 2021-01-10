use crate::store::{Store, guild::GuildSettings};
use serenity::{
    client::Context,
    model::channel::Message,
    framework::standard::{ macros::*, * },
};

#[group]
#[checks(Owner)]
#[commands(ping, prefix)]
struct Owner;

#[check]
#[name = "Owner"]
async fn check_owner(ctx: &Context, msg: &Message) -> Result<(), Reason> {
    match msg.guild(ctx).await {
        Some(g) if g.owner_id == msg.author.id => Ok(()),
        Some(_) => Err(Reason::User("User is not guild owner".into())),
        None => Err(Reason::User("Owner check only valid in guilds".into())),
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn prefix(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Some(id) = msg.guild_id {
        let new_prefix: String = args.parse()?;

        let mut data = ctx.data.write().await;
        let store = data.get_mut::<Store<GuildSettings>>().unwrap();
        store.with_mut(id, |g| g.prefix = new_prefix.clone())?;

        msg.reply(ctx, format!("Server prefix changed to {}", new_prefix)).await?;
    }

    Ok(())
}
