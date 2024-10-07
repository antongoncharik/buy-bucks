use reqwest;

pub fn get_html() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let res = client.get("https://bnb.by").send()?.text()?;

    Ok(res)
}
