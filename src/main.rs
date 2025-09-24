// A simple app to format the size of a file
use std::env;
// An enum to represent the size of a file
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}
impl FileSize {
    fn format_size(size: u64) -> String {
        let filesize = match size {
            0..=999 => FileSize::Bytes(size),
            1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
            1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
            _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
        };

        match filesize {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
            FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
        }
    }
}

// A struct to hold the size and its formatted string
#[warn(dead_code)]
struct FileSizeFormatter {
    // size: u64,
    format: String,
}

impl FileSizeFormatter {
    fn new(size: u64) -> Self {
        let format = FileSize::format_size(size);
        FileSizeFormatter { format }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command line arguments

    // Check if the user provided a size argument
    if args.len() < 2 {
        panic!("Please provide the size of the file as a command line argument");
    }

    // Trim the input and parse it to a u64
    let size: u64 = match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Please enter a valid number");
        }
    };

    // Create a FileSizeFormatter instance and print the formatted size
    let formatter = FileSizeFormatter::new(size);
    println!("{}", formatter.format)
}
