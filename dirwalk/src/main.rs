use std::fs::read_dir;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

fn get_lines_in_file(path: &Path, linecount: &mut i64) {
    if let Ok(lines) = read_lines(path.as_os_str()) {
        (*linecount) += lines.count() as i64;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn wal_dir(path: &Path, count: &mut i64, linescount: &mut i64) {
    let dirobj = match read_dir(path) {
        Ok(v) => v,
        Err(e) => {
            println!("Path not found: {}", e);
            exit(1);
        }
    };

    let it = dirobj.into_iter();

    for x in it {
        match x {
            Ok(v) => {
                if v.path().is_dir() {
                    wal_dir(v.path().as_path(), count, linescount)
                } else {
                    (*count) += 1;
                    let previouscout = *linescount;
                    get_lines_in_file(v.path().as_path(), linescount);
                    print!("{}", v.path().as_path().display());
                    println!(" - No of lines in this file: {}", (*linescount) - previouscout)
                }
            }
            Err(e) => {
                println!("Path not found: {}", e);
                exit(1);
            }
        }
    }
}

fn main() {
    let mut count: i64 = 0;
    let mut linescount: i64 = 0;
    //let root = Path::new("/home/anand/CDS");
    let root = Path::new("/mnt/60114826-61c3-4ad9-af5c-40136414e188/Projects/CDS/dirwalk");
    wal_dir(root, &mut count, &mut linescount);
    println!("Total no of file found {}", count);
    println!("Total no of lines found {}", linescount);
}
