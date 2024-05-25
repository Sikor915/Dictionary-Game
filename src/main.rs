mod game_logic;
mod dictionary;

fn main() {
    let json = "json";
    let dictionary = dictionary::json_dictionary::JsonDictionary::new(json);
    println!("Hello, world!");
}
