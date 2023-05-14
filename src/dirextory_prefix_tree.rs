use std::fs;
use std::io;

// When the user types a tab (\t), we want to autocomplete the current input buffer to the longest match in the working directory.
// When the user changes the working directory, the prefix tree instance is updated. 
// When a tab is entered, the longest match is found by searching the tree. 

pub struct DirPrefixTree {
    
}

impl DirPrefixTree {
    pub fn make() -> DirPrefixTree {
        DirPrefixTree {}
    }

    pub fn update_to_current_dir(&mut self) -> std::io::Result<()> {
        let paths = fs::read_dir("./")?; 

        

        Ok(())
    }
}