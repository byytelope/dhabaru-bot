use poise::{
    serenity_prelude::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
    CreateReply,
};
use serde::Deserialize;

use crate::{Context, PoiseError};

#[derive(Debug, poise::ChoiceParameter)]
pub enum ServerRegion {
    #[name = "Asia Pacific"]
    AP,
    #[name = "Europe"]
    EU,
    #[name = "North America"]
    NA,
    #[name = "Latin America"]
    #[allow(clippy::upper_case_acronyms)]
    LATAM,
    #[name = "Brazil"]
    BR,
    #[name = "Korea"]
    KR,
}

impl From<ServerRegion> for String {
    fn from(val: ServerRegion) -> Self {
        match val {
            ServerRegion::AP => "ap".to_string(),
            ServerRegion::EU => "eu".to_string(),
            ServerRegion::NA => "na".to_string(),
            ServerRegion::LATAM => "latam".to_string(),
            ServerRegion::BR => "br".to_string(),
            ServerRegion::KR => "kr".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ValoApiRes {
    pub status: u16,
    pub errors: Option<Vec<ValoApiError>>,
    pub data: Option<ValoApiData>,
}

#[derive(Debug, Deserialize)]
pub struct ValoApiError {
    pub message: String,
    pub code: u16,
    pub details: String,
}

#[derive(Debug, Deserialize)]
pub struct ValoApiData {
    pub currenttier: u32,
    pub currenttierpatched: String,
    pub images: ValoApiImages,
    pub ranking_in_tier: u32,
    pub elo: u32,
    pub old: bool,
}

#[derive(Debug, Deserialize)]
pub struct ValoApiImages {
    pub large: String,
    pub small: String,
}

/// Check your Valorant rank
#[poise::command(slash_command)]
pub async fn vlr_rank(
    ctx: Context<'_>,
    #[description = "The region of your account"] server: ServerRegion,
    #[description = "The part of your username before the '#'"] name: String,
    #[description = "The part of your username after the '#'"] tag: String,
) -> Result<(), PoiseError> {
    let res = reqwest::get(format!(
        "https://api.henrikdev.xyz/valorant/v1/mmr/{}/{}/{}",
        String::from(server),
        name,
        tag
    ))
    .await
    .unwrap()
    .json::<ValoApiRes>()
    .await
    .unwrap();

    if let Some(data) = res.data {
        let embed = CreateEmbed::new()
            .author(CreateEmbedAuthor::new(format!("{}#{}", name, tag)))
            .footer(
                CreateEmbedFooter::new(format!("requested by: {}", ctx.author().name)).icon_url(
                    ctx.author()
                        .avatar_url()
                        .unwrap_or(ctx.author().default_avatar_url()),
                ),
            )
            .thumbnail(data.images.large)
            .field("Rank", data.currenttierpatched, true)
            .field("Progress", data.ranking_in_tier.to_string(), true);

        ctx.send(CreateReply::default().embed(embed)).await?;
    }

    if let Some(errors) = res.errors {
        for error in &errors {
            tracing::error!(
                r#"Error occurred while fetching from Valorant API: "{}""#,
                error.message
            );
        }

        let msg = match res.status {
            404 => "Player not found...",
            _ => "Please try again in a few minutes...",
        };

        ctx.send(CreateReply::default().content(msg).ephemeral(true))
            .await?;
    }

    Ok(())
}
