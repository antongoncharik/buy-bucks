mod bnb;
mod constants;
mod nbrb;

fn main() {
    let bnb_price = bnb::get_price().unwrap();
    let nbrb = nbrb::get_price().unwrap();

    println!("{} {}", bnb_price, nbrb)
}
