const BUFFER_SIZE: usize = 4096;
const EOF: char = 0 as char;

pub trait Stream {
    fn new(input: &str) -> Self
    where
        Self: Sized;

    fn rollback(&mut self);
    fn next_char(&mut self) -> char;
    fn get_input_pointer(&self) -> usize;
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
}

// pub struct Buffer<'a> {
//     pub input: &'a str,
//     buffer: [char; BUFFER_SIZE],
//     pub input_pointer: usize,
//     pub fence: usize,
// }

// impl<'a> Stream<'a> for Buffer<'a> {
//     fn new(input: &'a str) -> &'a Buffer<'a> {
//         let mut buffer = [0 as char; BUFFER_SIZE];

//         for (i, c) in buffer.iter_mut().enumerate() {
//             *c = input.chars().nth(i).unwrap_or(EOF);
//         }

//         &Buffer {
//             input,
//             buffer,
//             input_pointer: 0,
//             fence: 0,
//         }
//     }

//     fn rollback(&mut self) {
//         if self.input_pointer == self.fence {
//             panic!("Cannot rollback past the fence");
//         }

//         self.input_pointer -= 1 % (2 * BUFFER_SIZE);
//     }

//     fn next_char(&mut self) -> char {
//         let char = self.buffer[self.input_pointer];

//         if char != EOF {
//             self.input_pointer += 1 % (2 * BUFFER_SIZE);

//             if self.input_pointer % BUFFER_SIZE == 0 {
//                 // fill buffer with next chunk of input
//                 for (i, c) in self.buffer.iter_mut().enumerate() {
//                     *c = self.input.chars().nth(i).unwrap_or(EOF);
//                 }
//                 self.fence = (self.input_pointer + BUFFER_SIZE) % (2 * BUFFER_SIZE);
//             }
//         }

//         char
//     }

//     fn get_input_pointer(&self) -> usize {
//         self.input_pointer
//     }
// }
