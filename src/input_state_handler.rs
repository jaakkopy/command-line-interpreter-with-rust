
pub enum InputState {
    ESC,
    BRACK,
    FUNC,
    BACKSPACE,
    ANY,
}

pub struct InputStateHandler {
    state: InputState,    
}

// A state machine for handling user input
impl InputStateHandler {
    
    pub fn current_state(&self) -> &InputState {
        &self.state
    }

    pub fn reset_to_any(&mut self) {
        self.state = InputState::ANY; 
    }

    pub fn make() -> InputStateHandler {
        InputStateHandler { state: InputState::ANY }
    }

    fn advance_any(&mut self, c: char) {
        match c{
            '\x1b' => self.state = InputState::ESC,
            '\u{7f}' => self.state = InputState::BACKSPACE,
            _ => ()
        }
    }

    fn advance_backspace(&mut self, c: char) {
        match c{
            '\x1b' => self.state = InputState::ESC,
            '\u{7f}' => self.state = InputState::BACKSPACE,
            _ => self.state = InputState::ANY 
        }
    }

    fn advance_esc(&mut self, c: char) {
        match c{
            '\x5b' => self.state = InputState::BRACK,
            '\u{7f}' => self.state = InputState::BACKSPACE,
            _ => self.state = InputState::ANY 
        }
    }

    fn advance_brack(&mut self, c: char) {
        match c{
            '\x41'|'\x42'|'\x43'|'\x44' => self.state = InputState::FUNC,
            '\u{7f}' => self.state = InputState::BACKSPACE,
            _ => self.state = InputState::ANY 
        }

    }

    pub fn advance_state(&mut self, c: char) {
        match &self.state {
            InputState::ANY => self.advance_any(c),
            InputState::BACKSPACE => self.advance_backspace(c),
            InputState::ESC => self.advance_esc(c),
            InputState::BRACK => self.advance_brack(c),
            _ => self.state = InputState::ANY,
        }
    }

}
