use serenity::prelude::Context;
use serenity::model::prelude::Message;
use serenity::framework::standard::{macros::command, Args, CommandResult};

#[command]
async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>()?;
    let two = args.single::<f64>()?;

    let product = one * two;

    msg.reply(ctx, product).await?;

    Ok(())
}