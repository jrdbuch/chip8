use std::path::Path;
use std::fs;


enum KeyBoard {

}

type FontChar = [u8; 5];
const FONTSET_START_ADDRESS: u16 = 0x50;
const ROM_START_ADDRESS: u16 = 0x200;

pub const PIXEL_WIDTH: usize = 32;
pub const PIXEL_HEIGHT: usize = 64;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

type Opcode = u16;

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
    pub display_memory: [[u32; PIXEL_HEIGHT]; PIXEL_WIDTH],
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
        self.memory[..80].copy_from_slice(&FONT_SET);
    }

    pub fn exec_cycle(&mut self) {
        // fetch next opcode at PC
        let opcode = self.get_next_opcode();
        // println!("{:#01x}", opcode);
        // decode instruction

        // handle timers
    }

    fn get_next_opcode(&mut self) -> u16 {
        let b1: u8 = self.memory[self.pc as usize];
        self.pc += 1;
        let b2: u8 = self.memory[self.pc as usize];
        self.pc += 1;

        let opcode = ((b1 as u16) << 8) | b2 as u16;

        opcode
    }

    fn handle_opcode(&mut self, opcode: Opcode) {

    }
}