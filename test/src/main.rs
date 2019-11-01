use std::process;
use std::fs::metadata;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use users::{get_user_by_uid, get_current_uid};

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

        ("-g", Some(filename_match)) => metadata(filename_match.value_of("filename").unwrap()).unwrap().permissions().mode() & 0o2000 > 0,
        ("-k", Some(filename_match)) => metadata(filename_match.value_of("filename").unwrap()).unwrap().permissions().mode() & 0o1000 > 0,
        ("-u", Some(filename_match)) => metadata(filename_match.value_of("filename").unwrap()).unwrap().permissions().mode() & 0o4000 > 0,
        ("-r", Some(filename_match)) => {
            let c_str = filename_match.value_of("filename").unwrap();
            (unsafe { libc::access(c_str.as_ptr() as *const i8, 4) } == 0)
        }
        ("-w", Some(filename_match)) => {
            let c_str = filename_match.value_of("filename").unwrap();
            (unsafe { libc::access(c_str.as_ptr() as *const i8, 2) } == 0)
        }
        ("-x", Some(filename_match)) => {
            let c_str = filename_match.value_of("filename").unwrap();
            (unsafe { libc::access(c_str.as_ptr() as *const i8, 1) } == 0)
        }
        ("-O", Some(filename_match)) => {
            let current_user_id = get_current_uid();
            let file_user_id = metadata(filename_match.value_of("filename").unwrap()).unwrap().uid();
            current_user_id == file_user_id
        }
        ("-G", Some(filename_match)) => {
            let current_group_id = get_user_by_uid(get_current_uid()).unwrap().primary_group_id();
            let file_group_id = metadata(filename_match.value_of("filename").unwrap()).unwrap().gid();
            current_group_id == file_group_id
        }

        _ => process::exit(1),
    };

    if result {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
