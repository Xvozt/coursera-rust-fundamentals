enum FileSize {
    Bytes(f64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

impl FileSize {
    fn format(&self) -> String {
        match self {
            FileSize::Bytes(val) => format!("{:.2} B", val),
            FileSize::Kilobytes(val) => format!("{:.2} KB", val),
            FileSize::Megabytes(val) => format!("{:.2} MB", val),
            FileSize::Gigabytes(val) => format!("{:.2} GB", val),
            FileSize::Terabytes(val) => format!("{:.2} TB", val),
        }
    }
}

fn closest_representation(bytes: u64) -> FileSize {
    let bytes_f64 = bytes as f64;

    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    if bytes_f64 >= TB {
        FileSize::Terabytes(bytes_f64 / TB)
    } else if bytes_f64 >= GB {
        FileSize::Gigabytes(bytes_f64 / GB)
    } else if bytes_f64 >= MB {
        FileSize::Megabytes(bytes_f64 / MB)
    } else if bytes_f64 >= KB {
        FileSize::Kilobytes(bytes_f64 / KB)
    } else {
        FileSize::Bytes(bytes_f64)
    }
}

// fn format_size(size: u64) -> String {
//     let filesize = match size {
//         0..=999 => FileSize::Bytes(size),
//         1000..=999_999 => FileSize::Kilobytes(size / 1024),
//         1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
//         1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size / 1_000_000_000),
//         _ => FileSize::Terabytes(size / 1_000_000_000_000),
//     };

//     match filesize {
//         FileSize::Bytes(bytes) => format!("{} bytes", bytes),
//         FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0)),
//         FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0)),
//         FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0)),
//         FileSize::Terabytes(tb) => format!("{:.2} TB", tb as f64 / 1000.0)),
//     }
// }

fn main() {
    // let result = format_size(2500);
    // println!("{}", result);

    let sizes = vec![
        2500,
        999,
        2_500_000,
        8_500_000_000,
        999_999_999_999,
        5_000_000_000_000,
    ];

    for size in sizes {
        let result = closest_representation(size);
        println!("{} bytes => {}", size, result.format())
    }
}
