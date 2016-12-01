use std::fs::File;
use std::path::Path;
use std::io;
use std::char;

use std::io::Read;

/// Dump the file at `filepath` to the hex format string
/// and output it to console.
///
/// # Example
///
/// Basic usage:  
///
/// ```
/// // the file `data` contain 18 bytes "abcefghijklmnopq\n".  
/// dump("data").unwrap();  
///  
/// // the console will show:  
/// //     00000000  61 62 63 64 65 66 67 68  69 6a 6b 6c 6d 6e 6f 70  |abcdefghijklmnop|  
/// //     00000010  71 0a                                             |q.|  
/// //     00000012  
///
/// ```
pub fn dump(filepath: &Path) -> io::Result<()> {
    // read the file to buffer
    let mut file = try!(File::open(filepath));
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // dump the buffer
    let mut line = [0; 16];           // store current line content
    let mut prev_line = line.clone(); // storage previous line content
    let tail = buffer.len() - 1;      // last index for buffer
    let mut s = String::new();        // output string for each line
    let mut has_omitted = false;      // mark if prev line has been omitted

    for (count, i) in buffer.iter().enumerate() {
        // store the bytes for display
        line[count % 16] = i.clone();

        // line head
        if count % 16 == 0 {
            // clear the output string
            s.clear();
            // add the position in file for each line
            s.push_str(&format!("{:08x}  ", count));
        // line middle
        } else if count % 8 == 0 {
            s.push_str("  ");
        // seperate by whitespace
        } else {
            s.push_str(" ");
        }

        // show bytes in upper hex
        s.push_str(&format!("{:02x}", i));

        // line tail
        if count % 16 == 15 || count == tail {
            // omit the line, replace it with "*" if same as the prev line,
            // and if the prev line has been omitted, current line does not
            // display. 
            if line == prev_line {
                if !has_omitted { println!("*"); }
                has_omitted = true;
            // display the content with the printable chars
            } else {
                has_omitted = false;
                // save the prev line
                prev_line = line.clone();
                // padding by whitespace if not enough to 16 chars in one line
                s.push_str(&format!("{:padding$}", "", 
                                    padding=3*(16-(count%16))-(count%16)/8));
                // add contents
                s.push_str(&format!("|{}|",
                                    to_printable_chars(&line[0..(count % 16 + 1)])));
                // print to the console
                println!("{}", s);
            }
        }
    }

    // show the total amount of bytes
    println!("{:08x}", tail + 1);

    Ok(())
}


/// Convert the bytes to printable chars
/// 
/// # Example
///
/// Basic usage:
///
/// ```
/// let arr: Vec<u8> = vec![0x61, 0x62, 0x63, 0x0A, 0x65];
/// assert!(to_printable_chars(&arr[..], String::from("abc.e"));
/// ```
pub fn to_printable_chars(arr: &[u8]) -> String {
    let mut content = String::new();
    for i in arr.iter() {
        let c = match char::from_u32(i.clone() as u32) {
            Some(val) => val,
            None => unreachable!(),
        };
        // regard all control chars as '.'
        content.push(if c.is_control() { '.' } else { c });
    }
    content
}
