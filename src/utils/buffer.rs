// const BUFFER_SIZE: usize = 4096;
const EOF: char = 0 as char;
// const N2: usize = BUFFER_SIZE;
// const N: usize = N2 / 2;

pub trait Stream {
    fn new(input: &str) -> Self
    where
        Self: Sized;

    fn rollback(&mut self);
    fn next_char(&mut self) -> char;
    fn get_input_pointer(&self) -> usize;
    fn is_eof(&self) -> bool;
}

pub struct SimpleBuffer {
    pub input: String,
    pub input_pointer: usize,
}

impl Stream for SimpleBuffer {
    fn new(input: &str) -> SimpleBuffer {
        SimpleBuffer {
            input: input.to_string(),
            input_pointer: 0,
        }
    }

    fn rollback(&mut self) {
        self.input_pointer -= 1;
    }

    fn next_char(&mut self) -> char {
        let char = self.input.chars().nth(self.input_pointer).unwrap_or(EOF);
        self.input_pointer += 1;
        char
    }

    fn get_input_pointer(&self) -> usize {
        self.input_pointer
    }

    fn is_eof(&self) -> bool {
        self.input_pointer >= self.input.len()
    }
}

// pub struct Buffer {
//     pub input: String,
//     pub buffer: [char; N2],
//     pub input_pointer: usize,
//     pub fence: usize,
// }

// impl Stream for Buffer {
//     fn new(input: &str) -> Buffer {
//         let mut buffer = [0 as char; N2];

//         for (i, c) in buffer.iter_mut().enumerate().take(N) {
//             *c = input.chars().nth(i).unwrap_or(EOF);
//         }

//         Buffer {
//             input: input.to_string(),
//             buffer,
//             input_pointer: 0,
//             fence: 0,
//         }
//     }

//     fn rollback(&mut self) {
//         if self.input_pointer == self.fence {
//             panic!("Cannot rollback past the fence");
//         }

//         self.input_pointer = (self.input_pointer - 1) % (N2);
//     }

//     fn next_char(&mut self) -> char {
//         let char = self.buffer[self.input_pointer];

//         if char != EOF {
//             self.input_pointer = (self.input_pointer + 1) % N2;

//             if self.input_pointer % N == 0 {
//                 for (i, c) in self
//                     .buffer
//                     .iter_mut()
//                     .enumerate()
//                     .skip(self.input_pointer)
//                     .take(self.input_pointer + N2)
//                 {
//                     *c = self.input.chars().nth(i).unwrap_or(EOF);
//                 }
//                 self.fence = (self.input_pointer + N) % N2;
//             }
//         }

//         char
//     }

//     fn get_input_pointer(&self) -> usize {
//         self.input_pointer
//     }
// }

// impl Default for Buffer {
//     fn default() -> Self {
//         Buffer {
//             input: String::new(),
//             buffer: [0 as char; BUFFER_SIZE],
//             input_pointer: 0,
//             fence: 0,
//         }
//     }
// }
