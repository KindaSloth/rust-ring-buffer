use std::convert::TryInto;

fn convert_vec_into_array<T, const SIZE: usize>(v: Vec<T>) -> [T; SIZE] {
    let boxed_slice = v.into_boxed_slice();

    let boxed_array: Box<[T; SIZE]> = match boxed_slice.try_into() {
        Ok(ba) => ba,
        Err(o) => panic!("Expected a Vec of length {} but it was {}", SIZE, o.len()),
    };

    *boxed_array
}

struct RingBuffer<const SIZE: usize> {
    buffer: [Option<String>; SIZE],
    size: usize,
    write_pointer: usize,
    read_pointer: usize
}

impl<const SIZE: usize> RingBuffer<SIZE> {
    fn new() -> Self {
        let initial_buffer: Vec<Option<String>> = vec![None; SIZE];

        Self { 
            buffer: convert_vec_into_array(initial_buffer), 
            size: SIZE, 
            write_pointer: 0, 
            read_pointer: 0 
        }
    }

    fn write(&mut self, value: String) {
        self.buffer[self.write_pointer] = Some(value);
        self.write_pointer = (self.write_pointer + 1) % self.size;
    }

    fn read(&mut self) -> Option<&String> {
        let x = self.buffer[self.read_pointer].as_ref();

        self.read_pointer = (self.read_pointer + 1) % self.size;

        x
    }

    fn listen(&mut self) {
        while self.read_pointer < self.write_pointer {
            println!("{:?}", self.read())
        }
    }
}

fn main() {
    let mut ring_buffer: RingBuffer<20> = RingBuffer::new();

    ring_buffer.write("test".into());
    ring_buffer.write("log test".into());
    ring_buffer.write("amazing test".into());

    ring_buffer.listen();
}  
