mod cpu;

use cpu::Cpu;
use std::{env, fs};

const CPU_HZ: u64 = 500;

const TIMER_HZ: u64 = 60;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: chip8 <rom.ch8>");
        std::process::exit(1);
    }

    let rom_data = fs::read(&args[1]).expect("Failed to load ROM file");
    
    let mut cpu = Cpu::new();
    cpu.load_rom(&rom_data);

    println!("Loaded ROM: {} ({} bytes)", args[1], rom_data.len());
    println!("Starting emulation at {} Hz (timers at {} Hz)", CPU_HZ, TIMER_HZ);
    println!("---- Press Ctrl-C to quit ----");

    let cpu_interval = std::time::Duration::from_nanos(1_000_000_000 / CPU_HZ);
    let timer_interval = std::time::Duration::from_nanos(1_000_000_000 / TIMER_HZ);
    let mut last_timer_tick = std::time::Instant::now();

}