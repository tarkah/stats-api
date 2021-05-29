use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamsResponse {
    #[serde(default)]
    pub teams: Vec<Team>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub link: String,
    #[serde(default)]
    pub abbreviation: String,
    #[serde(default)]
    pub team_name: String,
    #[serde(default)]
    pub location_name: String,
    pub first_year_of_play: Option<String>,
    pub short_name: Option<String>,
    pub official_site_url: Option<String>,
    #[serde(default)]
    pub franchise_id: u32,
    #[serde(default)]
    pub active: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleResponse {
    #[serde(default)]
    pub dates: Vec<Schedule>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub date: NaiveDate,
    #[serde(default)]
    pub games: Vec<ScheduleGame>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleGame {
    pub game_pk: u64,
    #[serde(default)]
    pub link: String,
    #[serde(rename(deserialize = "gameDate"))]
    pub date: DateTime<Utc>,
    #[serde(default)]
    pub game_type: String,
    #[serde(default)]
    pub season: String,
    pub teams: ScheduleGameTeams,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleGameTeams {
    pub away: ScheduleGameTeam,
    pub home: ScheduleGameTeam,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleGameTeam {
    pub score: Option<u8>,
    #[serde(rename(deserialize = "team"))]
    pub detail: ScheduleGameTeamDetail,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleGameTeamDetail {
    pub id: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub link: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentResponse {
    pub editorial: GameContentEditorial,
    pub media: GameContentMedia,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentMedia {
    pub milestones: GameContentMilestones,
    pub epg: Vec<GameContentEpg>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentEpg {
    #[serde(default)]
    pub title: String,
    #[serde(deserialize_with = "fail_as_none")]
    pub items: Option<Vec<GameContentEpgItem>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentEpgItem {
    #[serde(default)]
    pub media_feed_type: String,
    #[serde(default)]
    pub call_letters: String,
    #[serde(default)]
    pub media_state: String,
    #[serde(default)]
    pub media_playback_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentEditorial {
    pub preview: GameContentEditorialItem,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentEditorialItem {
    #[serde(default)]
    pub title: String,
    pub items: Option<Vec<GameContentEditorialItemArticle>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentEditorialItemArticle {
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub headline: String,
    #[serde(default)]
    pub subhead: String,
    #[serde(default)]
    pub seo_title: String,
    #[serde(default)]
    pub seo_description: String,
    #[serde(deserialize_with = "fail_as_none")]
    pub media: Option<GameContentArticleMedia>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentArticleMedia {
    #[serde(default)]
    pub r#type: String,
    pub image: GameContentArticleMediaImage,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentArticleMediaImage {
    pub cuts: GameContentArticleMediaImageCut,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentArticleMediaImageCut {
    #[serde(rename(deserialize = "2208x1242"))]
    pub cut_2208_1242: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "2048x1152"))]
    pub cut_2048_1152: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "1704x960"))]
    pub cut_1704_960: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "1536x864"))]
    pub cut_1536_864: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "1284x722"))]
    pub cut_1284_722: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "1136x640"))]
    pub cut_1136_640: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "1024x576"))]
    pub cut_1024_576: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "960x540"))]
    pub cut_960_540: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "768x432"))]
    pub cut_768_432: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "640x360"))]
    pub cut_640_360: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "568x320"))]
    pub cut_568_320: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "372x210"))]
    pub cut_372_210: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "320x180"))]
    pub cut_320_180: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "248x140"))]
    pub cut_248_140: GameContentArticleMediaImageCutDetail,
    #[serde(rename(deserialize = "124x70"))]
    pub cut_124_70: GameContentArticleMediaImageCutDetail,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentArticleMediaImageCutDetail {
    #[serde(default)]
    pub aspect_ratio: String,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
    #[serde(default)]
    pub src: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentMilestones {
    pub stream_start: Option<DateTime<Utc>>,
    pub items: Option<Vec<GameContentMilestoneItem>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentMilestoneItem {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub period: String,
    #[serde(default)]
    pub period_time: String,
    #[serde(default)]
    pub ordinal_num: String,
    #[serde(default)]
    pub team_id: String,
    #[serde(default)]
    pub stats_event_id: String,
    #[serde(deserialize_with = "fail_as_none")]
    pub highlight: Option<GameContentMilestoneItemHighlight>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentMilestoneItemHighlight {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub blurb: String,
    #[serde(default)]
    pub description: String,
    pub playbacks: Option<Vec<GameContentMilestoneItemHighlightPlayback>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameContentMilestoneItemHighlightPlayback {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameLinescoreResponse {
    #[serde(default)]
    pub current_period: u8,
    pub teams: GameLinescoreTeams,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameLinescoreTeams {
    pub home: GameLinescoreTeam,
    pub away: GameLinescoreTeam,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameLinescoreTeam {
    #[serde(rename(deserialize = "team"))]
    pub detail: GameLinescoreTeamDetail,
    #[serde(default)]
    pub goals: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameLinescoreTeamDetail {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub link: String,
}

fn fail_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let result = T::deserialize(de);
    match result {
        Ok(t) => Ok(Some(t)),
        Err(_) => Ok(None),
    }
}
