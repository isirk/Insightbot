mod cogs;
use cogs::{math::*};

use std::collections::HashSet;
use serenity::framework::standard::macros::help;
use serenity::framework::standard::{
    help_commands,
    Args,
    CommandGroup,
    CommandResult,
    HelpOptions,
    StandardFramework, macros::{command, group},
};
use serenity::model::channel::Message;
use serenity::model::gateway::{Ready, GatewayIntents};
use serenity::model::id::UserId;
use serenity::prelude::*;
use serenity::async_trait;

//Cogs and Commands
#[group]
#[commands(ping)]
struct General;

#[group]
#[commands(multiply)]
struct Math;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[help]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&MATH_GROUP);

    let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}