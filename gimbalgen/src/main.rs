#![allow(non_upper_case_globals)]
extern crate xml_writer;

use xml_writer::XmlWriter;
use std::fs::File;
use std::io::Write;
use std::f64::consts::PI;

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
const axis_h: f64 = 10.00;

const strut_tab_width: f64 = 14.00;

const azi_r_inner: f64 = 0.5*base_width + r_margin;
const azi_r_outer: f64 = azi_r_inner + flange_to_end + thickness;
const azi_r_margin: f64 = 3.50;
const azi_angle: f64 = 45.00;

/// Servo measurements
const servo_shaft_h: f64 = 14.50;
const servo_body_width: f64 = 22.50;
const shaft_to_edge: f64 = 5.50;
const flange_to_end: f64 = 21.00;
const flange_len: f64 = 5.00;
const servo_thickness: f64 = 11.00;

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
            writer.attr("x", &f64str(offset.x + shaft_to_edge)).unwrap();
            writer.attr("y", &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - bearing_tolerance - thickness)).unwrap();
            writer.attr("width", &f64str(flange_len)).unwrap();
            writer.attr("height", &f64str(thickness)).unwrap();
            writer.end_elem().unwrap();

            writer.begin_elem("rect").unwrap();
            cut_style(writer);
            writer.attr("x", &f64str(offset.x + shaft_to_edge - servo_body_width - flange_len)).unwrap();
            writer.attr("y", &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness - bearing_tolerance - thickness)).unwrap();
            writer.attr("width", &f64str(flange_len)).unwrap();
            writer.attr("height", &f64str(thickness)).unwrap();
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
        writer.attr("x", &f64str(offset.x - 0.5*tab_width)).unwrap();
        writer.attr("y", &f64str(offset.y + stick_to_bottom - 0.5*raised_dia - r_margin - thickness)).unwrap();
        writer.attr("width", &f64str(tab_width)).unwrap();
        writer.attr("height", &f64str(raised_dia + 2.*r_margin + 2.*thickness)).unwrap();
        writer.end_elem().unwrap();

        writer.begin_elem("circle").unwrap();
        cut_style(writer);
        writer.attr("cx", &f64str(offset.x)).unwrap();
        writer.attr("cy", &f64str(offset.y + stick_to_bottom)).unwrap();
        writer.attr("r", &f64str(0.5*raised_dia - r_margin)).unwrap();
        writer.end_elem().unwrap();
    }

    // Tab for the upper bearing
    {
        writer.begin_elem("rect").unwrap();
        cut_style(writer);
        writer.attr("x", &f64str(offset.x - 0.5*tab_width)).unwrap();
        writer.attr("y", &f64str(offset.y + stick_to_bottom + 0.5*raised_dia + r_margin + thickness + bearing_tolerance)).unwrap();
        writer.attr("width", &f64str(tab_width)).unwrap();
        writer.attr("height", &f64str(thickness)).unwrap();
        writer.end_elem().unwrap();
    }
}

fn write_base_bearing<W: Write>(writer: &mut XmlWriter<W>, offset: Point) {
    writer.begin_elem("circle").unwrap();
    cut_style(writer);
    writer.attr("cx", &f64str(offset.x)).unwrap();
    writer.attr("cy", &f64str(offset.y)).unwrap();
    writer.attr("r", &f64str(0.5*rod_dia)).unwrap();
    writer.end_elem().unwrap();

    writer.begin_elem("path").unwrap();
    cut_style(writer);
    let path = "M".to_string() + &f64str(offset.x - 0.5*rod_dia - r_margin) + "," +
        &f64str(offset.y - axis_h) +
    " L" + &f64str(offset.x - 0.5*rod_dia - r_margin) + "," +
        &f64str(offset.y) + 
    " A" + &f64str(0.5*rod_dia + r_margin) + "," + &f64str(0.5*rod_dia + r_margin) + ",0,0,0," +
        &f64str(offset.x + 0.5*rod_dia + r_margin) + "," +
        &f64str(offset.y) +
    " L" + &f64str(offset.x + 0.5*rod_dia + r_margin) + "," +
        &f64str(offset.y - axis_h) +
    " L" + &f64str(offset.x + 0.5*rod_dia + r_margin - tab_margin) + "," +
        &f64str(offset.y - axis_h) + 
    " L" + &f64str(offset.x + 0.5*rod_dia + r_margin - tab_margin) + "," +
        &f64str(offset.y - axis_h - thickness) +
    " L" + &f64str(offset.x - 0.5*rod_dia - r_margin + tab_margin) + "," +
        &f64str(offset.y - axis_h - thickness) +
    " L" + &f64str(offset.x - 0.5*rod_dia - r_margin + tab_margin) + "," +
        &f64str(offset.y - axis_h) +
    " L" + &f64str(offset.x - 0.5*rod_dia - r_margin) + "," +
        &f64str(offset.y - axis_h) +
    " Z";
    writer.attr("d", &path).unwrap();
    writer.end_elem().unwrap();
}

fn write_servo_bearing<W: Write>(writer: &mut XmlWriter<W>, offset: Point) {
    writer.begin_elem("path").unwrap();
    cut_style(writer);
    let elem_height = 0.5*servo_thickness + axis_h;
    if elem_height < servo_thickness {
        panic!("write_servo_bearing: Sorry, can't solve with that; the servo doesn't clear the base");
    }
    let path = "M".to_string() + &f64str(offset.x) + "," +
        &f64str(offset.y) +
    " L" + &f64str(offset.x + servo_body_width) + "," +
        &f64str(offset.y) +
    " L" + &f64str(offset.x + servo_body_width) + "," +
        &f64str(offset.y - elem_height - thickness) +
    " L" + &f64str(offset.x + servo_body_width + flange_len) + "," +
        &f64str(offset.y - elem_height - thickness) +
    " L" + &f64str(offset.x + servo_body_width + flange_len) + "," +
        &f64str(offset.y - elem_height) +
    " L" + &f64str(offset.x + servo_body_width + flange_len + tab_margin) + "," +
        &f64str(offset.y - elem_height) + 
    " L" + &f64str(offset.x + servo_body_width + flange_len + tab_margin) + "," +
        &f64str(offset.y + 2.0*tab_margin) +
    " L" + &f64str(offset.x - flange_len - tab_margin) + "," +
        &f64str(offset.y + 2.0*tab_margin) + 
    " L" + &f64str(offset.x - flange_len - tab_margin) + "," +
        &f64str(offset.y - elem_height) +
    " L" + &f64str(offset.x - flange_len) + "," +
        &f64str(offset.y - elem_height) +
    " L" + &f64str(offset.x - flange_len) + "," +
        &f64str(offset.y - elem_height - thickness) +
    " L" + &f64str(offset.x) + "," +
        &f64str(offset.y - elem_height - thickness) +
    " Z";
    writer.attr("d", &path).unwrap();
    writer.end_elem().unwrap();
}

fn write_azimuth_arm<W: Write>(writer: &mut XmlWriter<W>, offset: Point) {
    let r = 0.5*rod_dia + azi_r_margin;
    {
        writer.begin_elem("path").unwrap();
        cut_style(writer);
        let phi_rad = azi_angle*PI/180.;
        let t = azi_r_outer - azi_r_inner;
        let l = (r*r + azi_r_outer*azi_r_outer - 2.*r*azi_r_outer*(PI - phi_rad).cos()).sqrt();
        let l2 = (azi_r_inner*azi_r_inner - r*r).sqrt();
        let path = "M".to_string() + &f64str(offset.x) + "," +
            &f64str(offset.y) +
        " L" + &f64str(offset.x - l * phi_rad.sin()) + "," +
            &f64str(offset.y + l * phi_rad.cos()) +
        " A" + &f64str(azi_r_outer) + "," + &f64str(azi_r_outer) + ",0,0,1," +
            &f64str(offset.x - azi_r_outer) + "," + &f64str(offset.y) +
        " L" + &f64str(offset.x - azi_r_outer) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness - thickness) +
        " L" + &f64str(offset.x - azi_r_outer + t/3.) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness - thickness) +
        " L" + &f64str(offset.x - azi_r_outer + t/3.) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness) +
        " L" + &f64str(offset.x - azi_r_outer + 2.*t/3.) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness) +
        " L" + &f64str(offset.x - azi_r_outer + 2.*t/3.) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness - thickness) +
        " L" + &f64str(offset.x - azi_r_inner) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness - thickness) +
        " L" + &f64str(offset.x - azi_r_inner) + "," +
            &f64str(offset.y - r) +
        " A" + &f64str(azi_r_inner) + "," + &f64str(azi_r_inner) + ",0,0,0," +
            &f64str(offset.x - r * (PI - phi_rad).sin() - l2 * phi_rad.cos()) + "," +
            &f64str(offset.y - r + r * (PI - phi_rad).cos() + l2 * phi_rad.sin()) +
        " L" + &f64str(offset.x - r * (PI - phi_rad).sin()) + "," +
            &f64str(offset.y - r + r * (PI - phi_rad).cos()) +
        " A" + &f64str(r) + "," + &f64str(r) + ",0,0,1," +
            &f64str(offset.x + r * (PI - phi_rad).sin()) + "," +
            &f64str(offset.y - r + r * (PI - phi_rad).cos()) +
        " L" + &f64str(offset.x + r * (PI - phi_rad).sin() + l2 * phi_rad.cos()) + "," +
            &f64str(offset.y - r + r * (PI - phi_rad).cos() + l2 * phi_rad.sin()) +
        " A" + &f64str(azi_r_inner) + "," + &f64str(azi_r_inner) + ",0,0,0," +
            &f64str(offset.x + azi_r_inner) + "," +
            &f64str(offset.y - r) +
        " L" + &f64str(offset.x + azi_r_inner) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness - thickness) +
        " L" + &f64str(offset.x + azi_r_outer - 2.*t/3.) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness - thickness) +
        " L" + &f64str(offset.x + azi_r_outer - 2.*t/3.) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness) +
        " L" + &f64str(offset.x + azi_r_outer - t/3.) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness) +
        " L" + &f64str(offset.x + azi_r_outer - t/3.) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness - thickness) +
        " L" + &f64str(offset.x + azi_r_outer) + "," +
            &f64str(offset.y - r - 0.5*servo_thickness - thickness) +
        " L" + &f64str(offset.x + azi_r_outer) + "," +
            &f64str(offset.y) +
        " A" + &f64str(azi_r_outer) + "," + &f64str(azi_r_outer) + ",0,0,1," +
            &f64str(offset.x + l * phi_rad.sin()) + "," +
            &f64str(offset.y + l * phi_rad.cos()) +
        " Z";
        writer.attr("d", &path).unwrap();
        writer.end_elem().unwrap();
    }

    {
        writer.begin_elem("circle").unwrap();
        cut_style(writer);
        writer.attr("cx", &f64str(offset.x)).unwrap();
        writer.attr("cy", &f64str(offset.y - r)).unwrap();
        writer.attr("r", &f64str(rod_dia/2.)).unwrap();
        writer.end_elem().unwrap();
    }

    // TODO: slots for support struts
}

fn write_azimuthal_strut<W: Write>(writer: &mut XmlWriter<W>, offset: Point) {
    writer.begin_elem("path").unwrap();
    cut_style(writer);
    let t = azi_r_outer - azi_r_inner;
    let path = "M".to_string() + &f64str(offset.x) + "," + &f64str(offset.y) +
        " L" + &f64str(offset.x) + "," +
            &f64str(offset.y + raised_dia/2. - strut_tab_width/2.) +
        " L" + &f64str(offset.x + thickness) + "," +
            &f64str(offset.y + raised_dia/2. - strut_tab_width/2.) +
        " L" + &f64str(offset.x + thickness) + "," +
            &f64str(offset.y + raised_dia/2. + strut_tab_width/2.) +
        " L" + &f64str(offset.x) + "," +
            &f64str(offset.y + raised_dia/2. + strut_tab_width/2.) +
        " L" + &f64str(offset.x) + "," +
            &f64str(offset.y + raised_dia + 2.*r_margin) +
        " L" + &f64str(offset.x + t/3.) + "," +
            &f64str(offset.y + raised_dia + 2.*r_margin) +
        " L" + &f64str(offset.x + t/3.) + "," +
            &f64str(offset.y + raised_dia + 2.*r_margin + thickness) +
        " L" + &f64str(offset.x + 2.*t/3.) + "," +
            &f64str(offset.y + raised_dia + 2.*r_margin + thickness) +
        " L" + &f64str(offset.x + 2.*t/3.) + "," +
            &f64str(offset.y + raised_dia + 2.*r_margin) +
        " L" + &f64str(offset.x + t) + "," +
            &f64str(offset.y + raised_dia + 2.*r_margin) +
        " L" + &f64str(offset.x + t) + "," +
            &f64str(offset.y) +
        " L" + &f64str(offset.x + 2.*t/3.) + "," +
            &f64str(offset.y) +
        " L" + &f64str(offset.x + 2.*t/3.) + "," +
            &f64str(offset.y - thickness) +
        " L" + &f64str(offset.x + t/3.) + "," +
            &f64str(offset.y - thickness) +
        " L" + &f64str(offset.x + t/3.) + "," +
            &f64str(offset.y) +
        " Z";
    writer.attr("d", &path).unwrap();
    writer.end_elem().unwrap();
}

fn write_azimuthal_servo_mount<W: Write>(writer: &mut XmlWriter<W>, offset: Point) {
    assert!(strut_tab_width/2. > shaft_to_edge);
    writer.begin_elem("path").unwrap();
    cut_style(writer);
    let path = "M".to_string() + &f64str(offset.x) + "," + &f64str(offset.y) +
        " L" + &f64str(offset.x) + "," + &f64str(offset.y - thickness) + 
        " L" + &f64str(offset.x + strut_tab_width) + "," + &f64str(offset.y - thickness) +
        " L" + &f64str(offset.x + strut_tab_width) + "," + &f64str(offset.y) +
        " L" + &f64str(offset.x + strut_tab_width/2. + shaft_to_edge + flange_len) + "," +
            &f64str(offset.y) +
        " L" + &f64str(offset.x + strut_tab_width/2. + shaft_to_edge + flange_len) + "," +
            &f64str(offset.y + servo_thickness + 2.*tab_margin) +
        " L" + &f64str(offset.x + strut_tab_width/2. + shaft_to_edge - servo_body_width - flange_len) + "," +
            &f64str(offset.y + servo_thickness + 2.*tab_margin) +
        " L" + &f64str(offset.x + strut_tab_width/2. + shaft_to_edge - servo_body_width - flange_len) + "," +
            &f64str(offset.y) +
        " L" + &f64str(offset.x + strut_tab_width/2. + shaft_to_edge - servo_body_width) + "," +
            &f64str(offset.y) +
        " L" + &f64str(offset.x + strut_tab_width/2. + shaft_to_edge - servo_body_width) + "," +
            &f64str(offset.y + servo_thickness) +
        " L" + &f64str(offset.x + strut_tab_width/2. + shaft_to_edge) + "," +
            &f64str(offset.y + servo_thickness) +
        " L" + &f64str(offset.x + strut_tab_width/2. + shaft_to_edge) + "," +
            &f64str(offset.y) +
        " Z";
    writer.attr("d", &path).unwrap();
    writer.end_elem().unwrap();
}

fn write_azimuthal_bearing_mount<W: Write>(writer: &mut XmlWriter<W>, offset: Point) {
    assert!(rod_dia/2. < servo_thickness/2. + thickness);
    {
        writer.begin_elem("path").unwrap();
        cut_style(writer);
        let path = "M".to_string() + &f64str(offset.x) + "," + &f64str(offset.y) +
            " L" + &f64str(offset.x) + "," + &f64str(offset.y - thickness) +
            " L" + &f64str(offset.x + strut_tab_width) + "," +
                &f64str(offset.y - thickness) +
            " L" + &f64str(offset.x + strut_tab_width) + "," + &f64str(offset.y) +
            " L" + &f64str(offset.x + strut_tab_width + tab_margin) + "," +
                &f64str(offset.y) +
            " L" + &f64str(offset.x + strut_tab_width + tab_margin) + "," +
                &f64str(offset.y + servo_thickness + thickness) +
            " L" + &f64str(offset.x - tab_margin) + "," +
                &f64str(offset.y + servo_thickness + thickness) +
            " L" + &f64str(offset.x - tab_margin) + "," +
                &f64str(offset.y) +
            " Z";
        writer.attr("d", &path).unwrap();
        writer.end_elem().unwrap();
    }
    {
        writer.begin_elem("circle").unwrap();
        cut_style(writer);
        writer.attr("cx", &f64str(offset.x + strut_tab_width/2.)).unwrap();
        writer.attr("cy", &f64str(offset.y + servo_thickness/2.)).unwrap();
        writer.attr("r", &f64str(rod_dia)).unwrap();
        writer.end_elem().unwrap();
    }
}

fn main() {
    println!("Running gimbal design generator!");
    let mut out = File::create("test.svg").unwrap();
    out.write(b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n").unwrap();
    let mut xml_out = XmlWriter::new(out);

    write_root(&mut xml_out, |xmlout| {
        write_base(xmlout, pt(width * in_to_mm/2.0, base_height));
        write_base_bearing(xmlout, pt(width * in_to_mm/2.0 - 20.0, base_height + 100.0));
        write_servo_bearing(xmlout, pt(width * in_to_mm/2.0 + 20.0, base_height + 100.0));
        write_azimuth_arm(xmlout, pt(width * in_to_mm/2.0, base_height + 300.0));
        write_azimuthal_strut(xmlout, pt(width * in_to_mm/2.0 - 60.0, base_height + 100.0));
        write_azimuthal_strut(xmlout, pt(width * in_to_mm/2.0 + 60.0, base_height + 100.0));
        write_azimuthal_strut(xmlout, pt(width * in_to_mm/2.0 - 60.0, base_height + 200.0));
        write_azimuthal_strut(xmlout, pt(width * in_to_mm/2.0 + 60.0, base_height + 200.0));
        write_azimuthal_servo_mount(xmlout, pt(width * in_to_mm/2.0, base_height + 150.0));
        write_azimuthal_bearing_mount(xmlout, pt(width * in_to_mm/2.0, base_height + 200.0));
    });
}
