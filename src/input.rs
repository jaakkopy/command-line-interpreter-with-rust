use std::io::{self, Read, Write, stdout};
use crate::dirextory_prefix_tree::*;
pub use crate::history::*;
pub use crate::input_state_handler::*;


pub struct Input {
    input_buf: String,
    input_buf_index: usize,
    buf_len: usize,
    input_state: InputStateHandler, 
    history: InputHistory,
    prefix_tree: DirPrefixTree
}

impl Input {
    pub fn make() -> Input {
        Input {
            input_buf: String::new(),
            input_buf_index: 0,
            buf_len: 0,
            input_state: InputStateHandler::make(),
            history: InputHistory::make(50),
            prefix_tree: DirPrefixTree::make()
        }
    }

    // clear the row by writing the input buffer's length's worth of backspaces
    fn clear_input(&mut self) -> std::io::Result<()> {
        let ln = self.input_buf.len();
        self.erase_chars(ln)?;
        Ok(())
    }

    // "replace" the user given text with another by writing backspaces and printing the new buffer to stdout
    fn replace_buf(&mut self, replace: String) -> std::io::Result<()> {
        self.clear_input()?;
        self.buf_len = replace.len();
        self.input_buf_index = self.buf_len;
        self.input_buf = replace;
        print!("{}", &self.input_buf);
        stdout().flush()?;
        Ok(())
    }

    // Scroll up the history storage and restore the found text as the current text 
    pub fn arrow_up(&mut self) -> std::io::Result<()> {
        if let Some(from_history) = self.history.scroll_up() {
            let s = String::from(from_history);
            self.replace_buf(s)?;
        }
        Ok(())
    }

    // Scroll down the history storage and restore the found the text as the current text. If the position is already the earliest, replace the input buffer with an empty one.
    fn arrow_down(&mut self) -> std::io::Result<()> {
        if let Some(from_history) = self.history.scroll_down() {
            let s = String::from(from_history);
            self.replace_buf(s)?;
        } else {
            self.replace_buf(String::new())?;
        }
        Ok(())
    }

    // Move the cursor forward. Not implemented
    fn arrow_forward(&self) -> std::io::Result<()> {
        Ok(())
    }

    // Move the cursor backward. Not implemented
    fn arrow_backward(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    // Calls one of the "arrow functions"
    fn call_func(&mut self, c: char) -> std::io::Result<()> {
        match c {
            '\x41' => self.arrow_up()?,         
            '\x42' => self.arrow_down()?,       
            '\x43' => self.arrow_forward()?,    
            '\x44' => self.arrow_backward()?,   
            _ => ()
        }
        self.input_state.reset_to_any();
        Ok(())
    }

    fn add_to_buf(&mut self, c: char) -> std::io::Result<()> {
        print!("{}", c);
        stdout().flush()?;
        self.input_buf.push(c);
        self.buf_len += 1;
        self.input_buf_index += 1;
        return Ok(());
    }

    fn erase_chars(&mut self, amount: usize) -> std::io::Result<()> {
        let amnt = std::cmp::min(amount, self.input_buf.len());        
        for _ in 0..amnt {
            print!("{} {}", '\u{8}', '\u{8}'); 
            self.input_buf.pop();
            self.buf_len -= 1;
            self.input_buf_index -= 1;
        }
        stdout().flush()?; 
        Ok(())
    }

    pub fn store_to_history(&mut self, buf: String) {
        self.history.store(buf);
    }

    // autocomplete the input string to match the longest common beginning character sequences found in the working directory
    fn autocomplete_input_buf(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    pub fn update_prefix_tree(&mut self) {
        self.prefix_tree.update_to_current_dir(); 
    }

    pub fn handle_char(&mut self, c: char) -> std::io::Result<()> {
        if c == '\t' {

            return Ok(());
        }        


        self.input_state.advance_state(c);
        match &self.input_state.current_state() {
            InputState::ANY => self.add_to_buf(c)?,
            InputState::FUNC => self.call_func(c)?,
            InputState::BACKSPACE => self.erase_chars(1)?,
            _ => (),
        }
        Ok(())
    }

    fn print_prompt(&self) -> Result<(),std::io::Error> {
        print!("> ");
        io::stdout().flush()?;
        Ok(())
    }

    fn reset(&mut self) {
        self.input_buf.clear();
        self.buf_len = 0;
        self.input_buf_index = 0;
    }

    // Read user input byte by byte. Returns the input string after entering a newline
    pub fn read_input(&mut self) -> std::io::Result<String> {
        self.print_prompt()?;

        let mut buf =  vec![0; 1];
        let mut stdin = io::stdin();

        loop {
            stdin.read_exact(&mut buf)?; 
            let c = buf[0] as char;
            if c == '\n' {
                let s = String::from(&self.input_buf);
                self.store_to_history(s);
                break;
            }
            self.handle_char(c)?;
        }
        println!();

        let s = String::from(&self.input_buf);
        self.reset();
        Ok(s)
    }

}
