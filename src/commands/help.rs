use std::collections::HashSet;
use serenity::{
    client::Context,
    model::{ id::UserId, channel::Message },
    framework::standard::{ macros::*, * },
};

#[help]
async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    opts: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, opts, groups, owners).await;
    Ok(())
}
