const BUFFER_SIZE: usize = 4096;
const EOF: char = 0 as char;

pub struct Buffer<'a> {
    input: &'a str,
    buffer: [char; BUFFER_SIZE],
    input_pointer: usize,
    fence: usize,
}

impl<'a> Buffer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut buffer = [0 as char; BUFFER_SIZE];

        for (i, c) in buffer.iter_mut().enumerate() {
            *c = input.chars().nth(i).unwrap_or(EOF);
        }

        Buffer {
            input,
            buffer,
            input_pointer: 0,
            fence: 0,
        }
    }

    pub fn rollback(&mut self) {
        if self.input_pointer == self.fence {
            panic!("Cannot rollback past the fence");
        }

        self.input_pointer -= 1 % (2 * BUFFER_SIZE);
    }

    pub fn next_char(&mut self) -> char {
        let char = self.buffer[self.input_pointer];

        if char != EOF {
            self.input_pointer += 1 % (2 * BUFFER_SIZE);

            if self.input_pointer % BUFFER_SIZE == 0 {
                for i in 0..BUFFER_SIZE {
                    self.buffer[i] = self.input.chars().nth(i).unwrap_or(EOF);
                }
            }
        }

        char
    }
}
