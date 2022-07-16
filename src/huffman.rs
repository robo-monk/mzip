pub struct Tree<T> {
    root: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
}

impl Tree<String> {
    fn new() -> Self {
        Tree { root: None }
    }
}

impl Node<String> {
    fn new(value: String) -> Self {
        Node {
            value,
            right: None,
            left: None
        }
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
