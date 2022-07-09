use vte::{Params, Parser, Perform};

struct Position{
    rows : u64,
    cols : u64
}

pub struct Chtty{
    cursor_pos : Position,
    window_size : Position,
    data  : Vec<Vec<u8>>
}

impl Chtty{
    pub fn new(rows : u64 , cols : u64) {
        let data = vec![];
        for i in 0..rows{
            data.push(vec![]);
            for j in 0..cols {
                data[i] = vec![0u8 ; cols];
            }
        }
        Self {
            cursor_pos : Position{0 , 0},
            window_size : Position{rows , cols},
            data : vec![]
        }
    }
}

impl Perform for Chtty{
    fn print(&mut self, c: char) {
        println!("[print] {:?}", c);
    }

    fn execute(&mut self, byte: u8) {
        println!("[execute] {:02x}", byte);
    }

    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        println!(
            "[hook] params={:?}, intermediates={:?}, ignore={:?}, char={:?}",
            params, intermediates, ignore, c
        );
    }

    fn put(&mut self, byte: u8) {
        println!("[put] {:02x}", byte);
    }

    fn unhook(&mut self) {
        println!("[unhook]");
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        println!("[osc_dispatch] params={:?} bell_terminated={}", params, bell_terminated);
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        println!(
            "[csi_dispatch] params={:#?}, intermediates={:?}, ignore={:?}, char={:?}",
            params, intermediates, ignore, c
        );
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        println!(
            "[esc_dispatch] intermediates={:?}, ignore={:?}, byte={:02x}",
            intermediates, ignore, byte
        );
    }
}