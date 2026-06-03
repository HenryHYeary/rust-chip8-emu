pub const RAM_SIZE: usize = 4096;
pub const PROGRAM_START: u16 = 0x200;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub const STACK_SIZE: usize = 16;

pub const FONT_START: usize = 0x000;
#[rustfmt::skip]
const FONT_DATA: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Cpu {
    pub ram: [u8; RAM_SIZE],
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub stack: [u16; STACK_SIZE],
    pub sp: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    pub keys: [bool; 16],
    pub draw_flag: bool,   
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            ram: [0; RAM_SIZE],
            v: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            keys: [false; 16],
            draw_flag: false,
        };

        cpu.ram[FONT_START..FONT_START + FONT_DATA.len()].copy_from_slice(&FONT_DATA);
        cpu
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let start = PROGRAM_START as usize;
        self.ram[start..start + data.len()].copy_from_slice(data);
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    } 

    pub fn fetch(&mut self) -> u16 {
        let hi = self.ram[self.pc as usize] as u16;
        let lo = self.ram[self.pc as usize + 1] as u16;
        self.pc += 2;
        (hi << 8) | lo
    }

    fn execute(&mut self, op: u16) {
        let nibble = ((op & 0xF000) >> 12) as u8;
    }
}

