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

impl From<Node<(u32, char)>> for Option<Box<Node<(u32, char)>>> {
    fn from(node: Node<(u32, char)>) -> Self {
        Some(Box::new(node))
    }

    
}

impl Tree<(u32, char)> {
    fn new(root: Node<(u32, char)>) -> Self {
        Tree { root: root.into() }
    }
}

impl Node<(u32, char)> {
    fn new(v: (u32, char)) -> Self {
        Node {
            v,
            right: None,
            left: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.right.is_none() && self.left.is_none()
    }

    fn serialize_debug(&self) {
        println!("<node data=\"[{}, '{}']\">", self.v.0, self.v.1);
        if !self.is_leaf() {
            self.left.as_ref().unwrap().serialize_debug();
            // print!("  ]        [ ");
            self.right.as_ref().unwrap().serialize_debug();
            // print!("] \n");
        }

        print!("<node/>");
    }

    pub fn value(&self) -> char {
        return self.v.1;
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
        3. Make each unique character a leaf node
        3. Create a new node Z. Left child of Z will be the minimum frequency, Right will be the 2nd min.
           Value of Z will be the sum of the 2 childs
        -> 2 'z'[left: [1 e] right:[1 s]] [2 t]
        4. Insert Z into the tree

*/

fn queue_to_tree(mut queue: Vec<Node<(u32, char)>>) -> Tree<(u32, char)> {
    if queue.len() == 1 {
        return Tree::new(queue.pop().unwrap());
    }

    queue.sort_by(|a, b| b.frequency().cmp(&a.frequency()));

    let left = queue.pop().unwrap();
    let right = queue.pop().unwrap();
    let combined_frequency =  left.frequency() + right.frequency();
    let new_node: Node<(u32, char)> = Node {
        v: (combined_frequency, char::from(1)), // refactor so char could be none
        right: right.into(),
        left: left.into()
     };

    queue.push(new_node);

    return queue_to_tree(queue);
}

// converts a HashMap of (char, frequency) to a Vec of (freq, Node)
fn map_to_queue(map: HashMap<char, u32>) -> Vec<Node<(u32, char)>> {
    // let mut queue: Vec<(u32, Node<(u32, char)>)> = Vec::new();
    let mut queue: Vec<Node<(u32, char)>> = Vec::new();
    // let mut queue: Vec<(char, u32)> = map.values().into();
    for (c, frequency) in map.iter() {
        println!("char: {c} freq: {frequency}");
        queue.push(Node::new((*frequency, *c)));
    }

    queue
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

    let queue = map_to_queue(map);
    let tree = queue_to_tree(queue);

    tree.root.unwrap().serialize_debug();

    return String::from("");
}
