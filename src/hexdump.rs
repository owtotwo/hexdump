use std::fs::File;
use std::path::Path;
use std::io;
use std::io::Read;
use std::error;
use std::fmt;
use std::char;

#[derive(Debug)]
pub struct HexDumpError {
    side: io::Error,
}

impl fmt::Display for HexDumpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "can not identify the file".fmt(f)
    }
}

impl error::Error for HexDumpError {
    fn description(&self) -> &str {
        "failed to dump file"
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(&self.side)
    }
}

impl From<io::Error> for HexDumpError {
    fn from(err: io::Error) -> HexDumpError {
        HexDumpError { side: err }
    }
}


pub fn dump(path: &Path) -> Result<(), HexDumpError> {
    let mut file = try!(File::open(path));
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    let mut line = [0; 16];
    let mut prev_line = line.clone();
    let tail = buffer.len() - 1;
    let mut s = String::new();
    let mut is_omit = false;
    for (count, i) in buffer.iter().enumerate() {
        line[count % 16] = i.clone();
        if count % 16 == 0 {
            s += &format!("{:08x}  ", count);
        } else if count % 8 == 0 {
            s.push_str("  ");
        } else {
            s.push_str(" ");
        }
        s += &format!("{:02x}", i);
        if count % 16 == 15 || count == tail {
            if line == prev_line {
                if !is_omit { println!("*"); }
                is_omit = true;
            } else {
                is_omit = false;
                prev_line = line.clone();
                s += &format!("{:padding$}", "", padding=3*(16-(count%16))-(count%16)/8);
                s += &format!("|{}|", parse_printable_char(&line[0..(count % 16 + 1)]));
                println!("{}", s);
            }
            s = String::new();
        }
    }
    println!("{:08x}", tail + 1);
    Ok(())
}

fn parse_printable_char(arr: &[u8]) -> String {
    let mut content = String::new();
    for i in arr.iter() {
        let c = match char::from_u32(i.clone() as u32) {
            Some(val) => val,
            None => unreachable!(),
        };
        content.push(if c.is_control() { '.' } else { c });
    }
    content
}
