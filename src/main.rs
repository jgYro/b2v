use clap::Parser;
use image::{ImageBuffer, Rgb};
use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;
use std::cmp;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    // File to analyze
    #[arg(short, long)]
    file: String,

    // Name of output file
    #[arg(short, long)]
    output: String,
}

fn main() -> io::Result<()> {
    // Read in user arguments
    let args = Args::parse();

    // Assign user path to variable
    let mut file = File::open(args.file)?;

    // Create a buffer to hold file contents
    let mut buffer = Vec::new();

    // Read the file's contents into the buffer
    file.read_to_end(&mut buffer)?;

    let imgx = 256u32;
    let imgy = 256u32;

    // Initialize a hashmap to count the occurrences of each coordinate
    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();

    // Count occurrences of coordinates to better show patterns
    for i in 0..buffer.len() - 1 {
        let x = buffer[i] as u32 % imgx;
        let y = buffer[i + 1] as u32 % imgy;
        *counts.entry((x, y)).or_insert(0) += 1;
    }

    // Create a new image
    let mut img = ImageBuffer::new(imgx, imgy);

    // Determine the maximum hit count to scale the colors appropriately
    let &max_count = counts.values().max().unwrap_or(&1);

    // Apply a logarithmic scale for intensity based on hit counts
    for ((x, y), &count) in &counts {
        let scaled_intensity = if count > 0 {
            let log_scale = (255.0 * (count as f32).log10() / (max_count as f32).log10()) as u8;
            cmp::max(log_scale, 1) // Ensure minimum visibility for any counted hit
        } else {
            0
        };
        // Set the pixel to a shade of gray based on the scaled intensity
        img.put_pixel(*x, *y, Rgb([scaled_intensity, scaled_intensity, scaled_intensity]));
    }

    let output = format!("{}.png", args.output);

    // Save the image
    if let Err(e) = img.save(output) {
        eprintln!("Failed to save the image: {}", e);
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to save image"));
    }

    Ok(())
}
