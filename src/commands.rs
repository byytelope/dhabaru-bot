use poise::serenity_prelude::{ActivityType, Mention, User};

use crate::{Context, Error};

/// Get latency
/// May be inconsistent
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("🏓 {:?}ms", ctx.ping().await.as_millis()))
        .await
        .unwrap();

    Ok(())
}

/// Get activity status of a user
#[poise::command(slash_command)]
pub async fn activity(
    ctx: Context<'_>,
    #[description = "User to check status"] user: User,
) -> Result<(), Error> {
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

    ctx.reply(res).await.unwrap();

    Ok(())
}
