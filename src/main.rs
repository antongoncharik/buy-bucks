mod bnb;
mod constants;
mod nbrb;

fn main() {
    let bnb_html = bnb::get_html().unwrap();
    let bnb_price = bnb::get_price(bnb_html).unwrap();
    let nbrb = nbrb::get_price().unwrap();

    println!("{} {}", bnb_price, nbrb)
}
