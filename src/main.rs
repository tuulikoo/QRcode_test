use qrcode::QrCode;
use qrcode::render::svg;
use std::{env, sync::Mutex};
use std::fs::File;
use std::io::Write;
use sha2::{Sha256, Digest};
use std::path::Path;
use std::fs;

//Run main function with "cargo run <input string>"
//If no input string given, default string used
//Run tests with "cargo test" or "cargo tarpaulin" (no input string)
#[cfg(not(tarpaulin_include))]
fn main() {
    //read from cli
    let input_string = read_args();

    // Encode input string into a QR code
    let code = encode_qr_code(&input_string);

    // Save the QR code as an SVG image and call cli-print function
    save_svg_image(&code, &input_string);
}

fn encode_qr_code(input: &str) -> QrCode {
    QrCode::new(input.as_bytes()).unwrap()
}

//Print function, buffer for testing
fn print_qr_code(qr_code: &QrCode, buffer: &Mutex<Vec<u8>>) {
    let printable = qr_code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    print!("{}", printable);

    // Ignore errors while writing to the buffer
    let _ = buffer.lock().unwrap().write_all(printable.as_bytes());
}


fn save_svg_image(qr_code: &QrCode, input: &str) {
    //make sure img-folder exista
    let img_folder = "img";
    if !Path::new(img_folder).exists() {
        fs::create_dir(img_folder).expect("Unable to create 'img' folder");
    }
    // Generate filename for SVG image with hash-function
    let file_name = format!("img/{}.svg", hash(input));

    // Render the QR code into an SVG image
    let image = qr_code.render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))  //Black for the QR code
        .light_color(svg::Color("#FFFFFF")) // White for the background
        .build();

    // Save the SVG image to the hashed filename
    let mut file = File::create(&file_name).expect("Unable to create file");
    write!(file, "{}", image).expect("Unable to write to file");

    // Print a message to show the input and the filename
    println!("QR code image generated for '{}': {}", input, file_name);
    println!("To view the QR code, open the generated SVG file using a live server.");
    
    //Print QR code to console
    print_qr_code(&qr_code, &Mutex::new(Vec::new()));
}

fn read_args() -> String {
    let args: Vec<String> = env::args().collect();

    // If at least one argument is provided, join all arguments into a single string
    if args.len() >= 2 {
        return args[1..].join(" ");
    }
    // If no arguments, return default string
    #[cfg(not(tarpaulin_include))]
    "--test-threads 1".to_string()
}

fn hash(word: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(word.as_bytes());
    let result = hasher.finalize();

    result
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}


#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use crate::{encode_qr_code, print_qr_code, save_svg_image};
    use super::{hash, read_args};
    use std::fs;

    #[test]
    fn test_hash() {
        let word = String::from("Hello world");
        let result: String = hash(&word);
        assert_eq!(
            result,
            "64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c"
        );
    }

    #[test]
    fn test_empty_hash() {
        let word: String = String::from("");
        let result: String = hash(&word);
        assert_eq!(
            result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
    

    #[test]
    fn test_read_args() {
        let word = read_args();
        assert_eq!(word, "--test-threads 1");
    }

    #[test]
    fn test_save_svg_image() {
        let input = "test_input";
        let qr_code = encode_qr_code(input);

        save_svg_image(&qr_code, input);

        // Verify file is created
        let expected_file_name = format!("img/{}.svg", hash(input));
        assert!(fs::metadata(&expected_file_name).is_ok());
    }

    #[test]
    fn test_print_qr_code_success() {
        let input = "test_input";
        let qr_code = encode_qr_code(input);

        // Create a Mutex to synchronize access to the buffer
        let buffer = Mutex::new(Vec::new());

        // Print QR code to buffer
        print_qr_code(&qr_code, &buffer);

        // Access buffer and check contents
        let content = buffer.lock().unwrap();

        let expected_printable = qr_code
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build();

        // Convert the expected printable to bytes
        let expected_bytes = expected_printable.as_bytes();

        // Assert that the contents of the buffer match the expected printable representation
        assert_eq!(content.as_slice(), expected_bytes);
    }

    
}

