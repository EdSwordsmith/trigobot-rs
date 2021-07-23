pub mod commands;
pub mod env;
pub(crate) mod model;
mod network;

use self::model::Feed;

use std::collections::HashMap;
use std::fs;
use std::io::Result;

use bincode;

use serde::{Deserialize, Serialize};

use serenity::model::id::ChannelId;
use serenity::prelude::TypeMapKey;

#[derive(Deserialize, Serialize)]
pub struct State {
    feeds: HashMap<String, Feed>,
    category: Option<ChannelId>,
}

impl State {
    /// Create an empty state
    pub fn new() -> Self {
        State {
            feeds: HashMap::new(),
            category: None,
        }
    }

    pub(crate) fn get_feeds(&mut self) -> &mut HashMap<String, Feed> {
        &mut self.feeds
    }

    pub(crate) fn set_category(&mut self, category: ChannelId) {
        self.category = Some(category);
    }

    /// Save a list of feeds to a file
    pub fn save_to_file(file: &str, value: &State) -> Result<()> {
        fs::write(file, bincode::serialize(&value).unwrap())?;

        Ok(())
    }

    /// Load a list of feeds from a file
    pub fn load_from_file(file: &str) -> Result<State> {
        match bincode::deserialize(&fs::read(file)?) {
            Ok(val) => Ok(val),
            Err(e) => panic!("Invalid data!\nFeeds couldn't be loaded: {}", e),
        }
    }
}

// So State can be included in the global state of the bot
impl TypeMapKey for State {
    type Value = State;
}
