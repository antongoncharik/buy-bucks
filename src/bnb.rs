use reqwest;

use scraper::{ElementRef, Html, Selector};

use crate::constants;

pub async fn get_price() -> Result<f64, Box<dyn std::error::Error>> {
    let html = get_html().await?;

    let document = Html::parse_document(&html);

    let currency_selector = Selector::parse("td.rates-table__currency")?;
    let price_selector = Selector::parse("span.currency_value")?;

    let mut prices: Vec<String> = vec![];

    for currency_element in document.select(&currency_selector) {
        let currency_text = currency_element
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        if currency_text.contains("USD") {
            if let Some(tr_element) = currency_element
                .ancestors()
                .find(|ancestor| ancestor.value().is_element())
            {
                if let Some(tr_element_ref) = ElementRef::wrap(tr_element) {
                    let values: Vec<String> = tr_element_ref
                        .select(&price_selector)
                        .map(|e| e.inner_html())
                        .collect();
                    prices.extend(values);
                }
            }
        }
    }

    let price: f64 = prices[1].parse()?;

    Ok(price)
}

async fn get_html() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let res = client.get(constants::BNB_URL).send().await?.text().await?;

    Ok(res)
}
