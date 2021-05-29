use crate::*;
use async_std::task;
use chrono::NaiveDate;
use mockito::{mock, Matcher};

#[test]
fn test_teams() {
    task::block_on(async {
        let client = MlbClient::default();

        let _m = mock("GET", "/teams?sportId=1")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/mlb/teams.json")
            .create();

        let resp = client.get_teams().await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_team() {
    task::block_on(async {
        let client = MlbClient::default();

        let team_id = 133;
        let _m = mock("GET", "/teams/133")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/mlb/team.json")
            .create();

        let resp = client.get_team(team_id).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_schedule() {
    task::block_on(async {
        let client = MlbClient::default();
        let date = NaiveDate::from_ymd(2019, 11, 10);

        let _m = mock("GET", "/schedule")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("date".into(), "2019-11-10".into()),
                Matcher::UrlEncoded("sportId".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/mlb/schedule.json")
            .create();

        let resp = client.get_schedule_for(date).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_schedule_empty() {
    task::block_on(async {
        let client = MlbClient::default();
        let date = NaiveDate::from_ymd(2019, 11, 11);

        let _m = mock("GET", "/schedule")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("date".into(), "2019-11-11".into()),
                Matcher::UrlEncoded("sportId".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/mlb/schedule_empty.json")
            .create();

        let resp = client.get_schedule_for(date).await;
        assert!(&format!("{}", resp.err().unwrap()) == "No games for today.");
    });
}

#[test]
fn test_game_content_pre() {
    task::block_on(async {
        let client = MlbClient::default();

        let game_pk = 530_428;
        let _m = mock("GET", "/game/530428/content")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/mlb/game_content_pre_game.json")
            .create();

        let resp = client.get_game_content(game_pk).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_game_content_post() {
    task::block_on(async {
        let client = MlbClient::default();

        let game_pk = 530_429;
        let _m = mock("GET", "/game/530429/content")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/mlb/game_content_post_game.json")
            .create();

        let resp = client.get_game_content(game_pk).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_game_linescore_pre() {
    task::block_on(async {
        let client = MlbClient::default();

        let game_pk = 530_428;
        let _m = mock("GET", "/game/530428/linescore")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/mlb/game_linescore_pre_game.json")
            .create();

        let resp = client.get_game_linescore(game_pk).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_game_linescore_post() {
    task::block_on(async {
        let client = MlbClient::default();

        let game_pk = 530_429;
        let _m = mock("GET", "/game/530429/linescore")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/mlb/game_linescore_post_game.json")
            .create();

        let resp = client.get_game_linescore(game_pk).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}
