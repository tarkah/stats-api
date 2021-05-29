use crate::*;
use async_std::task;
use chrono::NaiveDate;
use mockito::mock;

#[test]
fn test_teams() {
    task::block_on(async {
        let client = NhlClient::default();

        let _m = mock("GET", "/teams")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/nhl/teams.json")
            .create();

        let resp = client.get_teams().await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_team() {
    task::block_on(async {
        let client = NhlClient::default();

        let team_id = 1;
        let _m = mock("GET", "/teams/1")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/nhl/team.json")
            .create();

        let resp = client.get_team(team_id).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_schedule() {
    task::block_on(async {
        let client = NhlClient::default();
        let date = NaiveDate::from_ymd(2019, 12, 10);

        let _m = mock("GET", "/schedule?date=2019-12-10")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/nhl/schedule.json")
            .create();

        let resp = client.get_schedule_for(date).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_schedule_empty() {
    task::block_on(async {
        let client = NhlClient::default();
        let date = NaiveDate::from_ymd(2019, 12, 11);

        let _m = mock("GET", "/schedule?date=2019-12-11")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/nhl/schedule_empty.json")
            .create();

        let resp = client.get_schedule_for(date).await;
        assert!(&format!("{}", resp.err().unwrap()) == "No games for today.");
    });
}

#[test]
fn test_game_content_pre() {
    task::block_on(async {
        let client = NhlClient::default();

        let game_pk = 2_019_020_400;
        let _m = mock("GET", "/game/2019020400/content")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/nhl/game_content_pre_game.json")
            .create();

        let resp = client.get_game_content(game_pk).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_game_content_post() {
    task::block_on(async {
        let client = NhlClient::default();

        let game_pk = 2_019_020_401;
        let _m = mock("GET", "/game/2019020401/content")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/nhl/game_content_post_game.json")
            .create();

        let resp = client.get_game_content(game_pk).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_game_linescore_pre() {
    task::block_on(async {
        let client = NhlClient::default();

        let game_pk = 2_019_020_400;
        let _m = mock("GET", "/game/2019020400/linescore")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/nhl/game_linescore_pre_game.json")
            .create();

        let resp = client.get_game_linescore(game_pk).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}

#[test]
fn test_game_linescore_post() {
    task::block_on(async {
        let client = NhlClient::default();

        let game_pk = 2_019_020_401;
        let _m = mock("GET", "/game/2019020401/linescore")
            .with_status(200)
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("./src/tests/responses/nhl/game_linescore_post_game.json")
            .create();

        let resp = client.get_game_linescore(game_pk).await;
        assert!(resp.is_ok(), "{}", resp.err().unwrap());
    });
}
