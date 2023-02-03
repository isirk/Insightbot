use serenity::builder::{CreateEmbed, CreateEmbedAuthor};
use serenity::prelude::Context;
use serenity::model::prelude::Message;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::utils::Color;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let mut embed = CreateEmbed::default();
    embed.color(Color::BLURPLE);
    embed.title("About R.Daggy");
    embed.description("A basic Moderation bot witha few fun stuff designed for the DaggyTech server. More like a learning experiment. Built with \u{002764} using Rust!");
    embed.field("Urls", "[Invite Link](https://discordapp.com/api/oauth2/authorize?client_id=675589737372975124&permissions=378944&scope=bot)\n[Support Server](https://discord.gg/grGkdeS)\n[API](https://dagpi.xyz)\n[Website](https://dagbot.daggy.tech)\n[Source](https://github.com/Daggy1234/r.daggy)",true);
    embed.field(
        "Metadata",
        "Rustc: 1.47.0\nSerenity: 0.9.1\nBot: v0.1.0",
        true,
    );
    let mut auth = CreateEmbedAuthor::default();
    auth.name(&msg.author.name);
    auth.url(
        &msg.author
            .avatar_url()
            .unwrap_or(String::from(&msg.author.default_avatar_url())),
    );
    msg.channel_id
        .send_message(&ctx, |f| {
            f.content("").embed(|e| {
                e.0 = embed.0;
                e
            })
        })
        .await
        .unwrap();
    Ok(())
}