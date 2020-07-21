extern crate image;
extern crate scoped_threadpool;
use std::convert::TryFrom;
use std::time::Instant;

fn main() {
    let img_path = "flower.jpeg";

    invert_colours_threaded(img_path);
    invert_colours(img_path);
}

fn invert_colours_threaded (img_path: &str) {
    let img = image::open(img_path).unwrap().to_rgb();
    let img_width = img.width();
    let img_height = img.height();

    let num_threads = 5;
    let num_threads_u = usize::try_from(num_threads).unwrap();

    let mut pixel_values = img.into_vec();
    let size_chunk = pixel_values.len() / (num_threads_u - 1);

    // Create thread pool of 5 threads, each for a given scope
    let mut pool = scoped_threadpool::Pool::new(num_threads);

    let now = Instant::now();
    
    // Each scoped thread can reference things outside of closure
    pool.scoped(|scope| {
        for slice in pixel_values.chunks_mut(size_chunk) {
            scope.execute(move || {
                for pixel in slice.iter_mut() {
                    *pixel = 255 - *pixel;
                }
            })
        }
    });

    let elapsed = now.elapsed();
    println!("Time elapsed for colour inversion (threaded): {:?}", elapsed);

    let _ = image::save_buffer("inverted_t.png", &pixel_values, img_width, img_height, image::ColorType::Rgb8).unwrap();
}

fn invert_colours(img_path: &str) {
    let img = image::open(img_path).unwrap().to_rgb();
    let img_width = img.width();
    let img_height = img.height();

    let mut pixel_values = img.into_vec();

    let now = Instant::now();

    for i in 0 .. pixel_values.len() {
        let tmp = pixel_values[i];
        pixel_values[i] = 255 - tmp;
    }

    let elapsed = now.elapsed();
    println!("Time elapsed for colour inversion (non-threaded): {:?}", elapsed);

    let _ = image::save_buffer("inverted.png", &pixel_values, img_width, img_height, image::ColorType::Rgb8).unwrap();
}