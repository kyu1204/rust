use std::collections::HashMap;


#[derive(Debug)]
struct Node {
    node: HashMap<char, Node>,
    is_end: bool
}

impl Node {
    fn new() -> Self {
        Node {
            node: HashMap::new(),
            is_end: false
        }
    }
}


#[derive(Debug)]
struct Trie {
    root: Node
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: Node::new()
        }
    }

    fn insert(&mut self, word: String) {
        let mut current = &mut self.root;
        for w in word.chars(){
            let next = current.node.entry(w).or_insert(Node::new());
            current = next;
        }
        if !current.is_end {
            current.is_end = true;
        }
    }

    fn search(&mut self, word: String) -> bool {
        match self.tracking(word) {
            None => {
                false
            }
            Some(current) => {
                if current.is_end {
                    true
                }
                else {
                    false
                }
            }
        }
    }

    fn starts_with(&mut self, word: String) -> bool {
        match self.tracking(word) {
            None => {
                false
            }
            Some(current) => {
                if !current.is_end {
                    !current.is_end
                }
                else {
                    current.is_end
                }
            }
        }
    }

    fn tracking(&mut self, word: String) -> Option<&Node> {
        let mut current = &self.root;
        for w in word.chars() {
            match current.node.get(&w) {
                None => {
                    return None
                }
                Some(next) => {
                    current = next;
                }
            }
        }
        Option::from(current)
    }
}


fn main() {
    let mut trie = Trie::new();
    trie.insert(String::from("test"));
    trie.insert(String::from("tesla"));

    assert_eq!(trie.search(String::from("test")), true);
    assert_eq!(trie.starts_with(String::from("tes")), true);
    assert_eq!(trie.starts_with(String::from("app")), false);
    assert_eq!(trie.search(String::from("tes")), false);
    assert_eq!(trie.search(String::from("tesla")), true);

    trie.insert(String::from("aesargorithm"));
    assert_eq!(trie.starts_with(String::from("aes")), true);
    assert_eq!(trie.search(String::from("aesargorithm")), true);
    assert_eq!(trie.search(String::from("asg")), false);
    assert_eq!(trie.starts_with(String::from("aar")), false);
}
