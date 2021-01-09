use std::{
    error::Error,
    future::Future,
    pin::Pin,
};
use serenity::{
    prelude::*,
    async_trait,
    model::channel::Message,
    framework::standard::StandardFramework,
    model::gateway::Ready,
};
use rsdis::{
    config::AppConfig,
    db::DbClient,
    store::{Store, guild::GuildSettings},
    commands::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot user {} connected", ready.user.tag());
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = AppConfig::load().expect("Could not load config");

    let store: Store<GuildSettings> = Store::new(
        DbClient::new(&config.database.uri, &config.database.name).await?,
        "guilds");

    let framework = StandardFramework::new()
        .configure(|c| c.dynamic_prefix(dynamic_prefix_handler))
        .help(&HELP)
        .group(&OWNER_GROUP);

    let mut client = Client::builder(&config.bot.token)
        .type_map_insert::<AppConfig>(config)
        .type_map_insert::<Store<GuildSettings>>(store)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Could not create client");

    if let Err(e) = client.start().await {
        eprintln!("Client exited with error: {:?}", e);
    }

    Ok(())
}

/// Callback to `Configuration::dynamic_prefix` that will query the store for the
/// guild-configured prefix or return the default prefix from the config
fn dynamic_prefix_handler<'f>(ctx: &'f Context, msg: &'f Message)
    -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'f>>
{
    Box::pin(async move {
        if let Some(id) = msg.guild_id {
            if let Some(store) = ctx.data.write().await.get_mut::<Store<GuildSettings>>() {
                return store.get(id).await
                    .map_or(None, |g| Some(g.prefix.clone()));
            }
        }
        ctx.data.read().await
            .get::<AppConfig>()
            .map(|c| c.bot.default_prefix.clone())
    })
}
