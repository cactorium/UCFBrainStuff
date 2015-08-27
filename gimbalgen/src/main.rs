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
const base_height: f64 = 77.81;

const base_hmargin: f64 = 20.00;

const bearing_tolerance: f64 = 1.00;
const r_margin: f64 = 2.50;

/// Measurements
const stick_to_bottom: f64 = 36.79;
const raised_dia: f64 = 61.40;
const outer_stick_dia: f64 = 42.53;
const lower_stick_dia: f64 = 19.32;

const rod_dia: f64 = 4.84;
const tab_margin: f64 = 1.20;

/// Servo measurements
const servo_shaft_h: f64 = 14.50;
const servo_body_width: f64 = 22.50;
const shaft_to_edge: f64 = 5.50;
const flange_to_end: f64 = 21.00;
const flange_len: f64 = 5.00;

struct Point {
    x: f64,
    y: f64
}
fn pt(x: f64, y: f64) -> Point {
    Point{x: x, y: y}
}

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

fn f64str(arg: f64) -> String { f64_to_string(arg) }

fn write_root<W: Write, F: FnOnce(&mut XmlWriter<W>)>(writer: &mut XmlWriter<W>, f: F) {
    writer.begin_elem("svg").unwrap();
    let width_in: String = f64_to_string(width) + "in";
    let height_in: String = f64_to_string(height) + "in";
    writer.attr("width", &width_in).unwrap();
    writer.attr("height", &height_in).unwrap();
    writer.attr("version", "1.1").unwrap();
    writer.attr("xmlns", "http://www.w3.org/2000/svg").unwrap();
    let view_box: String = "0 0 ".to_string() + &f64str(width*in_to_mm) + " " + 
                &f64str(height*in_to_mm);
    writer.attr("viewBox", &view_box).unwrap();
    f(writer);
    writer.close().unwrap();
    writer.flush().unwrap();
}

fn cut_style<W: Write>(writer: &mut XmlWriter<W>) {
    writer.attr("stroke", "red").unwrap();
    writer.attr("fill", "transparent").unwrap();
    writer.attr("fill-opacity", "0.0").unwrap();
    writer.attr("stroke-width", "0.001in").unwrap();
}

fn write_base<W: Write>(writer: &mut XmlWriter<W>, offset: Point) {
    // Outer outline
    {
        writer.begin_elem("path").unwrap();
        cut_style(writer);
        // Four main sides
        let mut path = 
            "M".to_string() + &f64str(offset.x - base_width/2.0 - base_hmargin) +
                "," + &f64str(offset.y) + 
            " L" + &f64str(offset.x - base_width/2.0 - base_hmargin) + "," +
                &f64str(offset.y + base_height) +
            " L" + &f64str(offset.x + base_width/2.0 + base_hmargin) + "," +
                &f64str(offset.y + base_height) +
            " L" + &f64str(offset.x + base_width/2.0 + base_hmargin) + "," +
                &f64str(offset.y);
        if stick_to_bottom - 0.5*raised_dia - r_margin - thickness - servo_shaft_h > 0.0 {
            // panic!("write_base: Didn't solve for this case, sorry!");
            path = path + " L" + &f64str(offset.x + shaft_to_edge + flange_len) + "," +
                &f64str(offset.y) +
            " L" + &f64str(offset.x + shaft_to_edge + flange_len) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - bearing_tolerance - servo_shaft_h - flange_to_end) +
            " L" + &f64str(offset.x + shaft_to_edge - servo_body_width - flange_len) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - bearing_tolerance - servo_shaft_h - flange_to_end) +
            " L" + &f64str(offset.x + shaft_to_edge - servo_body_width - flange_len) + "," +
                &f64str(offset.y) +
            " Z";
            writer.attr("d", &path).unwrap();
            writer.end_elem().unwrap();

            writer.begin_elem("rect").unwrap();
            cut_style(writer);
            writer.attr("x", &f64str(offset.x + shaft_to_edge));
            writer.attr("y", &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - bearing_tolerance - thickness));
            writer.attr("width", &f64str(flange_len));
            writer.attr("height", &f64str(thickness));
            writer.end_elem().unwrap();

            writer.begin_elem("rect").unwrap();
            cut_style(writer);
            writer.attr("x", &f64str(offset.x + shaft_to_edge - servo_body_width - flange_len));
            writer.attr("y", &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - bearing_tolerance - thickness));
            writer.attr("width", &f64str(flange_len));
            writer.attr("height", &f64str(thickness));
            writer.end_elem().unwrap();
        } else {
            path = path + " L" + &f64str(offset.x + shaft_to_edge + flange_len) + "," +
                &f64str(offset.y) +
            " L" + &f64str(offset.x + shaft_to_edge + flange_len) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - servo_shaft_h) +
            " L" + &f64str(offset.x + shaft_to_edge) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - servo_shaft_h) +
            " L" + &f64str(offset.x + shaft_to_edge) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - servo_shaft_h - thickness) +
            " L" + &f64str(offset.x + shaft_to_edge + flange_len) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - servo_shaft_h - thickness) +
            " L" + &f64str(offset.x + shaft_to_edge + flange_len) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - servo_shaft_h - flange_to_end) +
            " L" + &f64str(offset.x + shaft_to_edge - servo_body_width - flange_len) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - servo_shaft_h - flange_to_end) +
            " L" + &f64str(offset.x + shaft_to_edge - servo_body_width - flange_len) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - servo_shaft_h - thickness) +
            " L" + &f64str(offset.x + shaft_to_edge - servo_body_width) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - servo_shaft_h - thickness) +
            " L" + &f64str(offset.x + shaft_to_edge - servo_body_width) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - servo_shaft_h) +
            " L" + &f64str(offset.x + shaft_to_edge - servo_body_width - flange_len) + "," +
                &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - servo_shaft_h) +
            " L" + &f64str(offset.x + shaft_to_edge - servo_body_width - flange_len) + "," +
                &f64str(offset.y);
            path = path + " Z";
            writer.attr("d", &path).unwrap();
            writer.end_elem().unwrap();
        }
    }

    let tab_width = rod_dia + 2.*r_margin - 2.*tab_margin;
    // Inner outline; fuck it, let's draw a rectange and a circle
    {
        writer.begin_elem("rect").unwrap();
        cut_style(writer);
        writer.attr("x", &f64str(offset.x - 0.5*tab_width));
        writer.attr("y", &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness));
        writer.attr("width", &f64str(tab_width));
        writer.attr("height", &f64str(raised_dia + 2.*r_margin + 2.*thickness));
        writer.end_elem().unwrap();

        writer.begin_elem("circle").unwrap();
        cut_style(writer);
        writer.attr("cx", &f64str(offset.x));
        writer.attr("cy", &f64str(offset.y + stick_to_bottom));
        writer.attr("r", &f64str(0.5*raised_dia - r_margin));
        writer.end_elem().unwrap();
    }

    // Tab for the upper bearing
    {
        writer.begin_elem("rect").unwrap();
        cut_style(writer);
        writer.attr("x", &f64str(offset.x - 0.5*tab_width));
        writer.attr("y", &f64str(offset.y + stick_to_bottom + 0.5*raised_dia + r_margin + thickness + bearing_tolerance));
        writer.attr("width", &f64str(tab_width));
        writer.attr("height", &f64str(thickness));
        writer.end_elem().unwrap();
    }
}

fn base_bearing<W: Write>(writer: &mut XmlWriter<W>, offset: Point) {
}

fn motor_bearing<W: Write>(writer: &mut XmlWriter<W>, offset: Point) {
}

fn main() {
    println!("Running gimbal design generator!");
    let mut out = File::create("test.svg").unwrap();
    out.write(b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n").unwrap();
    let mut xml_out = XmlWriter::new(out);

    write_root(&mut xml_out, |xmlout| {
        write_base(xmlout, pt(width * in_to_mm/2.0, base_height));
    });
}
