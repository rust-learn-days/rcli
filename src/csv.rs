use std::fs;

use serde::{Deserialize, Serialize};

use crate::Csv2JsonOpts;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn csv2json(opts: Csv2JsonOpts) -> anyhow::Result<()> {
    let mut rdr = csv::Reader::from_path(opts.input)?;
    let mut ret = Vec::with_capacity(128);
    for result in rdr.deserialize() {
        let player: Player = result?;
        ret.push(player);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(opts.output, json)?;
    Ok(())
}
