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
    pub puuid: String,
    pub current_data: ValoApiCurrData,
}

#[derive(Debug, Deserialize)]
pub struct ValoApiCurrData {
    pub currenttier: u32,
    pub currenttierpatched: String,
    pub images: ValoApiImages,
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
        "https://api.henrikdev.xyz/valorant/v2/mmr/{}/{}/{}",
        String::from(server),
        name,
        tag
    ))
    .await
    .unwrap()
    .json::<ValoApiRes>()
    .await
    .unwrap();

    println!("{:#?}", res);

    ctx.say("Bruvc").await?;
    Ok(())
}
