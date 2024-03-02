use qrcode::QrCode;
use image::Luma;
use sha2::{Sha256, Digest};
use std::{env, fs, sync::Mutex};
use image::DynamicImage;
use std::io::Write;

// Handle command line arguments
fn get_input_string(args: &[String]) -> String {
    let mut args = args.to_vec(); // Clone the arguments to modify

    if let Some(index) = args.iter().position(|s| s == "--png") {
        args.remove(index);
    }

    args[1..].join(" ")
}

// Encode input string into a QR code
fn encode_qr_code(input_string: &str) -> QrCode {
    QrCode::new(input_string.as_bytes()).unwrap()
}

// Render QR code into an image.
fn render_qr_code_image(code: &QrCode) -> DynamicImage {
    code.render::<Luma<u8>>().build().into()
}

// Function to save image in the "img" folder
fn save_image(image: &DynamicImage, input: &str) {
    let args: Vec<String> = env::args().collect();
    let png_flag_set = args.contains(&String::from("--png")) || env::var("FORCE_SAVE_PNG").is_ok();

    if png_flag_set {
        if !fs::metadata("img").is_ok() {
            fs::create_dir("img").expect("Unable to create 'img' folder");
        }

        let file_name = format!("img/{}.png", hash(input));
        image.save(&file_name).expect("Error saving image");

        println!("Image saved as: {}", file_name);
    }
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

fn print_qr_code(qr_code: &QrCode, buffer: &Mutex<Vec<u8>>) {
    let printable = qr_code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    print!("{}", printable);

    // Ignore errors
    let _ = buffer.lock().unwrap().write_all(printable.as_bytes());
}

fn main_workflow(input_string: &str) {
    let code = encode_qr_code(input_string);
    let image = render_qr_code_image(&code);
    save_image(&image, input_string);
    print_qr_code(&code, &Mutex::new(Vec::new()));
}
//exclude main-function from tarpaulin
#[cfg(not(tarpaulin_include))]
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_string = get_input_string(&args);
    main_workflow(&input_string);
}


#[cfg(test)]
mod tests {
    use image::GenericImageView;

    use super::*;
    use std::sync::Mutex;
    use std::fs::{self};
    use std::path::Path;

    #[test]
    fn test_hash() {
        let word = "Hello world";
        let result = hash(word);
        assert_eq!(
            result,
            "64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c"
        );
    }

    #[test]
    fn test_empty_hash() {
        let word = "";
        let result = hash(word);
        assert_eq!(
            result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_get_input_string() {
        let args = vec!["program_name".to_string(), "input_string".to_string()];
        assert_eq!(get_input_string(&args), "input_string");

        let args_with_option = vec!["program_name".to_string(), "input_string".to_string(), "--png".to_string()];
        assert_eq!(get_input_string(&args_with_option), "input_string");
    }
    #[test]
    fn test_get_input_string_with_png_flag() {
        let args = vec!["".to_string(), "".to_string()];
        let result = get_input_string(&args);
        assert_eq!(result, "");
    }
  
    #[test]
    fn test_render_qr_code_image_dimensions() {
        let data = "Hello, QR Code!";
        let qr = QrCode::new(data.as_bytes()).expect("Failed to create QR code");
        let image = render_qr_code_image(&qr);
        let dimensions = image.dimensions();
        assert!(dimensions.0 > 0 && dimensions.1 > 0, "Image has invalid dimensions.");

    }

    #[test]
    fn test_render_qr_code_image_content() {
        let data = "Check Content";
        let qr = QrCode::new(data.as_bytes()).unwrap();
        let image = render_qr_code_image(&qr).to_luma8();
        let white_pixel = Luma([255u8]);
        let black_pixel = Luma([0u8]);
        let all_white = image.pixels().all(|&p| p == white_pixel);
        let all_black = image.pixels().all(|&p| p == black_pixel);
        assert!(!all_white, "Image is all white, indicating an empty QR code.");
        assert!(!all_black, "Image is all black, which is unlikely for a valid QR code.");
    }
    
    #[test]
    fn test_cli_command_simulation() {
        std::env::set_var("FORCE_SAVE_PNG", "1");
    
        let img_dir = Path::new("img");
    
        if img_dir.exists() {
            fs::remove_dir_all(img_dir).expect("Failed to delete existing img directory");
        }
    
        let input_string = "testsaving";
        main_workflow(input_string);
    
        let expected_file_name = "2cc82650f81adda284cb84078a9403ec640690586bb66ce7489ba429328344a0.png";
        let file_path = img_dir.join(expected_file_name);
    
        // Wait for the file to be created
        let mut attempts = 0;
        while !file_path.exists() && attempts < 10 {
            std::thread::sleep(std::time::Duration::from_millis(100));
            attempts += 1;
        }
    
        // Check if the expected image file was created
        assert!(file_path.exists(), "The expected image file was not created.");
    
        // Cleanup
        std::env::remove_var("FORCE_SAVE_PNG");
        fs::remove_file(&file_path).expect("Failed to delete the test image file.");

    }


    #[test]
    fn test_print_qr_code() {
        let qr_code = encode_qr_code("Print QR");
        let buffer = Mutex::new(Vec::new());
        print_qr_code(&qr_code, &buffer);

        let buffer_lock = buffer.lock().unwrap();
        let output = String::from_utf8_lossy(&buffer_lock);
        assert!(!output.is_empty());
    }

    #[test]
    fn test_print_qr_code_success() {
        let input = "test_input";
        let qr_code = encode_qr_code(input);
        let buffer = Mutex::new(Vec::new());
        print_qr_code(&qr_code, &buffer);

        let content = buffer.lock().unwrap();

        let expected_printable = qr_code
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build();

        let expected_bytes = expected_printable.as_bytes();

        assert_eq!(content.as_slice(), expected_bytes);
    }
}
