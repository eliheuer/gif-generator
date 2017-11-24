extern crate rand;
extern crate gif;

use rand::Rng;
use gif::{Frame, Encoder, Repeat, SetParameter};

use std::io;
use std::fs::File;
use std::borrow::Cow;

// Main
fn main() {
    
    // Intro/welcome
    println!("Welcome to GIF generator 0.1.0!");
    println!("*******************************\n");

    // Random number:
    let rand_num_test: u32 = rand::thread_rng().gen_range(1, 101);
    println!("Random Number = {}", rand_num_test);

    // Menu loop
    loop {
        println!("Please input a size for width & height: ");
        let mut img_size = String::new();
        io::stdin().read_line(&mut img_size)
            .expect("Failed to read line");
        let img_size: u16 = match img_size.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Image will be {}px X {}px...", img_size, img_size);
        break;
    }
    
    // Output a new gif file
    output_gif(4);
}

fn output_gif(img_size: u16) {

    // Static array:
    let array_1: [u8; 16] = [
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    // Dynamic array:
    let mut array_2: [u8; 16] = Default::default();
    for i in 0..16 {
        let mut rand_num: u8 = rand::thread_rng().gen_range(0, 2);
        println!("RN: {}", rand_num);
        array_2[i] = rand_num as u8;
    }
    println!("\nArray 2: ");
    println!("{:?}", array_2);
    
    // Set a color for earch array index number:
    let color_map = &[
        0xFF, 0xFF, 0xFF, // Color 0
        0xFF, 0x00, 0x00, // Color 1
    ];
    
    let (width, height) = (img_size, img_size);
    let gif_states = [array_2, array_1];

    let mut image = File::create("target/output.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    for state in &gif_states {
        let mut frame = Frame::default();
        frame.width = width;
        frame.height = height;
        frame.buffer = Cow::Borrowed(&*state);
        encoder.write_frame(&frame).unwrap();
    }
}
