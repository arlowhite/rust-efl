// Eet Rust bindings for EFL.
// Copyright (C) 2014  Luis Araujo <araujoc.luisf@gmail.com>

// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA


extern crate libc;

use eet::libc::{c_int, c_uint, c_char, c_void};
use std::cast::transmute;


pub enum EetFile {}

/// Modes that a file can be opened.
pub enum EetFileMode {
    /// File is read-only.
    EetFileModeRead,
    /// File is write-only.
    EetFileModeWrite,
    /// File is for both read and write.
    EetFileModeReadWrite	
}

#[deriving(Show)]
pub enum EetError {
  EetErrorNone, 
  EetErrorBadObject,
  EetErrorEmpty,
  EetErrorNotWritable,
  EetErrorOutOfMemory,
  EetErrorWriteError,
  EetErrorWriteErrorFileTooBig,
  EetErrorWriteErrorIOError,
  EetErrorWriteErrorOutOfSpace,
  EetErrorWriteErrorFileClosed,
  EetErrorMmapFailed,
  EetErrorX509EncodingFailed,
  EetErrorSignatureFailed,
  EetErrorInvalidSignature,
  EetErrorNotSigned,
  EetErrorNotImplemented,
  EetErrorPrngNotSeeded,
  EetErrorEncryptFailed,
  EetErrorDecryptFailed
}

fn return_eet_error(c: u32) -> EetError {
    match c {
        0 => EetErrorNone, 
        1 => EetErrorBadObject,
        2 => EetErrorEmpty,
        3 => EetErrorNotWritable,
        4 => EetErrorOutOfMemory,
        5 => EetErrorWriteError,
        6 => EetErrorWriteErrorFileTooBig,
        7 => EetErrorWriteErrorIOError,
        8 => EetErrorWriteErrorOutOfSpace,
        9 => EetErrorWriteErrorFileClosed,
        10 => EetErrorMmapFailed,
        11 => EetErrorX509EncodingFailed,
        12 => EetErrorSignatureFailed,
        13 => EetErrorInvalidSignature,
        14 => EetErrorNotSigned,
        15 => EetErrorNotImplemented,
        16 => EetErrorPrngNotSeeded,
        17 => EetErrorEncryptFailed,
        18 => EetErrorDecryptFailed,
        _ => fail!("Not valid EetError")
    }
}

#[link(name = "eet")]
extern "C" {
    fn eet_init() -> c_int;
    fn eet_shutdown() -> c_int;
    fn eet_clearcache();
    fn eet_open(file: *c_char, mode: c_uint) -> *EetFile;
    fn eet_read(ef: *EetFile, name: *c_char, size_ret: *mut c_int) -> *c_void;
    fn eet_write(ef: *EetFile, name: *c_char, data: *c_void,
                 size: c_uint, compress: c_int) -> c_int;
    fn eet_close(ef: *EetFile) -> c_uint;
    fn eet_sync(ef: *EetFile) -> c_uint;
}

/// Initialize the EET library.
pub fn init() -> int { unsafe { eet_init() as int } }

/// Shut down the EET library.
pub fn shutdown() -> int { unsafe { eet_shutdown() as int } }

/// Clear eet cache.
pub fn clearcache() { unsafe { eet_clearcache() } }

/// Open an eet file on disk, and returns a handle to it.
pub fn open(file: &str, mode: EetFileMode) -> ~EetFile {
    file.with_c_str(|c_file| unsafe {
        transmute::<*EetFile,~EetFile>(eet_open(c_file, mode as c_uint))
    })
}

/// Read a specified entry from an eet file and return data.
pub fn read<T>(ef: &EetFile, name: &str, size_ret: &mut i32) -> ~T {
    name.with_c_str(|c_name| unsafe {
        transmute::<*c_void,~T>(eet_read(ef, c_name, size_ret))
    })
}

pub fn write<T>(ef: &EetFile, name: &str, data: &T,
                size: uint, compress: int) -> int {
    name.with_c_str(|c_name| unsafe {
        let c_data: *c_void = transmute(data);
        eet_write(ef, c_name, c_data, size as c_uint, compress as c_int) as int
    })
}

//// Close an eet file handle and flush pending writes.
pub fn close(ef: &EetFile) -> EetError {
    unsafe { return_eet_error(eet_close(ef)) }
}

/// Sync content of an eet file handle, flushing pending writes.
pub fn sync(ef: &EetFile) -> EetError {
    unsafe { return_eet_error(eet_sync(ef)) }
}