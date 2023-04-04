extern crate cc; 
extern crate bindgen; 

use std::env; 
use std::path::{Path, PathBuf};

use bindgen::CargoCallbacks; 

fn main() {
    /* Paths from CWD */
    let libdir_path = PathBuf::from("../arduino_comms")
        .canonicalize()
        .unwrap(); 
    let header_path = libdir_path.join("opcode.h"); 
    let header_path_str = header_path.to_str().unwrap(); 

    /* Build arduino_comms/opcode.h */
    cc::Build::new()
        .file(AsRef::<Path>::as_ref(&header_path))
        .compile("opcode"); 
    
    /* Configure and create bindings */
    let bindings = bindgen::Builder::default()
        .header(header_path_str)
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
        .fit_macro_constants(true)
        .size_t_is_usize(true)
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .unwrap(); 

    let out_path = PathBuf::from(env::current_dir().unwrap()).join("src/bindings.rs"); 
    bindings.write_to_file(out_path).unwrap(); 
}