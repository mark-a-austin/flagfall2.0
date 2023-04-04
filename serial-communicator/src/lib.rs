#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fmt::Display;

use itertools::Itertools;

pub mod util; 
mod bindings;

pub type Instruction = Vec<u8>; 

#[derive(PartialEq, Eq, Clone)]
pub enum Request {
    // Read, 
    Write(Instruction), 
}

impl Request {
    fn _try_parse_opcode(opword: &str) -> Result<u8, ()> {
        match opword {
            "SENSOR" => Ok(bindings::SENSOR), 
            "MAGNET" => Ok(bindings::MAGNET), 
            "LED"    => Ok(bindings::LED), 
            // "HANDSHAKE" => bindings::HANDSHAKE, 
            "ACK"    => Ok(bindings::ACK), 
            "QUIT"   => Ok(bindings::QUIT), 
            _        => Err(()), 
        }
    }

    fn _try_parse_arguments_into(
        instr_buf: &mut Instruction, 
        words: &mut dyn Iterator<Item = &str>
    ) -> Result<usize, ()> {
        if instr_buf.len() != 1 { return Err(()); }
        let mut idx: usize = 1; 
        let opcode = instr_buf[0]; 
        match opcode {
            bindings::MAGNET => {
                let elems = words.chunks(3); 
                for elem in &elems {
                    if let Some((x, y, is_on)) = elem.collect_tuple() {
                        let res = (
                            x.parse::<f32>(), 
                            y.parse::<f32>(), 
                            is_on.parse::<bool>()
                        ); 
                        if res.0.is_err() || res.1.is_err() || res.2.is_err() { return Err(()); }

                        // Insert x
                        let x_in_le_bytes = res.0.unwrap().to_le_bytes(); 
                        for b in x_in_le_bytes { 
                            instr_buf.push(b); 
                            idx += 1; 
                        }

                        // Insert y
                        let y_in_le_bytes = res.1.unwrap().to_le_bytes(); 
                        for b in y_in_le_bytes {
                            instr_buf.push(b); 
                            idx += 1; 
                        }

                        // Insert is_on: bool as u8
                        instr_buf.push(res.2.unwrap().into()); 
                        idx += 1; 
                    }
                    // Else malformed, continue.
                }
                return Ok(idx - 1); 
            }, 
            bindings::LED => {
                for word in words {
                    if let Ok(rgb_int) =  word.parse::<u32>() {
                        let tmp = rgb_int.to_be_bytes(); 
                        for b in &tmp[1..] {
                            // Correctness? 
                            instr_buf.push(*b); 
                            idx += 1; 
                        }
                    }
                    // Else malformed, continue.
                }
                return Ok(idx - 1); 
            }, 
            _ => 
                return Ok(0), 
        }
    }
}

#[derive(Debug)]
pub enum RequestConversionError {
    UndefinedOpSequence(String), 
    EmptyOpSequence(String), 
    MalformedOpSequence(String), 
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Request::Read => 
            //    write!(f, "READ"), 
            Request::Write(s) => 
                write!(f, "WRITE {:?}", s), 
        }
    }
}

impl TryFrom<&str> for Request {
    type Error = RequestConversionError;

    fn try_from(action: &str) -> Result<Self, Self::Error> {
        const _FN_NAME: &str = "[Action as TryFrom::try_from]"; 

        /* 1. Parse serial-communicator op */
        let mut split = action.split_ascii_whitespace(); 
        match split.next() {
            // Some("READ")  => return Ok(Request::Read), 
            Some("WRITE") => (), 
            Some(s) => 
                return Err(RequestConversionError::UndefinedOpSequence(
                    format!("{_FN_NAME} Expected \"WRITE\", got {s}")
                )), 
            None => 
                return Err(RequestConversionError::EmptyOpSequence(
                    format!("{_FN_NAME} Empty sequence as input")
                )), 
        }

        /* 2. Parse Arduino op */
        let mut instr_buf: Vec<u8> = Vec::with_capacity(512); 
        match split.next() {
            Some(s) => {
                if let Ok(opcode) = Request::_try_parse_opcode(s) {
                    instr_buf.push(opcode); 
                } else {
                    return Err(RequestConversionError::UndefinedOpSequence(
                        format!("{_FN_NAME} Undefined or invalid op name in sequence: {s}")
                    )); 
                }
            }, 
            None => 
                return Err(RequestConversionError::EmptyOpSequence(
                    format!("{_FN_NAME} Expected Arduino operation but 0 argument provided")
                )), 
        }

        /* 3. Parse Arduino arguments */
        if let Err(_) = Request::_try_parse_arguments_into(&mut instr_buf, &mut split) {
            return Err(RequestConversionError::MalformedOpSequence(
                format!("{_FN_NAME} Malformed argument list: {}", split.collect::<String>())
            )); 
        }
             
        return Ok(Request::Write(instr_buf)); 
    }
}