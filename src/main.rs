mod cpu;

use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{self, Event, WindowEvent};
use winit::event_loop::{EventLoop, ActiveEventLoop};
use winit::window::{WindowAttributes, WindowId};
use winit_input_helper::WinitInputHelper;
use cpu::Cpu;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCALE: u32 = 10;

struct App<'a> {
    cpu: Cpu,
    window: Option<Arc<winit::window::Window>>,
    pixels: Option<Pixels<'a>>,
}

impl ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(
            WindowAttributes::default()
                .with_title("CHIP-8")
                .with_inner_size(LogicalSize::new((WIDTH * SCALE) as f64, (HEIGHT * SCALE) as f64))
        ).unwrap();

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(
                window_size.width,
                window_size.height,
                Arc::clone(&window),
            );

            Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
        };

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    )
    {
        match event {
              WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::RedrawRequested => {
                    draw(&cpu, pixels.frame_mut());
                    pixels.render().unwrap();
                },  
                _ => {},
        }
    }
}

// const CPU_HZ: u64 = 500;

// const TIMER_HZ: u64 = 60;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() < 2 {
//         eprintln!("Usage: chip8 <rom.ch8>");
//         std::process::exit(1);
//     }

//     let rom_data = fs::read(&args[1]).expect("Failed to load ROM file");
    
//     let mut cpu = Cpu::new();
//     cpu.load_rom(&rom_data);

//     println!("Loaded ROM: {} ({} bytes)", args[1], rom_data.len());
//     println!("Starting emulation at {} Hz (timers at {} Hz)", CPU_HZ, TIMER_HZ);
//     println!("---- Press Ctrl-C to quit ----");

//     let cpu_interval = std::time::Duration::from_nanos(1_000_000_000 / CPU_HZ);
//     let timer_interval = std::time::Duration::from_nanos(1_000_000_000 / TIMER_HZ);
//     let mut last_timer_tick = std::time::Instant::now();


//     loop {
//         let frame_start = std::time::Instant::now();
//         cpu.tick();

//         if last_timer_tick.elapsed() >= timer_interval {
//             cpu.tick_timers();
//             last_timer_tick = std::time::Instant::now();

//             if cpu.sound_timer > 0 {
//                 print!("\x07");
//             }
//         }

//         if cpu.draw_flag {
//             render_to_terminal(&cpu);
//             cpu.draw_flag = false;
//         }

//         let elapsed = frame_start.elapsed();
//         if elapsed < cpu_interval {
//             std::thread::sleep(cpu_interval - elapsed);
//         }
//     }
// }

// fn render_to_terminal(cpu: &Cpu) {
//     use cpu::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

//     print!("\x1B[H");

//     for y in 0..DISPLAY_HEIGHT {
//         for x in 0..DISPLAY_WIDTH {
//             let pixel = cpu.display[y * DISPLAY_WIDTH + x];
//             print!("{}", if pixel { "█" } else { " " });
//         }
//         println!();
//     }
// }