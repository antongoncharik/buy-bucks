use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct NBRNResponse {
    #[serde(rename = "Cur_ID")]
    cur_id: i32,

    #[serde(rename = "Date")]
    date: String,

    #[serde(rename = "Cur_Abbreviation")]
    cur_abbreviation: String,

    #[serde(rename = "Cur_Scale")]
    cur_scale: i32,

    #[serde(rename = "Cur_Name")]
    cur_name: String,

    #[serde(rename = "Cur_OfficialRate")]
    cur_official_rate: f64,
}

pub fn get_price() -> Result<f64, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder().build()?;

    let res = client
        .get("https://api.nbrb.by/exrates/rates/USD?parammode=2&ondate=2024-09-30")
        .send()?
        .text()?;

    let nbrn_response: NBRNResponse = serde_json::from_str(&res)?;

    Ok(nbrn_response.cur_official_rate)
}
