struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            is_end: false,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self { root: TrieNode::new() }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            node = node.children[idx]
                .get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            match &node.children[idx] {
                Some(next) => node = next,
                None => return false,
            }
        }
        node.is_end
    }
}
