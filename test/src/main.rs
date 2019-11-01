use std::process;
use std::fs::metadata;
use std::os::unix::fs::FileTypeExt;

extern crate libc;

use clap::{load_yaml, App, AppSettings::ColoredHelp};

fn main() {
    let yaml = load_yaml!("test.yml");
    let matches = App::from_yaml(yaml).settings(&[ColoredHelp]).get_matches();

    let result = match matches.subcommand() {
        ("-b", Some(filename_match)) => metadata(filename_match.value_of("filename").unwrap()).unwrap().file_type().is_block_device(),
        ("-c", Some(filename_match)) => metadata(filename_match.value_of("filename").unwrap()).unwrap().file_type().is_char_device(),
        ("-d", Some(filename_match)) => metadata(filename_match.value_of("filename").unwrap()).unwrap().file_type().is_dir(),
        ("-f", Some(filename_match)) => metadata(filename_match.value_of("filename").unwrap()).unwrap().file_type().is_file(),
        ("-L", Some(filename_match)) |
        ("-h", Some(filename_match)) => metadata(filename_match.value_of("filename").unwrap()).unwrap().file_type().is_symlink(),
        ("-p", Some(filename_match)) => metadata(filename_match.value_of("filename").unwrap()).unwrap().file_type().is_fifo(),
        ("-S", Some(filename_match)) => metadata(filename_match.value_of("filename").unwrap()).unwrap().file_type().is_socket(),
        /*("-t", Some(filename_match)) => {
            let is_tty = unsafe { libc::isatty(libc::STDIN_FILENO as i32) } != 0;
            is_tty
        }*/

        _ => process::exit(1),
    };

    if result {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
