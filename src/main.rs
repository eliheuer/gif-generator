//View image with: mtpaint -v target/output.gif &> /dev/null

extern crate rand;
extern crate gif;
use rand::Rng;
use gif::{Frame, Encoder, Repeat, SetParameter};
use std::io;
use std::fs::File;
use std::borrow::Cow;

fn main() {

    // Intro/welcome:
    println!("\n\n\n\nWelcome to GIF generator 0.1.0!");
    println!("*******************************\n");
    
    // TODO figure out how type casting works in rust:
    // Menu loop:
    loop {
        println!("Please input an image size:");
        // let mut img_size_test = String::new();
        // io::stdin().read_line(&mut img_size_test)
        //     .expect("Failed to read line");
        // let my_int: u16 = img_size_test.parse().unwrap();
        // println!("img_size_test{}",img_size_test);
        break;
    }

    // Output a new gif file:
    output_gif(16);
}

// Output gif based on function arguments:
fn output_gif(img_size: u16) {

    println!("img_size: {}", img_size);

    // Static array://///////////////////////////////////////////////
    let _array_1: [u8; 64] = [
        1, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 1,
    ];
 
    // Static array://///////////////////////////////////////////////
    let eli: [u8; 256] = [
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 4, 4, 5, 5, 5, 5, 4, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 4, 4, 5, 5, 5, 5, 4, 4, 4, 3, 3, 3,
        3, 3, 3, 3, 4, 5, 5, 5, 5, 5, 5, 4, 4, 3, 3, 3,
        3, 3, 3, 3, 4, 5, 5, 5, 5, 5, 5, 4, 4, 3, 3, 3,
        3, 3, 3, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3, 3, 3, 3,
        3, 3, 3, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3, 3, 3, 3,
        3, 3, 3, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3, 3, 3, 3,
        3, 3, 3, 3, 1, 5, 5, 5, 5, 5, 5, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 1, 5, 5, 5, 5, 5, 5, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 1, 1, 5, 5, 5, 5, 4, 3, 3, 3, 3, 3,
        1, 1, 1, 1, 1, 1, 5, 5, 5, 5, 3, 3, 3, 3, 3, 3,
        1, 1, 1, 1, 2, 2, 5, 5, 5, 5, 2, 2, 3, 3, 3, 3,
        1, 1, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3,
        1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3,
    ];


    // Initialize random array://////////////////////////////////////
    let mut rand_col: [u8; 256];
    let mut rand_col = [0; 256];

    // Iterate over array, add random pixels: 
    for i in 0..256 {
        let rand_num: u8 = rand::thread_rng().gen_range(2, 9);
        print!("{}", rand_num);
        rand_col[i] = rand_num as u8;
    }

    // Initialize array A:///////////////////////////////////////////
    let mut ary_a: [u8; 256];
    let mut ary_a = [0; 256];

    // Iterate over array: 
    for i in 0..256 { 
        let rand_num: u8 = rand::thread_rng().gen_range(0, 9);
        if i % 3 == 0 {
            ary_a[i] = 6 as u8;
        } else if i % 3 != 0 {
            ary_a[i] = rand_num as u8;          
        } else {
            ary_a[i] = 0 as u8;
        }
    } 

    // Initialize array B:///////////////////////////////////////////
    let mut ary_b: [u8; 256];
    let mut ary_b = [0; 256];

    // Iterate over array: 
    let mut count = 3;
    for i in 0..256 { 
        let rand_num: u8 = rand::thread_rng().gen_range(1, 3);
        
        if i == 0 { 
            ary_b[i] = count as u8;
        } else if i % 3 == 0 {
            ary_b[i] = count as u8;
        } else {
            ary_b[i] = rand_num as u8;
        }

        if i % 32 == 0 {
            count += 1;
        }
    }

    // Define colors for map index
    let col_map = &[
        0xDD, 0xDD, 0xDD, // Color 0: White
        0x55, 0x55, 0x55, // Color 1: Grey
        0x40, 0x40, 0x40, // Color 2: Dark Grey
        0xF0, 0x28, 0x28, // Color 3: Red
        0xFF, 0x80, 0x00, // Color 4: Orange
        0xF4, 0xD0, 0x3F, // Color 5: Yellow
        0x27, 0xAE, 0x60, // Color 6: Green
        0x34, 0x98, 0xDB, // Color 7: Blue
        0x9B, 0x59, 0xB6, // Color 8: Purple
        0xF0, 0x28, 0x28, // Color 3: Red
        0xFF, 0x80, 0x00, // Color 4: Orange
        0xF4, 0xD0, 0x3F, // Color 5: Yellow
        0x27, 0xAE, 0x60, // Color 6: Green
        0x34, 0x98, 0xDB, // Color 7: Blue
        0x9B, 0x59, 0xB6, // Color 8: Purple
    ];
    
    // Make output file
    let (width, height) = (img_size, img_size);
    let gif_states = [eli, eli, rand_col];
    let mut image = File::create("target/output.gif").unwrap();
    let mut encoder = 
        Encoder::new(&mut image, width, height, col_map).unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    for state in &gif_states {
        let mut frame = Frame::default();
        frame.width = width;
        frame.height = height;
        frame.buffer = Cow::Borrowed(&*state);
        encoder.write_frame(&frame).unwrap();
    }
}
