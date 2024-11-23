use std::fs::File;
use std::io::{BufRead, BufReader, Error};

// struct MemoryInfo {
//     total: u64,
//     in_use: u64,
//     available: u64,
// }


fn read_data() -> Result<Vec<String>, Error> {
    let file = match File::open("/proc/meminfo") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err);
            return Err(err);
        }
    };

    BufReader::new(file)
        .lines()
        .take(3)
        .collect()
}

fn main() {
    match read_data() {
        Ok(lines) => println!("{:?}", lines),
        Err(err) => println!("{}", err)
    };
}
