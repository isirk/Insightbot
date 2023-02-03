mod cogs;
use cogs::{math::*, meta::*, dev::*};
use serenity::http::Http;
use serenity::model::prelude::Activity;

use std::collections::HashSet;
use serenity::framework::standard::macros::help;
use serenity::framework::standard::{
    help_commands,
    Args,
    CommandGroup,
    CommandResult,
    HelpOptions,
    StandardFramework, macros::group,
};
use serenity::model::channel::Message;
use serenity::model::gateway::{Ready, GatewayIntents};
use serenity::model::id::UserId;
use serenity::prelude::*;
use serenity::async_trait;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        const URL: &str = "https://twitch.tv/isirk";
        ctx.set_activity(Activity::streaming("!help", URL)).await;
        println!("{} is connected!", ready.user.name);
    }
}

#[help]
#[command_not_found_text = "Could not find: `{}`."]
#[embed_success_colour(BLURPLE)]
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
    let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let http = Http::new(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                    .with_whitespace(true)
                    .on_mention(Some(bot_id))
                    .prefix("!")
                    .delimiters(vec![", ", ","])
                    .owners(owners))
                    .help(&MY_HELP)
                    .group(&META_GROUP)
                    .group(&DEV_GROUP)
                    .group(&MATH_GROUP);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

//Cogs and Commands
#[group]
#[commands(ping, about)]
struct Meta;

#[group]
#[commands(multiply)]
struct Math;

#[group]
#[owners_only]
#[commands(status)]
struct Dev;