use std::fmt;
use std::fmt::Display;
use std::path::Path;
use std::fs;
use rand::Rng;
use crate::drivers::{KeyState, KeyboardDriver};
use sdl2::keyboard::Keycode;
use std::thread::sleep;
use std::time::Duration;
use crate::utils::*;

type FontChar = [u8; 5];
type OpCode = u16;

pub const PIXEL_WIDTH: usize = 64;
pub const PIXEL_HEIGHT: usize = 32;
const ROM_START_ADDRESS: u16 = 0x200;


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
struct InvalideOpCode {
    details: String
}


pub struct Chip8 {
    memory: [u8; 4096], 
    registers: [u8; 16],
    stack: [u16; 16],       // holds PC for when CALL was executed
    stack_pointer: usize,
    pc: u16,
    index_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    pub display_memory: [[u32; PIXEL_WIDTH]; PIXEL_HEIGHT],
    pub draw_flag: bool,
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
            display_memory: [[0; 64]; 32],  // (row, col)
            draw_flag: false,
        };

        chip8.load_fontset();

        chip8
    }

    pub fn load_rom(&mut self, path: &Path) {
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

    pub fn exec_cycle(&mut self, key_state: &KeyState) {
        self.draw_flag = false;
        // fetch next opcode at PC
        // println!("\n\npc start of cycle {:?}", self.pc);
        let opcode = self.get_next_opcode();
        // println!("pc after get next opcode {:?}", self.pc);

        // println!("Opcode {:?}", opcode);
        // decode instruction
        self.handle_opcode(opcode, key_state);
        // println!("pc after handle oppcode {:?}", self.pc);
        
        // handle timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }
        
        if self.sound_timer > 0 {
            self.sound_timer -= 1
        }

        // let exec cycle run at ~60 Hz
        sleep(Duration::new(1/60, 0));
    }

    // get 2 byte opcode and update program counter
    fn get_next_opcode(&mut self) -> OpCode {
        let b1: u8 = self.memory[self.pc as usize];
        self.pc += 1;
        let b2: u8 = self.memory[self.pc as usize];
        self.pc += 1;

        concat_bytes(b1, b2)
    }

    fn handle_opcode(&mut self, op: OpCode, key_state: &KeyState) {
        let mut nibs: [u16; 4] = [0; 4];
        for n in 0..4 {
            nibs[3-n] = get_nth_nibble(op, n as u8);
        }
        
        let nn = get_first_n_nibbles(op, 2) as u8;
        let nnn = get_first_n_nibbles(op, 3);

        println!("NIBS {:#x} {:#x} {:#x} {:#x}", nibs[0], nibs[1], nibs[2], nibs[3]);

        match nibs {
            [0, 0, 0xE, 0]      => self.clear_screen(),

            // return from subroutine
            [0, 0, 0xE, 0xE]    => {self.stack_pointer -= 1; self.pc = self.stack[self.stack_pointer];},

            // Jump
            [1, _, _, _]        => self.pc = nnn,

            // Call Subroutine
            [2, _, _, _]        => {self.stack[self.stack_pointer] = self.pc; self.stack_pointer += 1; self.pc = nnn},

            //Skips the next instruction if VX equals NN
            [3, x, _, _]        => {if self.registers[x as usize] == nn as u8 {self.pc += 2}},

            // Skips the next instruction if VX does not equal NN
            [4, x, _, _]        => {if self.registers[x as usize] != nn as u8 {self.pc += 2}},

            // Skips the next instruction if VX equals VY            
            [5, x, y, _]        => {if self.registers[x as usize] == self.registers[y as usize] {self.pc += 2}},

            // Sets VX to NN
            [6, x, _, _]        => {self.registers[x as usize] = nn},

            // Adds NN to VX. (Carry flag is not changed);
            [7, x, _, _]        => { 
                let overflow_res = addition_with_overflow(self.registers[x as usize], nn as u8);
                self.registers[x as usize] = overflow_res.val;
            },
                
            // Sets VX to the value of VY.
            [8, x, y, 0]        => {self.registers[x as usize] = self.registers[y as usize]},

            // bitwise or
            [8, x, y, 1]        => {self.registers[x as usize] |= self.registers[y as usize]}

            // bitwise and
            [8, x, y, 2]        => {self.registers[x as usize] &= self.registers[y as usize]}

            // bitwise xor
            [8, x, y, 3]        => {self.registers[x as usize] ^= self.registers[y as usize]},

            // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
            [8, x, y, 4]        => { 
                let overflow_res = addition_with_overflow(self.registers[x as usize], self.registers[y as usize]);
                self.registers[x as usize] = overflow_res.val;
                if overflow_res.overflowed {self.registers[0xF] = 1} else {self.registers[0xF] = 0};
            }

            // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
            [8, x, y, 5]        => {
                let overflow_res = subtract_with_overflow(self.registers[x as usize], self.registers[y as usize]);
                if overflow_res.overflowed {self.registers[0xF] = 0} else {self.registers[0xF] = 1}
                self.registers[x as usize] = overflow_res.val;
            },

            // Stores the least significant bit of VX in VF and then shifts VX to the right by 1.[b]
            [8, x, _, 6]        => {self.registers[0xF] = self.registers[x as usize] & 1; self.registers[x as usize] >>= 1;},

            // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
            [8, x, y, 7]        => {
                let overflow_res = subtract_with_overflow(self.registers[y as usize], self.registers[x as usize]);
                if overflow_res.overflowed {self.registers[0xF] = 0} else {self.registers[0xF] = 1}
                self.registers[x as usize] = overflow_res.val;
            },
            
            // Stores the most significant bit of VX in VF and then shifts VX to the left by 1
            [8, x, _, 0xE]      => {self.registers[0xF] = self.registers[x as usize] & 0x80; self.registers[x as usize] <<= 1;},

            // Skips the next instruction if VX does not equal VY
            [9, x, y, 0]        => {if self.registers[x as usize] != self.registers[y as usize] {self.pc += 2}},

            // Sets I to the address NNN.
            [0xA, _, _, _]      => self.index_register = nnn,

            // Jumps to the address NNN plus V0.
            [0xB, _, _, _]      => self.pc = self.registers[0] as u16 + nnn,

            // Sets VX to the result of a bitwise and operation on a random number and NN
            [0xC, x, _, _]      => self.registers[x as usize] = rand::thread_rng().gen_range(0..255) & nn,

            // Draw sprites
            [0xD, x, y, n]      => self.draw_sprite(x as u8, y as u8, n as u8),

            // Skips the next instruction if the key stored in VX is pressed.
            [0xE, x, 9, 0xE]    => {
                match KeyboardDriver::int_to_keycode(self.registers[x as usize]) {
                    Some(kc) => if key_state[&kc] {self.pc += 2;},
                    None => (),
                }
            }

            // Skips the next instruction if the key stored in VX is not pressed
            [0xE, x, 0xA, 1]    => {
                match KeyboardDriver::int_to_keycode(self.registers[x as usize]) {
                    Some(kc) => {if !key_state[&kc] {self.pc += 2;}},
                    None => (),
                }
            }

            // Sets VX to the value of the delay timer.
            [0xF, x, 0, 7]      => {self.registers[x as usize] = self.delay_timer},

            // Sets the delay timer to VX.
            [0xF, x, 1, 5]      => {self.delay_timer = self.registers[x as usize]},

            // Sets the sound timer to VX.
            [0xF, x, 1, 8]      => {self.sound_timer = self.registers[x as usize]},

            // Adds VX to I. VF is not affected
            [0xF, x, 1, 0xE]    => self.index_register += self.registers[x as usize] as u16,

            // A key press is awaited, and then stored in VX. Blocking Operation. 
            [0xF, x, 0, 0xA]    => self.wait_for_keypress(x as usize, key_state),
            
            // Sets I to the location of the sprite for the character in VX.
            [0xF, x, 2, 9]      => {
                self.index_register = (self.registers[x as usize] * 5) as u16; 
                println!("FONT SPRITE {} {}", self.registers[x as usize], self.index_register);
            },

            // Stores the binary-coded decimal representation of VX,
            [0xF, x, 3, 3]      => {
                for (i, dig) in convert_to_binary_encoded_decimal(self.registers[x as usize]).into_iter().enumerate() {
                    self.memory[(self.index_register as usize) + i] = dig;
                }
            },

            // Stores from V0 to VX (including VX) in memory, starting at address I. 
            // The offset from I is increased by 1 for each value written, but I itself is left unmodified.
            [0xF, x, 5, 5]      => {
                for i in 0..x+1 { 
                    self.memory[(self.index_register as usize) + i as usize] = self.registers[i as usize];
                }
            },

            // Fills from V0 to VX (including VX) with values from memory, starting at address I.
            // The offset from I is increased by 1 for each value written, but I itself is left unmodified.
            [0xF, x, 6, 5]      => {
                for i in 0..x+1 { 
                    self.registers[i as usize] = self.memory[self.index_register as usize + i as usize];
                }
            },

            [_, _, _, _] => panic!("Invalid OpCode as Hex: '{:#04x}' and as Decimal: '{}'", op, op)
        };
    }

    // Opcode Methods
    fn clear_screen(&mut self) {
        for row in self.display_memory.iter_mut() {
            for col in row.iter_mut() {
                *col = 0;
            }
        }
    }

    fn draw_sprite(&mut self, vx: u8, vy: u8, h: u8) {
        self.draw_flag = true;
        let x_pos = self.registers[vx as usize] as usize % PIXEL_WIDTH;
        let y_pos = self.registers[vy as usize] as usize % PIXEL_HEIGHT;
        // println!("x_pos {} y_pos {}", x_pos, y_pos);

        self.registers[0xF] = 0; // VF = 0

        for irow in 0..(h as usize) {

            // don't wrap draw position
            if irow + y_pos >= PIXEL_HEIGHT {
                continue
            }

            let pixel = self.memory[self.index_register as usize + irow ];

            for icol in 0..8 {
                
                // don't wrap draw position
                if icol + x_pos >= PIXEL_WIDTH {
                    continue
                }

                if pixel & (0x80 >> (icol as u8)) != 0 {
                    self.display_memory[y_pos+irow][x_pos+icol] ^= 1;

                    if self.display_memory[y_pos+irow][x_pos+icol] == 0 {
                        self.registers[0xF] = 1;
                    }
                }
            }
        }
    }

    fn wait_for_keypress(&mut self, reg_index: usize, key_state: &KeyState) {
        let mut block = true;
        for (key, pressed) in key_state.iter() {
            if *pressed {
                self.registers[reg_index] = KeyboardDriver::keycode_to_int(key).unwrap(); 
                block = false; 
                break
            }
        }
        if block {self.pc -= 2};
    }
}