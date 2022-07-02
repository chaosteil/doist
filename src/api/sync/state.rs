#![allow(dead_code)]
use std::fs::{self, File};

use super::data;
use crate::config::Config;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub sync: data::State,
}

pub type TaskID = u64;

pub struct Task {
    pub id: TaskID,
    pub title: String,
    pub checked: bool,
    pub subtask: Vec<Task>,
}

impl Task {
    fn from_item(item: &data::Item) -> Task {
        Task {
            id: item.id,
            title: item.content.clone(),
            checked: item.checked,
            subtask: Vec::new(),
        }
    }
}

impl State {
    // TODO: share client for all calls?
    // There's a couple of ways you could do this:
    // Mark each of the fields with a skip_serialising_if attribute to say when to skip them. This is much easier, but you'll have to remember to do it for every field.
    // Write your own Serde serialiser that does this custom JSON form. This is more work, but shouldn't be too bad, especially given you can still use the stock JSON deserialiser.
    // Emptry structs aww
    pub async fn load(cfg: &Config) -> color_eyre::Result<State> {
        let token = cfg
            .token
            .as_ref()
            .ok_or_else(|| eyre!("no token provided"))?;
        let state = xdg::BaseDirectories::with_prefix("todoist")?.get_state_file("state.json");
        let s: serde_json::Value = if let Ok(file) = File::open(state) {
            let mut sync: serde_json::Value = serde_json::from_reader::<_, serde_json::Value>(file)
                .unwrap_or_default()
                .get("sync")
                .unwrap_or(&serde_json::Value::Object(serde_json::Map::new()))
                .to_owned();
            let sync_token = serde_json::from_value::<data::State>(sync.clone())
                .map_or(None, |s| Some(s.sync_token));
            let patch = State::sync_api(token, sync_token.as_deref()).await?;
            println!("PATCH: {}", serde_json::to_string_pretty(&patch)?);

            data::merge_sync_state(&mut sync, &patch);
            sync
        } else {
            State::sync_api(token, None).await?
        };
        let s: data::State = serde_json::from_value(s)?;
        Ok(State { sync: s })
    }

    pub fn save(&self) -> color_eyre::Result<()> {
        let state = xdg::BaseDirectories::with_prefix("todoist")?.place_state_file("state.json")?;
        let data = serde_json::to_string(self)?;
        fs::write(&state, &data)?;
        Ok(())
    }

    async fn sync_api(
        auth_token: &str,
        sync_token: Option<&str>,
    ) -> color_eyre::Result<serde_json::Value> {
        let client = reqwest::Client::new();
        let params = [
            ("sync_token", sync_token.unwrap_or("*")),
            ("resource_types", r#"["all"]"#),
        ];
        let res = client
            .post("https://api.todoist.com/sync/v8/sync")
            .bearer_auth(&auth_token)
            .form(&params)
            .send()
            .await?
            .bytes()
            .await?;
        Ok(serde_json::from_slice(&res)?)
    }
}
