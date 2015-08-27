#![allow(non_upper_case_globals)]
extern crate xml_writer;

use xml_writer::XmlWriter;
use std::fs::File;
use std::io::Write;

const in_to_mm: f64 = 25.4;
const mm_to_in: f64 = 1.00/25.4;

// NOTE: Unlike the rest of the measurements here, width and height are in 
// inches
const width: f64 = 8.00;
const height: f64 = 24.00;
const thickness: f64 = 3.40;

/// The size of the base
const base_width: f64 = 81.81;
const base_height: f64 = 67.62;

/// Diameter of the raised section surrounding the joystick
const outer_dia: f64 = 61.40;

fn digit_to_char(digit: i64) -> char {
    match digit {
    0 => '0',
    1 => '1',
    2 => '2',
    3 => '3',
    4 => '4',
    5 => '5',
    6 => '6',
    7 => '7',
    8 => '8',
    9 => '9',
    _ => panic!(format!("Invalid digit found: {}", digit))
    }
}

fn f64_to_string(arg: f64) -> String {
    let mut result = String::new();
    let mut biggest = 1;
    let mut f = arg;
    if f < 0.0 {
        let negresult = f64_to_string(-f);
        return "-".to_string() + &negresult;
    }
    while f > (biggest as f64) {
        biggest = biggest * 10;
    }
    if biggest != 1 {
        biggest = biggest / 10;
        while biggest >= 1 {
            let digit = (f / (biggest as f64)).floor() as i64;
            result.push(digit_to_char(digit));
            f = f - (digit as f64) * (biggest as f64);
            biggest = biggest / 10;
        }
    } else {
        result.push('0');
    }
    result.push('.');
    f = f * 10.0;
    for _ in 0..3 {
        let digit = f.floor() as i64;
        // println!("{}, {}", f, digit);
        result.push(digit_to_char(digit));
        f = (f - (digit as f64)) * 10.0;
    }
    result
}

fn write_root<W: Write, F: FnOnce(&mut XmlWriter<W>)>(writer: &mut XmlWriter<W>, f: F) {
    writer.begin_elem("svg").unwrap();
    let width_in: String = f64_to_string(width) + "in";
    let height_in: String = f64_to_string(height) + "in";
    writer.attr("width", &width_in).unwrap();
    writer.attr("height", &height_in).unwrap();
    writer.attr("version", "1.1").unwrap();
    writer.attr("xmlns", "http://www.w3.org/2000/svg").unwrap();
    let view_box: String = "0 0 ".to_string() + &f64_to_string(width*in_to_mm) + " " + 
                &f64_to_string(height*in_to_mm);
    writer.attr("viewBox", &view_box).unwrap();
    f(writer);
    writer.close().unwrap();
    writer.flush().unwrap();
}

fn main() {
    println!("Running gimbal design generator!");
    let mut out = File::create("test.svg").unwrap();
    out.write(b"<?xml version=\"1.0\" standalone=\"no\"?>\n").unwrap();
    let mut xml_out = XmlWriter::new(out);

    write_root(&mut xml_out, |_| {});
}
