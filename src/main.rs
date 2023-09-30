use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::{c_char, CStr, CString};
use std::fs::File;
use std::io::BufReader;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Config {
    mime_type_to_program: HashMap<String, String>,
}

#[repr(C)]
pub struct magic_type {
    _data: [u8; 0],
}

#[allow(non_camel_case_types)]
pub type magic_t = *mut magic_type;

#[link(name = "magic")]
extern "C" {
    fn magic_open(x: i32) -> magic_t;
    fn magic_load(cookie: magic_t, filename: *const c_char) -> i32;
    fn magic_file(cookie: magic_t, filename: *const c_char) -> *const c_char;
}

fn get_mime_for_file_path(magic_cookie: magic_t, path: &str) -> Result<String, Box<dyn Error>> {
    let c_path = CString::new(path)?;

    let magic_mime_str = unsafe {
        let magic_mime = magic_file(magic_cookie, c_path.as_ptr());
        CStr::from_ptr(magic_mime).to_str()?
    };

    Ok(magic_mime_str.to_string())
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Ok(u)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let file_paths = &args[1..];
    if file_paths.len() == 0 {
        return Err("No file paths passed!".into());
    }

    let home: String = env::var("HOME")?;
    let config_path: String = format!("{}/.config/o.conf", home);
    let config: Config = read_config_from_file(config_path)?;

    const MAGIC_MIME_TYPE__MAGIC_SYMLINK: i32 = 0x000_0012;

    let maybe_descriptions: Result<Vec<String>, Box<dyn Error>> = unsafe {
        let magic_cookie = magic_open(MAGIC_MIME_TYPE__MAGIC_SYMLINK);
        magic_load(magic_cookie, std::ptr::null());

        file_paths
            .iter()
            .map(|p| get_mime_for_file_path(magic_cookie, p))
            .collect()
    };

    let mut descriptions = maybe_descriptions?;
    descriptions.sort();
    descriptions.dedup();

    if descriptions.len() > 1 {
        return Err(format!("More than one type found! {:?}", descriptions).into());
    }
    let description = &descriptions[0];

    let program = &config
        .mime_type_to_program
        .get(description.as_str())
        .expect(format!("No defined program for {}", description).as_str());

    Command::new(program).args(file_paths).exec();
    Ok(())
}
