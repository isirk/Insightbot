use serenity::prelude::Context;
use serenity::model::prelude::Message;
use serenity::framework::standard::{macros::command, Args, CommandResult};

#[command]
async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    msg.reply(ctx, first*second).await?;

    Ok(())
}