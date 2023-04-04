#![cfg(unix)]

extern crate serial_communicator; 

use std::time::Duration;
use serialport::{SerialPort, TTYPort, Result}; 

use serial_communicator::util::serial_helper::*; 

const TEST_QWORD: u64                = 0xcafe_beef_dead_acab; 
const TEST_QWORD_FLIPPED_ENDIAN: u64 = 0xabac_adde_efbe_feca; 
const TEST_DWORD: u32                = 0xcafe_beef; 
const TEST_DWORD_FLIPPED_ENDIAN: u32 = 0xefbe_feca; 
const TEST_I64: i64                  = i64::MIN; 
const TEST_I32: i32                  = i32::MIN; 

const TEST_STR_NOLF: &str = "This is a string slice!"; 
const TEST_STR_LF: &str   = "This is a string slice!\n"; 
const NEWLINE: u8         = unsafe{ *(&'\n' as *const char).cast() }; 

fn _set_up() -> Result<(TTYPort, TTYPort)> {
    TTYPort::pair()
}

fn _write_raw_read_raw_u32(tx: &mut TTYPort, rx: &mut TTYPort) {
    write_dword_raw(tx, TEST_DWORD)
        .expect("[write_raw_read_raw_u32] Cannot write to `tx`"); 
    if let Ok(dword) = read_dword_raw(rx) {
        assert_eq!(
            dword, 
            TEST_DWORD, 
            "[ERROR] `read_dword_raw` received incorrect DWORD at `rx` read"
        ); 
    } else {
        panic!("[write_raw_read_raw_u32] Cannot read at `rx`"); 
    }
}

fn _write_flipped_read_raw_u32(tx: &mut TTYPort, rx: &mut TTYPort) {
    write_dword_flipped_endian(tx, TEST_DWORD)
        .expect("[write_flipped_read_raw_u32] Cannot write to `tx`"); 
    if let Ok(dword) = read_dword_raw(rx) {
        assert_eq!(
            dword, 
            TEST_DWORD_FLIPPED_ENDIAN, 
            "[ERROR] `read_dword_raw` received incorrect DWORD at `rx` read"
        ); 
    } else {
        panic!("[write_flipped_read_raw_u32] Cannot read at `rx`"); 
    }
}

fn _write_raw_read_flipped_u32(tx: &mut TTYPort, rx: &mut TTYPort) {
    write_dword_raw(tx, TEST_DWORD)
        .expect("[write_raw_read_flipped_u32] Cannot write to `tx`"); 
    if let Ok(dword) = read_dword_flipped_endian(rx) {
        assert_eq!(
            dword, 
            TEST_DWORD_FLIPPED_ENDIAN, 
            "[ERROR] `read_dword_flipped_endian` received incorrect DWORD at `rx` read"
        ); 
    } else {
        panic!("[write_raw_read_flipped_u32] Cannot read at `rx`"); 
    }
}

fn _write_raw_read_i32(tx: &mut TTYPort, rx: &mut TTYPort) {
    unsafe {
        write_dword_raw(tx, *(&TEST_I32 as *const i32).cast())
            .expect("[write_raw_read_i32] Cannot write to `tx`"); 
    }
    if let Ok(s_int) = read_i32_raw(rx) {
        assert_eq!(
            s_int, 
            TEST_I32, 
            "[ERROR] `read_i32_raw` received incorrect i32 at `rx` read"
        ); 
    } else {
        panic!("[write_raw_read_i32] Cannot read at `rx`"); 
    }
}

fn _write_raw_read_raw_u64(tx: &mut TTYPort, rx: &mut TTYPort) {
    write_qword_raw(tx, TEST_QWORD)
        .expect("[write_raw_read_raw_u64] Cannot write to `tx`"); 
    if let Ok(qword) = read_qword_raw(rx) {
        assert_eq!(
            qword, 
            TEST_QWORD, 
            "[ERROR] `read_qword_raw` received incorrect QWORD at `rx` read"
        ); 
    } else {
        panic!("[write_raw_read_raw_u64] Cannot read at `rx`"); 
    }
}

fn _write_flipped_read_raw_u64(tx: &mut TTYPort, rx: &mut TTYPort) {
    write_qword_flipped_endian(tx, TEST_QWORD)
        .expect("[write_flipped_read_raw_u64] Cannot write to `tx`"); 
    if let Ok(qword) = read_qword_raw(rx) {
        assert_eq!(
            qword, 
            TEST_QWORD_FLIPPED_ENDIAN, 
            "[ERROR] `read_dword_raw` received incorrect DWORD at `rx` read"
        ); 
    } else {
        panic!("[write_flipped_read_raw_u64] Cannot read at `rx`"); 
    }
}

fn _write_raw_read_flipped_u64(tx: &mut TTYPort, rx: &mut TTYPort) {
    write_qword_raw(tx, TEST_QWORD)
        .expect("[write_raw_read_flipped_u64] Cannot write to `tx`"); 
    if let Ok(qword) = read_qword_flipped_endian(rx) {
        assert_eq!(
            qword, 
            TEST_QWORD_FLIPPED_ENDIAN, 
            "[ERROR] `read_dword_flipped_endian` received incorrect DWORD at `rx` read"
        ); 
    } else {
        panic!("[write_raw_read_flipped_u64] Cannot read at `rx`"); 
    }
}

fn _write_raw_read_i64(tx: &mut TTYPort, rx: &mut TTYPort) {
    unsafe {
        write_qword_raw(tx, *(&TEST_I64 as *const i64).cast())
            .expect("[write_raw_read_i64] Cannot write to `tx`"); 
    }
    if let Ok(s_int) = read_i64_raw(rx) {
        assert_eq!(
            s_int, 
            TEST_I64, 
            "[ERROR] `read_i64_raw` received incorrect i64 at `rx` read"
        ); 
    } else {
        panic!("[write_raw_read_i64] Cannot read at `rx`"); 
    }
}

fn _write_str_raw_read(tx: &mut TTYPort, rx: &mut TTYPort) {
    write_str_raw(tx, TEST_STR_LF)
        .expect("[write_str_raw_read] Cannot write to `tx`"); 
    if let Ok(s) = read_string_until_byte(rx, NEWLINE) {
        assert_eq!(
            &s, 
            TEST_STR_LF, 
            "[ERROR] `read_string_until_byte` received incorrect String at `rx` read"
        ); 
    } else {
        panic!("[write_str_raw_read] Cannot read at `rx`"); 
    }
}

fn _write_str_append_read(tx: &mut TTYPort, rx: &mut TTYPort) {
    write_str_ends_with(tx, TEST_STR_NOLF, NEWLINE)
        .expect("[write_str_append_read] Cannot write to `tx`"); 
    if let Ok(s) = read_string_until_byte(rx, NEWLINE) {
        assert_eq!(
            &s, 
            TEST_STR_LF, 
            "[ERROR] `read_string_until_byte` received incorrect String at `rx` read"
        ); 
    } else {
        panic!("[write_str_append_read] Cannot read at `rx`"); 
    }
}

#[test]
fn test_ttyport_pairs() {
    let (mut tx, mut rx) = _set_up()
        .expect("[local_test::set_up] Cannot create pseudo TTY ports");
    
    tx.set_timeout(Duration::from_millis(10))
        .expect("[local_test::test_ttyport_pairs] Cannot set timeout on `tx`");
    rx.set_timeout(Duration::from_millis(10))
        .expect("[local_test::test_ttyport_pairs] Cannot set timeout on `rx`"); 

    // DWORD
    _write_raw_read_raw_u32(&mut tx, &mut rx); 
    _write_flipped_read_raw_u32(&mut tx, &mut rx); 
    _write_raw_read_flipped_u32(&mut tx, &mut rx); 
    _write_raw_read_i32(&mut tx, &mut rx); 

    // QWORD
    _write_raw_read_raw_u64(&mut tx, &mut rx);
    _write_flipped_read_raw_u64(&mut tx, &mut rx);
    _write_raw_read_flipped_u64(&mut tx, &mut rx); 
    _write_raw_read_i64(&mut tx, &mut rx); 

    // String and &str
    _write_str_raw_read(&mut tx, &mut rx); 
    _write_str_append_read(&mut tx, &mut rx); 
}