use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn bruh(ctx: Context<'_>, text: Option<String>) -> Result<(), Error> {
    let res = if let Some(content) = text {
        format!("Your text: {}", content)
    } else {
        "You have not provided an input".into()
    };

    ctx.say(res).await?;
    Ok(())
}
