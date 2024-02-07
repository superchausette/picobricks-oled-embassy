const MAX_LINE_SIZE: u8 = 15;

fn next_ascii_char(current: u8) -> u8 {
    match current {
        b'a'..=b'y' => current +1,
        b'z' => b'A',
        b'A'..=b'Y' => current +1,
        b'Z' => b'a',
        _ => b'?'
    }
}
pub struct CharGenerator {
    first_char_for_line: u8,
    current_char: u8,
    current_idx: u8,
}

impl CharGenerator {
    pub fn new() -> Self {
        Self {
            first_char_for_line: b'a',
            current_char: b'a',
            current_idx: 0,
        }
    }

    pub fn next(&mut self) -> u8 {
        if self.current_idx == 0 {
            let ret = self.first_char_for_line;
            // Reset current char and incr idx
            self.current_char = self.first_char_for_line;
            self.current_idx += 1;
            // Prepare char for next line
            self.first_char_for_line = next_ascii_char(self.first_char_for_line);
            return ret;
        }
        if self.current_idx > MAX_LINE_SIZE {
            self.current_idx = 0;
            return self.next();
        }
        self.current_idx += 1;
        self.current_char = next_ascii_char(self.current_char);
        self.current_char
    }
}
