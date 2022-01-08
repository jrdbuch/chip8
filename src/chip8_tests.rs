use super::*;
use crate::drivers::create_key_state;

#[test]
fn test_00E0() {
	let rom: Vec<u8> = vec![0x00, 0xE0];	

	let mut chip8 = Chip8::new();
	let key_state = create_key_state();
	chip8.load_rom(rom);
	chip8.display_memory[10][10] = 1; 

	chip8.exec_cycle(&key_state);

	assert_eq!(chip8.display_memory[10][10], 0);
}

#[test]
fn test_1NNN() {
	let rom: Vec<u8> = vec![0x1A, 0xBC];	

	let mut chip8 = Chip8::new();
	let key_state = create_key_state();
	chip8.load_rom(rom);

	chip8.exec_cycle(&key_state);

	assert_eq!(chip8.pc, 0xABC);
}

#[test]
fn test_2NNN_and_00EE() {
	let rom: Vec<u8> = vec![0x22, 0x04, 0x00, 0xE0, 0x00, 0xEE];	

	let mut chip8 = Chip8::new();
	let key_state = create_key_state();
	chip8.load_rom(rom);

	assert_eq!(chip8.stack_pointer, 0);
	let pc_at_start = chip8.pc;

	// enter subroutine call
	chip8.exec_cycle(&key_state);

	assert_eq!(chip8.stack_pointer, 1);
	assert_eq!(chip8.stack[0], pc_at_start+2);
	assert_eq!(chip8.pc, 0x204);

	// call immediately returns
	chip8.exec_cycle(&key_state);
	assert_eq!(chip8.stack_pointer, 0);
	assert_eq!(chip8.pc, 0x202);
}
