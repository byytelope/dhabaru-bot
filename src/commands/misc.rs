use poise::serenity_prelude::{futures::StreamExt, ActivityType, Mention, User};

use crate::{Context, PoiseError};

/// Get latency
/// May be inconsistent
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), PoiseError> {
    ctx.say(format!("🏓 {:?}ms", ctx.ping().await.as_millis()))
        .await?;

    Ok(())
}

/// Get activity status of a user
#[poise::command(slash_command)]
pub async fn activity(
    ctx: Context<'_>,
    #[description = "User to check status"] user: User,
) -> Result<(), PoiseError> {
    let res = match ctx.guild().unwrap().presences.get(&user.id) {
        Some(presences) => {
            println!("{:#?}", presences);
            let status = presences.status.name();
            let activity = match presences.activities.first() {
                Some(ac) => match ac.kind {
                    ActivityType::Playing => format!("playing `{}`", ac.name),
                    ActivityType::Streaming => format!("streaming `{}`", ac.name),
                    ActivityType::Listening => format!("listening to `{}`", ac.name),
                    ActivityType::Watching => format!("watching `{}`", ac.name),
                    ActivityType::Competing => format!("competing in `{}`", ac.name),
                    _ => "chillin".into(),
                },
                None => "chillin".into(),
            };

            format!("{} is **{}**, {}", Mention::from(user.id), status, activity)
        }
        None => "Please try again in a moment...".into(),
    };

    ctx.reply(res).await?;

    Ok(())
}

/// Delete messages from channel
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn clear(
    ctx: Context<'_>,
    #[description = "Minimum of 2, maximum of 100"]
    #[min = 2]
    #[max = 100]
    amount: u32,
) -> Result<(), PoiseError> {
    let mut messages = ctx.channel_id().messages_iter(&ctx).boxed();

    for _ in 0..amount {
        if let Some(message_res) = messages.next().await {
            match message_res {
                Ok(message_res) => message_res.delete(&ctx.http()).await?,
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    ctx.reply(format!("Deleted `{}` messages", amount)).await?;

    Ok(())
}
