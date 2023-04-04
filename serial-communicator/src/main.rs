#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::io;
use std::io::Write; 
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialStream;
use log::{error, info};

use serial_communicator::{Request, Instruction}; 

mod util;
mod bindings;

const BAUD_RATE: u32 = 115_200; 

fn _find_devices() -> Vec<SerialStream> {
    const _FN_NAME: &str = "[serial-communicator::_find_devices]";

    let mut port_buf: Vec<SerialStream> = Vec::new(); 
    if let Ok(ports) = tokio_serial::available_ports() {
        for port_info in ports {
            let port_type = port_info.port_type;
            if let tokio_serial::SerialPortType::UsbPort(ref info) = port_type {
                if info.vid != 0x2341 || info.pid != 0x0042 {
                    continue; 
                }
            } else {
                continue; 
            }
            let port = tokio_serial::new(port_info.port_name, BAUD_RATE)
                .timeout(Duration::from_secs(1)); 
            if let Ok(port) = SerialStream::open(&port) {
                port_buf.push(port); 
            }
        }
    }
    return port_buf; 
}

async fn write_and_wait_response(
    port_stream: &mut SerialStream, 
    instruction: Instruction
) -> io::Result<Instruction> {
    const _FN_NAME: &str = "[serial-communicator::write_and_wait_response]"; 
    let opcode = instruction[0]; 

    port_stream.writable().await?; 
    AsyncWriteExt::write_all(port_stream, &instruction).await?; 
    AsyncWriteExt::flush(port_stream).await?; 

    port_stream.readable().await?; 
    let mut response_buf: Vec<u8> = Vec::with_capacity(8);  
    if opcode == bindings::SENSOR { 
        // => Wait for 8 bytes
        response_buf = vec![0; 8]; 
        AsyncReadExt::read_exact(port_stream, &mut response_buf).await?; 
    } else {
        // => Wait for 1 byte
        AsyncReadExt::read_buf(port_stream, &mut response_buf).await?; 
    }
    
    info!("{_FN_NAME} Received {:x?}", response_buf); 
    return Ok(response_buf); 
}

#[tokio::main]
async fn main() {
    const _FN_NAME: &str = "[serial-communicator::main]";
    simple_logger::init_with_env().unwrap(); 

    /* 1. Find Arduino device -- ONE device */
    let mut port_streams = _find_devices(); 
    if port_streams.is_empty() {
        error!("{_FN_NAME} Cannot find serial devices. Quitting..."); 
        return; 
    }
    let mut port_stream = port_streams.pop().unwrap(); 
    std::thread::sleep(std::time::Duration::from_secs(3)); 
    info!("{_FN_NAME} Connected to Arduino"); 

    let mut action_buffer: String = String::with_capacity(1024);
    // let mut read_buffer:   Vec<u8> = vec![0; 1024]; 
    
    loop {
        /* 2. Read from `stdin` and re-send to Arduino */
        action_buffer.clear();
        let action; 
        match io::stdin().read_line(&mut action_buffer) {
            Ok(0) => {
                // => EOF reached, close pipe
                info!("{_FN_NAME} EOF reached at stdin");
                return; 
            },
            Ok(_) => {
                // => Try convert to `Action` instance
                action = Request::try_from(action_buffer.as_ref())
            },
            Err(e) => {
                error!("{_FN_NAME} Unexpected error when reading from stdin: \n{:#?}", e);
                return;
            }
        };

        match action {
            Ok(Request::Write(v)) => {
                // => Write to Arduino, then wait on response and send to stdout
                match write_and_wait_response(&mut port_stream, v).await {
                    Ok(response) => {
                        if let Err(e) = io::stdout().write_all(&response) {
                            error!("{_FN_NAME} WRITE: Unexpected error when writing to stdout: \n{:#?}", e); 
                            return; 
                        }
                        if let Err(e) = io::stdout().flush() {
                            error!("{_FN_NAME} WRITE: Unexpected error when flushing stdout: \n{:#?}", e); 
                            return; 
                        }
                    }, 
                    Err(e) => {
                        error!("{_FN_NAME} WRITE: Unexpected error when requesting Arduino: \n{:#?}", e); 
                        return; 
                    }
                }  
            }, 
            Err(e) => 
                error!("{_FN_NAME} Invalid input from stdin: \n{:#?}", e), 
        }
    }
}
