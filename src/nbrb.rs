use chrono::{Datelike, Duration, Local, NaiveDate};
use reqwest;
use serde::{Deserialize, Serialize};

use crate::constants;

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
    let today = Local::now().date_naive();
    let first_day_of_current_month =
        NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
    let last_day_of_previous_month = first_day_of_current_month - Duration::days(1);

    let url = constants::NBRB_URL.to_string() + &last_day_of_previous_month.to_string();

    let client = reqwest::blocking::Client::builder().build()?;

    let res = client.get(url).send()?.text()?;

    let nbrn_response: NBRNResponse = serde_json::from_str(&res)?;

    Ok(nbrn_response.cur_official_rate)
}
