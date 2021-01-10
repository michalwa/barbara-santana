use std::{error::Error, future::Future, pin::Pin};

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::channel::Message,
    framework::standard::StandardFramework,
    model::gateway::Ready,
};

use rsdis::{
    config::AppConfig,
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

    let db = config.database.into_client().await?;
    let store: Store<GuildSettings> = Store::new(db, "guilds");

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
        let mut data = ctx.data.write().await;

        if let Some(id) = msg.guild_id {
            let store = data.get_mut::<Store<GuildSettings>>().unwrap();
            store.get(id).await.map_or(None, |g| Some(g.prefix.clone()))
        } else {
            Some(data.get::<AppConfig>().unwrap().bot.default_prefix.clone())
        }
    })
}
