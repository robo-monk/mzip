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

    pub fn value(&self) -> char {
        return self.v.1
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

fn queue_to_tree(map: HashMap<char, u32>) -> Tree<(u32, char)> {

    let mut queue: Vec<(u32, Node<(u32, char)>)> = Vec::new();

    for (c, frequency) in map.iter() {
        println!("char: {c} freq: {frequency}");
        let node = Node::new((*frequency, *c));
        queue.push((*frequency, node));
    }

    queue.sort_by(|a, b| a.0.cmp(&b.0));

    for t in queue.iter() {
        println!("[({}) '{}']", t.1.frequency(), t.1.value());
    }
    return Tree::new();
}

fn encode(content: String) -> String {
    let chars = content.chars();

    let mut map: HashMap<char, u32> = HashMap::new();

    for c in chars {
        println!("{}", c);
        let mut frequency = 0;

        if map.contains_key(&c) {
            frequency = *map.get(&c).unwrap();
        }

        map.insert(c, frequency + 1);
        // Node::new((frequency, c));
    }

    let tree = queue_to_tree(map);

    return String::from("");
}
