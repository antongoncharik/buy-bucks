use scraper::{ElementRef, Html, Selector};

pub fn parse(html: String) {
    let document = Html::parse_document(&html);

    let currency_selector = Selector::parse("td.rates-table__currency").unwrap();
    let price_selector = Selector::parse("span.currency_value").unwrap();

    for currency_element in document.select(&currency_selector) {
        // Get the inner text of the currency element
        let currency_text = currency_element
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        if currency_text.contains("USD") {
            // Find the parent <tr> (assuming the prices are in the same row)
            if let Some(tr_element) = currency_element
                .ancestors()
                .find(|ancestor| ancestor.value().is_element())
            {
                // Convert NodeRef to ElementRef for further selection
                if let Some(tr_element_ref) = ElementRef::wrap(tr_element) {
                    // Find all price elements in the row
                    let prices: Vec<String> = tr_element_ref
                        .select(&price_selector)
                        .map(|e| e.inner_html())
                        .collect();

                    // Print out the prices (buy and sell)
                    if prices.len() == 2 {
                        println!("USD Buy Price: {}", prices[0]);
                        println!("USD Sell Price: {}", prices[1]);
                    }
                }
            }
        }
    }

    // Ok(())
}
