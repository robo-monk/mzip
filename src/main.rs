// mod huffman;
// use huffman::Tree;
// self::huffman::Tree
// use encode::encode;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let compressed = encode(String::from("yooo what the fuck"));
    println!("{}", compressed);
}

pub struct Tree<T> {
    root: Option<Box<Node<T>>>,
}

struct Node<T> {
    v: T,
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
}


impl Tree<(u32, char)> {
    fn new() -> Self {
        Tree { root: None }
    }
}

impl Node<(u32, char)> {
    fn new(v: (u32, char)) -> Self {
        Node {
            v,
            right: None,
            left: None
        }
    }

    pub fn frequency(&self) -> u32 {
        return self.v.0;
    }
}

/*
        Huffman Tree

             (4)
            /  \
    [(2) t]     (2)
                / \
        [(1) e]   [(1) s]


        for left = 0 and right = 1
        test = 0 10 11 0
        instead of 01110100 01100101 01110011 01110100

        1. Calculate the frequency of each character within a string
        -> [2 t] [1 e] [1 s]
        2. Sort in increasing order of frequency
        -> [1 e] [1 s] [2 t] (queue) 
        3. Create a new node Z. Left child of Z will be the minimum frequency, Right will be the 2nd min.
           Value of Z will be the sum of the 2 childs
        -> 2 'z'[left: [1 e] right:[1 s]] [2 t] 
        4. Insert Z into the tree

*/

fn encode(content: String) -> String {
    let chars = content.chars();

    let mut queue: HashMap<char, u32> = HashMap::new();

    for c in chars {
        println!("{}", c);
        let mut frequency = 0;

        if queue.contains_key(&c) {
            frequency = *queue.get(&c).unwrap();
        }

        queue.insert(c, frequency + 1);
        // Node::new((frequency, c));
    }

    for (key, val) in queue.iter() {
        println!("key: {key} val: {val}");
    }

    return String::from("");
}
