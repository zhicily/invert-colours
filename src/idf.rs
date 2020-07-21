extern crate image;
use std::sync::{Arc, Mutex};
use std::convert::TryFrom;
use std::thread;
use rayon::prelude::*;

fn main() {
    let img_path = "flower.jpeg";

    let img = image::open(img_path).unwrap();
    let mut img = img.to_rgb();
    
    let img_width = img.width();
    let img_height = img.height();

    println!("width: {} height: {}", img_width, img_height);

    let num_threads = 5;
    let rows_per_thread = img_height / num_threads;
    let remainder_rows = img_height % num_threads;

    println!("rows per thread: {} rem: {}", rows_per_thread, remainder_rows);

    let mut boundaries = vec![];
    let rows_per_thread_u = usize::try_from(rows_per_thread).unwrap();
    let num_threads_u = usize::try_from(num_threads).unwrap();
    
    for i in (0 .. img_height).step_by(rows_per_thread_u) {
        boundaries.push((i, i + rows_per_thread));
    }

    println!("{:?}", boundaries);

    let mut threads = vec![];

    let mut pix = img.into_vec();
    let size_chunk = pix.len() / (num_threads_u - 1);

    let mut chunks = vec![];
    
    for c in pix.chunks_mut(size_chunk) {
        chunks.push(c.to_owned());
    }

    for chunk in chunks {
        threads.push(thread::spawn(move || {
            for rgb_val in chunk {
                let mut new = rgb_val;
                new = 255 - rgb_val;
            }
        }))
    }

    // pix.par_chunks_mut(num_threads_u).for_each(|c| {

    //     threads.lock().unwrap().push(thread::spawn( || {
    //         for pixel in c {

    //         }
    //     }))
    // });

    // for i in 0 .. num_threads_u {
    //     let 
    //     let boundary = boundaries[i];

    //     threads.push(thread::spawn(move || {
    //         println!("this is thread number {:?}", boundary);
    //         for r in boundary.0 .. boundary.1 {
    //             for c in 0 .. img_width {
    //                 invert_pixel(img.get_pixel_mut(c, r));
    //             }
    //         }
    //     }))
    // }

    for t in threads {
        let _ = t.join();
    }


    // for pixel in img.pixels_mut() {
    //     invert_pixel(pixel);
    // }

    // img.save("inverted.png").unwrap();
}

fn invert_pixel(pixel: &mut image::Rgb<u8>) {
    (*pixel)[0] = 255 - (*pixel)[0];
    (*pixel)[1] = 255 - (*pixel)[1];
    (*pixel)[2] = 255 - (*pixel)[2];
}

// impl Copy for image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {}