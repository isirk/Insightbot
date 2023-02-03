use serenity::{framework::standard::{macros::command, Args, CommandResult}, model::prelude::{Message, Activity}, prelude::Context};


#[command]
async fn status(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    const URL: &str = "https://twitch.tv/isirk";
    let name = args.message();
    ctx.set_activity(Activity::streaming(name, URL)).await;
    msg.reply(ctx, format!("Set activity to **{}**", args.message())).await?;
    Ok(())
}