/*
 * Test writing/reading an eet file.
 */

extern crate libc;
extern crate efl;

use libc::c_char;
use std::mem::{transmute, size_of_val};
use std::c_str::CString;
use efl::eet;

fn main() {

    // Initialize eet
    eet::init();
    
    // Open and write
    let mut ef: Box<eet::EetFile> = eet::open("test.eet", eet::EetFileModeWrite);
    // Write compressed string
    let s = "Hello Eet!";
    // Apply some magic for proper C string conversion
    s.with_c_str(|c_str| unsafe {
        eet::write(ef, "String", transmute::<*c_char,&c_char>(c_str), s.len()+1, 1)
    });
    // Write uncompressed integer
    let i = 13;
    eet::write(ef, "Integer", &i, size_of_val(&i), 0);
    // Write compressed float
    let f = 9.6729f32;
    eet::write(ef, "Float", &f, size_of_val(&f), 1);
    // Sync
    println!("Sync string/integer/float: {}", eet::sync(ef));
    // Close
    eet::close(ef);

    // Open and read
    ef = eet::open("test.eet", eet::EetFileModeRead);
    let mut size = 0;
    // Read string
    let retS: Box<c_char> = eet::read(ef, "String", &mut size);
    // Get a proper &str to show
    unsafe {
        match CString::new(transmute::<_,*c_char>(retS), true).as_str() {
            None => println!("Not valid"),
            Some(s) => println!("String: {}", s)
        }
    }
    // Read integer
    let retI: Box<int> = eet::read(ef, "Integer", &mut size);
    println!("Integer: {}", retI);
    // Read float
    let retF: Box<f32> = eet::read(ef, "Float", &mut size);
    println!("Float: {}", retF);

    // Close eet file and shutdown eet
    println!("Close: {}", eet::close(ef));
    eet::shutdown();

}
