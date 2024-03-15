extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use std::collections::HashSet;
use std::io::{self, Write};
use std::collections::HashMap;

fn generate_mapping() -> HashMap<Keycode, HashMap<Keycode,Keycode>> {
    let mut first_mapping = HashMap::new();
    first_mapping.insert(Keycode::H, Keycode::Y);
    first_mapping.insert(Keycode::J, Keycode::U);
    first_mapping.insert(Keycode::K, Keycode::I);
    first_mapping.insert(Keycode::L, Keycode::O);
    first_mapping.insert(Keycode::Semicolon, Keycode::P);

    let mut second_mapping = HashMap::new();
    second_mapping.insert(Keycode::A, Keycode::Q);
    second_mapping.insert(Keycode::S, Keycode::W);
    second_mapping.insert(Keycode::D, Keycode::E);
    second_mapping.insert(Keycode::F, Keycode::R);
    second_mapping.insert(Keycode::G, Keycode::T);
    
    let mut third_mapping = HashMap::new();
    third_mapping.insert(Keycode::A, Keycode::Z);
    third_mapping.insert(Keycode::S, Keycode::X);
    third_mapping.insert(Keycode::D, Keycode::C);
    third_mapping.insert(Keycode::F, Keycode::V);
    
    let mut fourth_mapping = HashMap::new();
    fourth_mapping.insert(Keycode::J, Keycode::B);
    fourth_mapping.insert(Keycode::K, Keycode::N);
    fourth_mapping.insert(Keycode::L, Keycode::M);

    let mut keycode_map = HashMap::new();
    keycode_map.insert(Keycode::A, first_mapping);
    keycode_map.insert(Keycode::Semicolon, second_mapping);
    keycode_map.insert(Keycode::L, third_mapping);
    keycode_map.insert(Keycode::S, fourth_mapping);

    return keycode_map;
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Combinational Typint", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut modifier_key: Option<Keycode> = None;
    let mut was_pressed: bool = false;
    let keycode_map = generate_mapping();

    println!("COMBINATIONAL TYPING, Never leave the home row!");
    println!("-----------------------------------------------");
    println!("Focus on the SDL2 window for the app to capture input, typing results print to console");
    println!("This works by using modifier keys to remap keys on the fly.");
    println!("Homerow works like usual");
    println!("If you hold A, then these keys are remapped H -> Y, J -> U, K -> I, L -> O, ; -> P");
    println!("If you hold S, then these keys are remapped J -> B, K -> N, L -> M");
    println!("If you hold ;, then these keys are remapped A -> Q, S -> W, D -> E, F -> R, G -> T");
    println!("If you hold L, then these keys are remapped A -> Z, S -> X, D -> C, F -> V");
    println!("EXAMPLE: H, ;+D, L, L, A+O       PRINTS: HELLO");
    println!("Backspace and Space work as usual. No shift/caps");

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {println!("\nQUIT"); break 'main;},
                Event::KeyDown { keycode: Some(key), .. } => {
                    if (modifier_key == None) {
                        if (keycode_map.get(&key) != None) {
                            modifier_key = Some(key);
                        }
                    } else {
                        was_pressed = true;
                        if let Some(m_key) = modifier_key {
                            if (m_key == key) {
                                break;
                            }
                            if (keycode_map.get(&m_key).unwrap().get(&key) != None) {
                                print!("{:?}", keycode_map.get(&m_key).unwrap().get(&key).unwrap());
                            }
                            io::stdout().flush().unwrap();
                        }
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } => { 
                    if (modifier_key == Some(key)) {
                        modifier_key = None;
                        if (was_pressed) {
                            was_pressed = false;
                        } else {
                            if (key == Keycode::Space) {
                                print!(" ");
                            } else if (key == Keycode::Backspace) {
                                print!("\x08 \x08");
                            } else {
                                print!("{:?}", key);
                            }
                            io::stdout().flush().unwrap();
                        }
                    } else if (modifier_key == None) {
                            if (key == Keycode::Space) {
                                print!(" ");
                            } else if (key == Keycode::Backspace) {
                                print!("\x08 \x08");
                            } else {
                                print!("{:?}", key);
                            }
                            io::stdout().flush().unwrap();
                    }
                },
                _ => {}
            }
        }
    }
}
