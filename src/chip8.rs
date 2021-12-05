use std::path::Path;
use std::fs;


enum KeyBoard {

}

type FontChar = [u8; 5];
const FONTSET_START_ADDRESS: u16 = 0x50;
const ROM_START_ADDRESS: u16 = 0x200;

pub const PIXEL_WIDTH: usize = 32;
pub const PIXEL_HEIGHT: usize = 64;

enum Opcode {

}

// enum opcode {
//     cls,
//     ret,
//     sys(addr),
//     jmp(addr),
//     call(addr),
// }

pub struct Chip8 {
    memory: [u8; 4096], 
    registers: [u8; 16],
    stack: [u16; 16],       // holds PC for when CALL was executed
    stack_pointer: u8,
    pc: u16,
    index_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    display_memory: [[u32; PIXEL_HEIGHT]; PIXEL_WIDTH],
}

impl Chip8 {
    pub fn new() -> Chip8{
        let mut chip8 = Chip8 {
            memory: [0; 4096], 
            registers: [0; 16],
            stack: [0; 16],
            stack_pointer: 0,
            pc: ROM_START_ADDRESS,
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            display_memory: [[0; 64]; 32],
        };

        chip8.load_fontset();

        chip8
    }

    pub fn load_ROM(&mut self, path: &Path) {
        // load into memory
        let rom: Vec<u8> = fs::read(path).unwrap();

        for (pos, e) in rom.iter().enumerate() {
            self.memory[pos+ROM_START_ADDRESS as usize] = *e;
        }
    }

    // load fontset into memory at predefined location
    fn load_fontset(&mut self) {

    }

    fn exec_cycle() {
        // fetch next opcode at PC

        // decode instruction

        // handle timers
    }

    fn handle_opcode(&mut self, opcode: Opcode) {

    }

}