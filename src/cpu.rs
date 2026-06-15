use rand::Rng;

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
        let x = ((op & 0x0F00) >> 8) as usize;
        let y = ((op & 0x00F0) >> 4) as usize;

        let n = (op & 0x000F) as u8;
        let nn = (op & 0x00FF) as u8;
        let nnn = op & 0x0FFF;

        match nibble {
            0x0 => match op {
                0x00E0 => self.op_cls(),
                0x00EE => self.op_ret(),
                _ => {},
            },
            0x1 => self.op_jp(nnn),
            0x2 => self.op_call(nnn),
            0x3 => self.op_se_vx_byte(x, nn),
            0x4 => self.op_sne_vx_byte(x, nn),
            0x5 => self.op_se_vx_vy(x, y),
            0x6 => self.op_ld_vx_byte(x, nn),
            0x7 => self.op_add_vx_byte(x, nn),

            0x8 => match n {
                0x0 => self.op_ld_vx_vy(x, y),
                0x1 => self.op_or(x, y),
                0x2 => self.op_and(x, y),
                0x3 => self.op_xor(x, y),
                0x4 => self.op_add_vx_vy(x, y),
                0x5 => self.op_sub(x, y),
                0x6 => self.op_shr(x),
                0x7 => self.op_subn(x, y),
                0xE => self.op_shl(x),
                _ => eprintln!("Unknown opcode: {:#06X}", op),                
            },

            0x9 => self.op_sne_vx_vy(x, y),
            0xA => self.op_ld_i(nnn),
            0xB => self.op_jp_v0(nnn),
            0xC => self.op_rnd(x, nn),
            0xD => self.op_drw(x, y, n),

            0xE => match nn {
                0x9E => self.op_skp(x),
                0xA1 => self.op_sknp(x),
                _ => eprintln!("Unknown opcode: {:06X}", op),
            },

            0xF => match nn {
                0x07 => self.op_ld_vx_dt(x),
                0x0A => self.op_ld_vx_k(x),
                0x15 => self.op_ld_dt_vx(x),
                0x18 => self.op_ld_st_vx(x),
                0x1E => self.op_add_i_vx(x),
                0x29 => self.op_ld_f_vx(x),
                0x33 => self.op_ld_b_vx(x),
                0x55 => self.op_ld_i_vx(x),
                0x65 => self.op_ld_vx_i(x),
                _ => eprintln!("Unknown opcode: {:06X}", op),
            },

            _ => eprintln!("Unknown opcode: {:06X}", op),
        }
    }

    fn op_cls(&mut self) {
        self.display = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.draw_flag = true;
    }

    fn op_ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    fn op_jp(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn op_call(&mut self, nnn: u16) {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    fn op_se_vx_byte(&mut self, x: usize, nn: u8) {
        if self.v[x] == nn {
            self.pc += 2;
        }
    }

    fn op_sne_vx_byte(&mut self, x: usize, nn: u8) {
        if self.v[x] != nn {
            self.pc += 2;
        }
    }

    fn op_se_vx_vy(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    fn op_ld_vx_byte(&mut self, x: usize, nn: u8) {
        if x < 16 {
            self.v[x] = nn;
        } else {
            eprintln!("Invalid register index: {}", x);
        }
    }

    fn op_add_vx_byte(&mut self, x: usize, nn: u8) {
        self.v[x] = self.v[x].wrapping_add(nn);
    }

    fn op_ld_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    fn op_or(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    fn op_and(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    fn op_xor(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    fn op_add_vx_vy(&mut self, x: usize, y: usize) {
        let (result, overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = result;
        self.v[0xF] = overflow as u8;
    }

    fn op_sub(&mut self, x: usize, y: usize) {
        let (result, borrow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = result;
        self.v[0xF] = !borrow as u8;
    }

    fn op_shr(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 0x1;
        self.v[x] >>= 1;
    }

    fn op_subn(&mut self, x: usize, y: usize) {
        let (result, borrow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = result;
        self.v[0xF] = !borrow as u8;
    }

    fn op_shl(&mut self, x: usize) {
        self.v[0xF] = (self.v[x] >> 7) & 0x1;
        self.v[x] <<= 1;
    }

    fn op_sne_vx_vy(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    fn op_ld_i(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn op_jp_v0(&mut self, nnn: u16) {
        self.pc = nnn + self.v[0] as u16;
    }

    fn op_rnd(&mut self, x: usize, nn: u8) {
        let rand_byte = rand::thread_rng().gen::<u8>() & nn;
        self.v[x] = rand_byte & nn;
    }

    fn op_drw(&mut self, x: usize, y: usize, n: u8) {
        let x_start = self.v[x] as usize % DISPLAY_WIDTH;
        let y_start = self.v[y] as usize % DISPLAY_HEIGHT;
        self.v[0xF] = 0;

        for row in 0..n as usize {
            let sprite_byte  = self.ram[self.i as usize + row];
            let py = (y_start + row) % DISPLAY_HEIGHT;

            for col in 0..8usize {
                if sprite_byte & (0x80 >> col) != 0 {
                    let px = (x_start + col) % DISPLAY_WIDTH;
                    let idx = py * DISPLAY_WIDTH + px;

                    if self.display[idx] {
                        self.v[0xF] = 1;
                    }
                    self.display[idx] ^= true;
                }
            }
        }

        self.draw_flag = true;
    }

    fn op_skp(&mut self, x: usize) {
        if self.keys[self.v[x] as usize] {
            self.pc += 2;
        }
    }

    fn op_sknp(&mut self, x: usize) {
        if !self.keys[self.v[x] as usize] {
            self.pc += 2;
        }
    }

    fn op_ld_vx_dt(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }

    fn op_ld_vx_k(&mut self, x: usize) {
        if let Some(key) = self.keys.iter().position(|&k| k) {
            self.v[x] = key as u8;
        } else {
            self.pc -= 2;
        }
    }

    fn op_ld_dt_vx(&mut self, x: usize) {
        self.delay_timer = self.v[x];
    }

    fn op_ld_st_vx(&mut self, x: usize) {
        self.sound_timer = self.v[x];
    }

    fn op_add_i_vx(&mut self, x: usize) {
        self.i = self.i.wrapping_add(self.v[x] as u16);
    }

    fn op_ld_f_vx(&mut self, x: usize) {
        self.i = (FONT_START + (self.v[x] as usize & 0xF) * 5) as u16;
    }

    fn op_ld_b_vx(&mut self, x: usize) {
        let val = self.v[x];
        self.ram[self.i as usize] = val / 100;
        self.ram[self.i as usize + 1] = (val / 10) % 10;
        self.ram[self.i as usize + 2] = val % 10;
    }

    fn op_ld_i_vx(&mut self, x: usize) {
        for reg in 0..=x {
            self.ram[self.i as usize + reg] = self.v[reg];
        }
    }

    fn op_ld_vx_i(&mut self, x: usize) {
        for reg in 0..=x {
            self.v[reg] = self.ram[self.i as usize + reg];
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}

