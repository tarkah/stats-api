use crate::model::mlb::{
    GameContentResponse, GameLinescoreResponse, Response, ResponseType, Schedule, Team,
};
use failure::{bail, format_err, Error, ResultExt};
use futures::AsyncReadExt;
use isahc::{
    http::{self, Uri},
    AsyncBody, HttpClient, Request,
};
use std::collections::HashMap;

#[cfg(test)]
use mockito;

pub struct Client {
    client: HttpClient,
    base: String,
    sport: Sport,
}

impl Client {
    pub fn new() -> Self {
        Client::default()
    }

    fn get_url(&self, path: &str, params: Option<HashMap<&str, String>>) -> http::Uri {
        if let Some(params) = params {
            let params = serde_urlencoded::to_string(params).unwrap_or_else(|_| String::from(""));
            let uri = format!("{}/{}?{}", self.base, path, params);
            uri.parse::<Uri>().unwrap()
        } else {
            let uri = format!("{}/{}", self.base, path);
            uri.parse::<Uri>().unwrap()
        }
    }
    async fn get(&self, url: Uri, response_type: ResponseType) -> Result<Response, Error> {
        let request = Request::builder()
            .method("GET")
            .uri(url)
            .body(AsyncBody::empty())
            .unwrap();

        let res = self
            .client
            .send_async(request)
            .await
            .context("Failed to get request")?;

        let mut body = res.into_body();
        let mut bytes = Vec::new();
        body.read_to_end(&mut bytes).await?;

        let response = response_type.deserialize(&bytes)?;

        Ok(response)
    }

    pub async fn get_teams(&self) -> Result<Vec<Team>, Error> {
        let mut modifiers = HashMap::new();
        modifiers.insert("sportId", String::from(&self.sport));

        let url = self.get_url("teams", Some(modifiers));
        let response_type = ResponseType::TeamsResponse;

        let _response = self.get(url, response_type).await?;

        if let Response::TeamsResponse(response) = _response {
            return Ok(response.teams);
        }
        bail!("Failed to get teams response.")
    }

    /// Get all teams, regardless of sportId
    ///
    /// Usefull during Exhibition games since an MLB team can
    /// face off against a college team
    pub async fn get_all_teams(&self) -> Result<Vec<Team>, Error> {
        let url = self.get_url("teams", None);
        let response_type = ResponseType::TeamsResponse;

        let _response = self.get(url, response_type).await?;

        if let Response::TeamsResponse(response) = _response {
            return Ok(response.teams);
        }
        bail!("Failed to get teams response.")
    }

    pub async fn get_team(&self, team_id: u32) -> Result<Team, Error> {
        let url = self.get_url(&format!("teams/{}", team_id), None);
        let response_type = ResponseType::TeamsResponse;

        let _response = self.get(url, response_type).await?;

        if let Response::TeamsResponse(mut response) = _response {
            let team = response
                .teams
                .pop()
                .ok_or_else(|| format_err!("Failed to get team response."))?;
            return Ok(team);
        }
        bail!("Failed to get team response.")
    }

    pub async fn get_todays_schedule(&self) -> Result<Schedule, Error> {
        let mut modifiers = HashMap::new();
        modifiers.insert("sportId", String::from(&self.sport));

        let url = self.get_url("schedule", Some(modifiers));
        let response_type = ResponseType::ScheduleResponse;

        let _response = self.get(url, response_type).await?;

        if let Response::ScheudleResponse(mut response) = _response {
            let schedule = response
                .dates
                .pop()
                .ok_or_else(|| format_err!("No games for today."))?;
            return Ok(schedule);
        }
        bail!("Failed to get schedule response.")
    }

    pub async fn get_schedule_for(&self, date: chrono::NaiveDate) -> Result<Schedule, Error> {
        let mut modifiers = HashMap::new();
        modifiers.insert("date", date.format("%Y-%m-%d").to_string());
        modifiers.insert("sportId", String::from(&self.sport));

        let url = self.get_url("schedule", Some(modifiers));
        let response_type = ResponseType::ScheduleResponse;

        let _response = self.get(url, response_type).await?;

        if let Response::ScheudleResponse(mut response) = _response {
            let schedule = response
                .dates
                .pop()
                .ok_or_else(|| format_err!("No games for today."))?;
            return Ok(schedule);
        }
        bail!("Failed to get schedule response.")
    }

    pub async fn get_game_content(&self, game_pk: u64) -> Result<GameContentResponse, Error> {
        let url = self.get_url(&format!("game/{}/content", game_pk), None);
        let response_type = ResponseType::GameContentResponse;

        let _response = self.get(url, response_type).await?;

        if let Response::GameContentResponse(response) = _response {
            return Ok(response);
        }
        bail!("Failed to get game content.")
    }

    pub async fn get_game_linescore(&self, game_pk: u64) -> Result<GameLinescoreResponse, Error> {
        let url = self.get_url(&format!("game/{}/linescore", game_pk), None);
        let response_type = ResponseType::GameLinescoreResponse;

        let _response = self.get(url, response_type).await?;

        if let Response::GameLinescoreResponse(response) = _response {
            return Ok(response);
        }
        bail!("Failed to get game content.")
    }
}

impl Default for Client {
    /// Returns client for Sport::Mlb
    fn default() -> Self {
        let client = HttpClient::builder()
            .max_connections_per_host(6)
            .build()
            .unwrap();
        let sport = Sport::Mlb;

        #[cfg(not(test))]
        let base = String::from("https://statsapi.mlb.com/api/v1");

        #[cfg(test)]
        let base = mockito::server_url();

        Client {
            client,
            base,
            sport,
        }
    }
}

#[derive(Clone)]
pub enum Sport {
    Mlb,
    Aaa,
    Aax,
    Afa,
    Afx,
    Asx,
    Roa,
    Rok,
    Win,
    Bbl,
    Min,
    Ind,
    Jml,
    Int,
    Nat,
    Nae,
    Nav,
    Nas,
    Naw,
    Bbc,
    Hsb,
}

impl From<&Sport> for String {
    fn from(sport: &Sport) -> String {
        let s = match sport {
            Sport::Mlb => "1",
            Sport::Aaa => "11",
            Sport::Aax => "12",
            Sport::Afa => "13",
            Sport::Afx => "14",
            Sport::Asx => "15",
            Sport::Roa => "5442",
            Sport::Rok => "16",
            Sport::Win => "17",
            Sport::Bbl => "8",
            Sport::Min => "21",
            Sport::Ind => "23",
            Sport::Jml => "31",
            Sport::Int => "51",
            Sport::Nat => "508",
            Sport::Nae => "509",
            Sport::Nav => "600",
            Sport::Nas => "510",
            Sport::Naw => "512",
            Sport::Bbc => "22",
            Sport::Hsb => "586",
        };
        String::from(s)
    }
}
