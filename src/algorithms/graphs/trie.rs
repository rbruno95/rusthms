#[allow(dead_code)]
pub struct TrieNode {
    pub children: Vec<Option<Box<TrieNode>>>,
    pub is_word: bool,
}

impl Clone for TrieNode {
    fn clone(&self) -> Self {
        TrieNode {
            children: self.children.clone(),
            is_word: self.is_word,
        }
    }
}

pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: TrieNode {
                children: vec![None; 26],
                is_word: false,
            },
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            let index = c as usize - 'a' as usize;
            if current_node.children[index].is_none() {
                current_node.children[index] = Some(Box::new(TrieNode {
                    children: vec![None; 26],
                    is_word: false,
                }));
            }
            current_node = current_node.children[index].as_mut().unwrap();
        }
        current_node.is_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        for c in word.chars() {
            let index = c as usize - 'a' as usize;
            if current_node.children[index].is_none() {
                return false;
            }
            current_node = current_node.children[index].as_ref().unwrap();
        }
        current_node.is_word
    }
}
