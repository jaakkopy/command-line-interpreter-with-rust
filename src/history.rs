use std::collections::VecDeque;

pub struct InputHistory {
    looking_at_index: usize,
    max_cap: usize,
    elements: usize,
    history: VecDeque<String>

}

impl InputHistory {
    pub fn make(max_capacity: usize) -> InputHistory {
        InputHistory { looking_at_index: 0, max_cap: max_capacity, elements: 0,  history: VecDeque::new() }
    }

    pub fn store(&mut self, input: String) {
        self.looking_at_index = 0;
        if self.elements + 1 >= self.max_cap {
            self.history.pop_back();
            self.elements -= 1;
        }
        self.history.push_front(input);
        self.elements += 1;
    }

    pub fn scroll_up(&mut self) -> Option<&String> {
        if self.looking_at_index >= self.history.len() {
            return None;
        } 
        self.looking_at_index += 1;
        self.history.get(self.looking_at_index - 1)
    }

    pub fn scroll_down(&mut self) -> Option<&String> {
        if self.looking_at_index == 0 {
            return None;
        } 
        self.looking_at_index -= 1;
        self.history.get(self.looking_at_index + 1)
    }


}
