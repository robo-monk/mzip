mod huffman;
use huffman::Tree;
// self::huffman::Tree
// use encode::encode;

fn main() {
    println!("Hello, world!");
    let compressed = encode(String::from("yooo what the fuck"));
    println!("{}", compressed);
}

fn encode(content: String) -> String {
    return String::from("");
}
