use std::fmt;
use std::fmt::Display;
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

#[derive(fmt::Debug)]
enum OpCode {
    CLS,
    JMP(u16),
    SET_REG(u16),
    ADD_REG(u16),
    SET_IDX_REG(u16),
    DISP(u16),
}

// impl fmt::Display for OpCode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "{}", self.to_string())
//     }
// }

fn get_nth_nibble(val: u16, n: u8) -> u16 {
    // n=0 is first nibble
    (val & (0xF << n*4)) >> n*4
}

fn get_first_n_nibbles(val: u16, n: u8) -> u16{
    let mut new_val: u16 = 0;

    for i in 0..n {
        new_val |= val & (0xF << i*4);
    }

    new_val
}

#[derive(fmt::Debug)]
struct InvalideOpCode {
    details: String
}

impl OpCode {
    fn from_bytes(b1: u8, b2: u8) -> Result<OpCode, InvalideOpCode> {
        // concat bytes into u16
        let b = ((b1 as u16) << 8) | b2 as u16;

        let nth_nib = |n| get_nth_nibble(b, n);
        let firts_n_nibs = |n| get_first_n_nibbles(b, n);

        if b == 0x00E0 {
            Ok(OpCode::CLS)    
        } else if nth_nib(3) == 0x1 {
            Ok(OpCode::JMP(b))
        } else if nth_nib(3) == 0x6 {
            Ok(OpCode::SET_REG(b))
        } else if nth_nib(3) == 0x7 {
            Ok(OpCode::ADD_REG(b))
        } else if nth_nib(3) == 0xA {
            Ok(OpCode::SET_IDX_REG(b))
        } else if nth_nib(3) == 0xD {
            Ok(OpCode::DISP(b))
        } else {
            Err(InvalideOpCode{details:format!("Invalid OpCode from 2 bytes {:#04x}", b)})
        }
    }
}

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
        let opcode = self.get_next_opcode().unwrap();

        println!("{:?}", opcode);
        // decode instruction
        self.handle_opcode(opcode);

        // handle timers
    }

    // get 2 byte opcode and update program counter
    fn get_next_opcode(&mut self) -> Result<OpCode, InvalideOpCode>{
        let b1: u8 = self.memory[self.pc as usize];
        self.pc += 1;
        let b2: u8 = self.memory[self.pc as usize];
        self.pc += 1;

        OpCode::from_bytes(b1, b2)
    }

    fn handle_opcode(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::CLS => self.clear_screen(),

            OpCode::JMP(b) => self.pc = get_first_n_nibbles(b, 3),

            OpCode::SET_REG(b) => self.registers[get_nth_nibble(b, 2) as usize] = get_first_n_nibbles(b, 2) as u8,

            OpCode::ADD_REG(b) => self.registers[get_nth_nibble(b, 2) as usize] = get_first_n_nibbles(b, 2) as u8,

            OpCode::SET_IDX_REG(b) => self.index_register = get_first_n_nibbles(b, 3),

            OpCode::DISP(b) =>  
        }

    }

    // Opcode Methods
    fn clear_screen(&mut self) {
        for row in self.display_memory.iter_mut() {
            for col in row.iter_mut() {
                *col = 0;
            }
        }
    }

    fn jump(&mut self, addr: u16) {

    }

    fn draw_sprite(&mut self, vx: u8, vy: u8, h: u8) {

        let x_pos = self.registers[vx] % PIXEL_WIDTH;
        let y_pos = self.registers[vy] % PIXEL_HEIGHT;

        for i in x..x+8 {
            for j in y..y+h {
                self.display_memory = 0;
            }
        }
    }

}