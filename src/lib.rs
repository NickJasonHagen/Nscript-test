//use core::num::dec2flt::parse::parse_number;
// Nscript v2 ( remade from au3 nscript) by Nick Hagen.
pub mod nscriptlib{
    use std::collections::{HashMap};
    //use std::{env, array, string};
    use std::fs;
    use std::fs::File;
    use std::path::{Path, PrefixComponent};
    use std::io::{self,Read, Write};
    use std::net::{SocketAddr, TcpStream};
    use std::process;
    use std::process::{Command, exit};
    use std::time::{Duration};
    use url::Url;
    use colored::Colorize;
    use std::net::ToSocketAddrs;
    use rand::seq::SliceRandom;
    use encoding_rs::{ UTF_8};

    //time
    use chrono::{Datelike, Timelike};

    use std::{
        net::{TcpListener },
    };
    use reqwest;
    use hex::FromHex;
    //use regex::Regex;
    use std::thread;
    //mod nscriptapilib;
    //pub mod includes {
        // pub mod includes::nscript_zip;
        // pub mod includes::nscript_api_lib;
        // pub mod includes::nscript_functions;
        // pub mod includes::nscript_arrays;
        // pub mod includes::nscript_file_and_system;
        // pub mod includes::nscript_strings;
        // pub mod includes::nscript_interpreter;
        // pub mod includes::nscript_rust_fn_bindings;
        // pub mod includes::nscript_http_html;
        // pub mod includes::nscript_time;

    //}
    use nscriptlib::includes::nscript_time::*;
    use nscriptlib::includes::nscript_http_html::*;
    use nscriptlib::includes::nscript_rust_fn_bindings::*;
    use nscriptlib::includes::nscript_zip::*;
    use nscriptlib::includes::nscript_interpreter::*;
    use nscriptlib::includes::nscript_api_lib::*;
    use nscriptlib::includes::nscript_functions::*;
    use nscriptlib::includes::nscript_strings::*;
    use nscriptlib::includes::nscript_arrays::*;
    use nscriptlib::includes::nscript_file_and_system::*;


    use reqwest::blocking::get;
    use rand::Rng;
    #[cfg(windows)]
   pub  mod ioctlsocket {
        use std::os::windows::raw::SOCKET;
        use std::os::raw::{c_long, c_ulong};

        extern "system" {
        pub fn ioctlsocket(s: SOCKET, cmd: c_long, argp: *mut c_ulong) -> i32;
    }
    }

    //#[cfg(not(windows))]
    //use std::os::unix::io::AsRawFd;
    const NSCRIPT_VERSION: &'static str = "v2.005";
    // const NSCRIPT_INFO: &'static str = "
    // Nscript core in Rust-language.
    // Created by Nick Hagen.
    // 2022-23";
    #[cfg(windows)]
    const LINE_ENDING: &'static str = "\n";
    #[cfg(not(windows))]
    const LINE_ENDING: &'static str = "\n";
    const CODE_LINE_ENDING: &'static str = "\n";
    #[cfg(windows)]
    const MACRO_OS: &'static str = "Windows";
    #[cfg(not(windows))]
    const MACRO_OS: &'static str = "Unix";
    const SERVER_ADDRESS: &str = "0.0.0.0";
    const SERVER_PORT: u16 = 8088;
    #[cfg(not(windows))]
    const SERVER_ROOT: &str = "./public/";
    #[cfg(windows)]
    const SERVER_ROOT: &str = ".\\public\\";
    #[cfg(not(windows))]
    const SCRIPT_DIR : &str = "./";
    #[cfg(windows)]
    const SCRIPT_DIR: &str = ".\\";
    use std::env;
    //use std::path::{PathBuf, Path};
}






