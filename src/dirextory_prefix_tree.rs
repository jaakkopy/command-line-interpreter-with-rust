use std::fs;
use std::collections::HashMap;

// When the user types a tab (\t), we want to autocomplete the cursor's part of the input to the longest match in the working directory.
// When the user changes the working directory, the prefix tree instance is updated. 
// When a tab is entered, the longest match is found by searching the tree. 

struct TreeNode {
    children: HashMap<char, TreeNode>
}

impl TreeNode {
    pub fn make() -> TreeNode {
        TreeNode { children: HashMap::new() }
    }

    pub fn create_child(&mut self, key: char) {
        self.children.insert(key, TreeNode::make());
    }

    pub fn child_nodes_count(&self) -> usize {
        self.children.len()
    }

    pub fn get_child(&self, key: char) -> Option<&TreeNode> {
        self.children.get(&key)
    }

    pub fn get_keys(&self) -> std::collections::hash_map::Keys<'_, char, TreeNode> { 
        self.children.keys()
    }
}

pub struct DirPrefixTree {
    root: TreeNode
}

impl DirPrefixTree {
    pub fn make() -> std::io::Result<DirPrefixTree> {
        let mut dpt = DirPrefixTree {root: TreeNode::make() };
        dpt.update_to_current_dir()?;
        Ok(dpt)
    }

    pub fn insert(&mut self, value: &str) {
        let mut node = &mut self.root;
        for c in value.chars() {
            if !node.children.contains_key(&c) {
                node.create_child(c);
            } 
            node = node.children.get_mut(&c).unwrap();
        }
    }

    // Advance the tree according to the input string's chars as far as we can get. Return the longer option between the match and the original input
    pub fn find_longest_match(&self, value: &str) -> String {
        let mut buf = String::new();
        let mut node = &self.root;

        // first advance the tree by using the input characters as keys 
        for key in value.chars() {
            if let Some(next_node) = node.get_child(key) {
                buf.push(key);
                node = next_node;
            }
        } 

        // no corresponding path was found
        if buf.len() < value.len() {
            return String::from(value);
        }

        // advance the match by picking only the paths where the nodes have one just one child node (just one possible completion)
        while node.child_nodes_count() == 1 {
            if let Some(key) = node.get_keys().next() { 
                buf.push(*key);
                if let Some(next_node) = node.get_child(*key) {
                    node = next_node;
                } else {
                    return buf;
                }
            }
        }
        
        buf
    }

    pub fn update_to_current_dir(&mut self) -> std::io::Result<()> {
        let paths = fs::read_dir("./")?; 

        for p in paths {
            if let Ok(entry) = p {
                if let Some(file_name) = entry.file_name().to_str() {
                    self.insert(file_name);
                }
            }
        }

        Ok(())
    }
}