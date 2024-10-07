mod loader;
mod parser;

fn main() {
    let html = loader::get_html().unwrap();
    parser::parse(html)
}
