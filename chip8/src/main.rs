#![allow(dead_code, unused)]
extern crate sfml;
extern crate rand;

mod chip;
mod ram;
mod utils;

use std::env;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::io;
use std::time::{Duration, Instant};

use chip::Chip;
use utils::*;

use sfml::window::{VideoMode, ContextSettings, Event, Key, Style};
use sfml::system::{Time, Clock, Vector2f};
use sfml::graphics::{RenderTarget, RectangleShape, Transformable, Drawable, RenderWindow, Shape,
                     Color};

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const PIXEL: u32 = 20;

fn main() {
    let arg1: String = env::args().nth(1).expect("No arguments given!");

    let mut chip = Chip::new();
    load_rom(arg1, &mut chip);
    chip.print_mem(true);
    //chip.print_mem();

    //Window etc
    let width = (SCREEN_COLUMNS * SCALE) as u32;
    let height = (SCREEN_ROWS * SCALE) as u32;
    let mut window = RenderWindow::new(
        (width, height),
        "Chip8 Emulator",
        Style::CLOSE,
        &Default::default(),
    );


    let mut rect = RectangleShape::new();
    rect.set_size((SCALE as f32, SCALE as f32));
    rect.set_fill_color(&Color::WHITE);

    let mut last_instruction = Instant::now();
    let mut last_screen = Instant::now();
    let mut delay_duration = Instant::now();

    while window.is_open() {

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }

        if Instant::now() - last_instruction > Duration::from_millis(2) {
            chip.emulate_cycle();
            last_instruction = Instant::now();
        }

        let mut s = String::new();

        if Instant::now() - delay_duration > Duration::from_millis(1000/60) {
            if chip.delay_timer > 0 {
                chip.delay_timer -= 1; 
            }
            if chip.sound_timer > 0 {
                chip.sound_timer -= 1; 
            }
            delay_duration = Instant::now();
        }

        if Instant::now() - last_screen > Duration::from_millis(10) {
            window.clear(&Color::BLACK);
            for x in 0..SCREEN_COLUMNS {
                for y in 0..SCREEN_ROWS {
                    if chip.vid_mem[y][x] == 1 {
                        let x_pos = (x * SCALE) as f32;
                        let y_pos = (y * SCALE) as f32;
                        &mut rect.set_position((x_pos, y_pos));
                        window.draw(&rect);
                    }
                }
            }
            window.display();
            last_screen = Instant::now();
        }
        read_keys(&mut chip, &window);
    }
}

fn read_keys(chip: &mut Chip, window: &RenderWindow) {
    chip.key[0x0] = Key::X.is_pressed();
    chip.key[0x1] = Key::Num1.is_pressed();
    chip.key[0x2] = Key::Num2.is_pressed();
    chip.key[0x3] = Key::Num3.is_pressed();

    chip.key[0x4] = Key::Q.is_pressed();
    chip.key[0x5] = Key::W.is_pressed();
    chip.key[0x6] = Key::E.is_pressed();
    chip.key[0x7] = Key::A.is_pressed();

    chip.key[0x8] = Key::S.is_pressed();
    chip.key[0x9] = Key::D.is_pressed();
    chip.key[0xa] = Key::Z.is_pressed();
    chip.key[0xb] = Key::C.is_pressed();

    chip.key[0xc] = Key::Num4.is_pressed();
    chip.key[0xd] = Key::R.is_pressed();
    chip.key[0xe] = Key::F.is_pressed();
    chip.key[0xf] = Key::V.is_pressed();
}

fn load_rom(filename: String, chip: &mut Chip) {
    let mut f = File::open(&filename).unwrap();
    let meta = fs::metadata(filename).unwrap();
    let file_length = meta.len();
    let mut buf: Vec<u8> = Vec::with_capacity(file_length as usize);
    f.read_to_end(&mut buf).expect("File not found");

    println!("Debug print of buffer:");
    for i in 0..10 {
        println!("{:#04X}", buf[i]);
    }
    chip.mem.write_rom(&buf);
}
