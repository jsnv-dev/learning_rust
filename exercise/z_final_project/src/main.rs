use clap::{App, Arg};
// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let matches = App::new("Image editor")
        .version("1.0.0")
        .author("Jason Villaluna")
        .about("Create modifications on image")
        .arg(
            Arg::new("input")
                .long("input")
                .value_name("INPUT_FILE")
                .takes_value(true),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .value_name("OUTPUT_FILE")
                .takes_value(true),
        )
        .arg(
            Arg::new("blur")
                .long("blur")
                .value_name("BLUR_AMOUNT")
                .takes_value(true),
        )
        .arg(
            Arg::new("brighten")
                .long("brighten")
                .value_name("BRIGHTEN_AMOUNT")
                .takes_value(true),
        )
        .arg(
            Arg::new("crop")
                .long("crop")
                .value_name("CROP_DIMENSIONS")
                .takes_value(true),
        )
        .arg(
            Arg::new("rotate")
                .long("rotate")
                .value_name("ROTATE_VALE")
                .takes_value(true),
        )
        .arg(
            Arg::new("generate")
                .long("generate")
                .value_name("RGB_VALUES")
                .takes_value(true),
        )
        .arg(Arg::new("invert").long("invert").takes_value(false))
        .arg(Arg::new("grayscale").long("grayscale").takes_value(false))
        .arg(Arg::new("fractal").long("fractal").takes_value(false))
        .get_matches();

    let mut infile = match matches.value_of("input") {
        Some(x) => x,
        None => "",
    };
    let outfile = matches.value_of("output").unwrap();

    let args = [
        "blur",
        "brighten",
        "crop",
        "rotate",
        "invert",
        "grayscale",
        "generate",
        "fractal",
    ];
    for arg in args {
        // skips the argument if not used
        if !matches.is_present(arg) {
            continue;
        }

        match matches.value_of(arg) {
            Some(amount) => match arg {
                "blur" => blur(&infile, &outfile, &amount),
                "brighten" => brighten(&infile, &outfile, &amount),
                "crop" => crop(&infile, &outfile, &amount),
                "rotate" => rotate(&infile, &outfile, &amount),
                "generate" => generate(&outfile, &amount),
                _ => return,
            },
            None => match arg {
                "invert" => invert(&infile, &outfile),
                "grayscale" => grayscale(&infile, &outfile),
                "fractal" => fractal(&outfile),
                _ => return,
            },
        }
        infile = outfile;
    }
}

fn blur(infile: &str, outfile: &str, amount: &str) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let amount: f32 = amount.parse().expect("Failed to parse blur number");
    let img2 = img.blur(amount);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: &str, outfile: &str, amount: &str) {
    // See blur() for an example of how to open / save an image.

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    let img = image::open(infile).expect("Failed to open INFILE.");
    let amount: i32 = amount.parse().expect("Failed to parse brighten amount");
    let img2 = img.brighten(amount);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: &str, outfile: &str, values: &str) {
    // See blur() for an example of how to open an image.

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    let split_values: Vec<u32> = values
        .split_whitespace()
        .map(|x| x.parse::<u32>().expect("Failed to parse brighten amount"))
        .collect::<Vec<u32>>();

    let img2 = img.crop(
        split_values[0],
        split_values[1],
        split_values[2],
        split_values[3],
    );
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: &str, outfile: &str, amount: &str) {
    // See blur() for an example of how to open an image.

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    let amount: i32 = amount.parse().expect("Failed to parse brighten amount");
    let img2 = match amount {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => return,
    };
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: &str, outfile: &str) {
    // See blur() for an example of how to open an image.

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: &str, outfile: &str) {
    // See blur() for an example of how to open an image.

    // .grayscale() takes no arguments. It returns a new image.

    // See blur() for an example of how to save the image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.grayscale();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: &str, values: &str) {
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
    let width = 800;
    let height = 800;
    let mut imgbuf = image::ImageBuffer::new(width, height);
    let split_values: Vec<u8> = values
        .split_whitespace()
        .map(|x| x.parse::<u8>().expect("Failed to parse brighten amount"))
        .collect::<Vec<u8>>();
    let rgb: [_; 3] = [split_values[0], split_values[1], split_values[2]];

    // Iterate over the coordinates and pixels of the image
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb(rgb);
    }

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: &str) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
// NOTE: This is implemented, sample usage:
//  cargo run --release -- --output out.png --input dyson.png --invert --brighten 100 --grayscale --rotate 90
