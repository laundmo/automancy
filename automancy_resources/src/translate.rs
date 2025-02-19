use std::ffi::OsStr;
use std::fs::{read_dir, read_to_string};
use std::path::Path;

use serde::Deserialize;

use automancy_defs::flexstr::{SharedStr, ToSharedStr};
use automancy_defs::hashbrown::HashMap;
use automancy_defs::id::{Id, IdRaw};
use automancy_defs::log;

use crate::{ResourceManager, JSON_EXT};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct TranslateJson {
    none: String,
    unnamed: String,
    items: HashMap<IdRaw, String>,
    tiles: HashMap<IdRaw, String>,
    scripts: HashMap<IdRaw, String>,
    gui: HashMap<IdRaw, String>,
    error: HashMap<IdRaw, String>,
}

#[derive(Debug, Default, Clone)]
pub struct Translate {
    pub none: SharedStr,
    pub unnamed: SharedStr,
    pub items: HashMap<Id, SharedStr>,
    pub tiles: HashMap<Id, SharedStr>,
    pub scripts: HashMap<Id, SharedStr>,
    pub gui: HashMap<Id, SharedStr>,
    pub error: HashMap<Id, SharedStr>,
}

impl ResourceManager {
    fn load_translate(&mut self, file: &Path) -> anyhow::Result<()> {
        log::info!("loading translate at: {file:?}");

        let translate: TranslateJson = serde_json::from_str(&read_to_string(file)?)?;

        let none = translate.none.to_shared_str();
        let unnamed = translate.unnamed.to_shared_str();

        let items = translate
            .items
            .into_iter()
            .map(|(id, str)| (id.to_id(&mut self.interner), str.into()))
            .collect();
        let tiles = translate
            .tiles
            .into_iter()
            .map(|(id, str)| (id.to_id(&mut self.interner), str.into()))
            .collect();
        let scripts = translate
            .scripts
            .into_iter()
            .map(|(id, str)| (id.to_id(&mut self.interner), str.into()))
            .collect();
        let gui = translate
            .gui
            .into_iter()
            .map(|(id, str)| (id.to_id(&mut self.interner), str.into()))
            .collect();
        let error = translate
            .error
            .into_iter()
            .map(|(id, str)| (id.to_id(&mut self.interner), str.into()))
            .collect();

        self.translates = Translate {
            none,
            unnamed,
            items,
            tiles,
            scripts,
            gui,
            error,
        };

        Ok(())
    }

    pub fn load_translates(&mut self, dir: &Path) -> anyhow::Result<()> {
        let translates = dir.join("translates");
        let translates = read_dir(translates);

        if let Ok(translates) = translates {
            for file in translates
                .into_iter()
                .flatten()
                .map(|v| v.path())
                .filter(|v| v.extension() == Some(OsStr::new(JSON_EXT)))
            {
                // TODO language selection
                if file.file_stem() == Some(OsStr::new("en_US")) {
                    self.load_translate(&file)?;
                }
            }
        }

        Ok(())
    }
}
