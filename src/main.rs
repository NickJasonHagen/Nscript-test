use std::char::ToLowercase;
// Nscript v2 ( remade from au3 nscript) by Nick Hagen.
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
mod includes {
    pub mod nscript_zip;
    pub mod nscript_api_lib;
    pub mod nscript_functions;
    pub mod nscript_arrays;
    pub mod nscript_file_and_system;
    pub mod nscript_strings;
    pub mod nscript_interpreter;
    pub mod nscript_rust_fn_bindings;
    pub mod nscript_http_html;
    pub mod nscript_time;


}
use includes::nscript_time::*;
use includes::nscript_http_html::*;
use includes::nscript_rust_fn_bindings::*;
use includes::nscript_zip::*;
use includes::nscript_interpreter::*;
use includes::nscript_api_lib::*;
use includes::nscript_functions::*;
use includes::nscript_strings::*;
use includes::nscript_arrays::*;
use includes::nscript_file_and_system::*;




use reqwest::blocking::get;
use rand::Rng;
#[cfg(windows)]
mod ioctlsocket {
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


fn main(){
    let mode = terminal_get_user_input("\n ---Nscript launcher!-------------\n\n1) Run clean (system/clean.nc) \n2) Run nscript as a http server\n3) run a custom script!   pick mode 1/2/3 \n -------------","1");
    match mode.as_str() {
        "1" => {
            main_clean();
        }
        "2" => {
            main_http();

        }
        "3" => {
            let scriptname = terminal_get_user_input("Your script full path (or from bin path)","./myscript.nc");
            if Nfile::checkexists(&scriptname) {
                main_custom(&scriptname);
            }
            else{
                cwrite("Coulnd find the script at the given location!?","r");
            }
        }
        _ => {
            main_clean();

        }

    }

}

fn testoverride(vmap: &mut Varmap)-> String{
// testoverride requires vmap, this function extents the parsers functions to be used in Nscript.
    // you can retrieve the nscript call's data by using : vmap.funcname ( the name of the function)
    // and vmap.param1 ~ vmap.param9 , hardcoded functions be capped to 9 arguments, here you can
    // map your own logic, just return the result as a String and the parser will manage the rest.
    // params and funcnames are all String. if they are unused by nscript they are set to be empty
    // if your function requires data you can check by if vmap.param1 != "" error(yourlogic)
    if vmap.funcname == "testing" {
        cwrite("testingoverrides!!","g");

        cwrite(&vmap.param1,"g");
        cwrite(&vmap.param2,"g");
        cwrite(&vmap.param3,"g");
        return vmap.param1.to_owned()
    }
    "".to_owned()

}

fn main_clean() -> std::io::Result<()>  {

    let mut vmap = Varmap::new(); // global



    vmap.setextentionfunctions(testoverride);


    println!("Starting fn main() Nscript {}",NSCRIPT_VERSION);
    println!("____________________________________");

    let serverscriptfilename = SCRIPT_DIR.to_owned() +"system/clean.nc";
    nscript_execute_script(&serverscriptfilename,"","","","","","","","","",&mut vmap);

    loop {
        nscript_loops(&mut vmap);

    }
}

fn main_custom(scriptloc: &str) -> std::io::Result<()>  {

    let mut vmap = Varmap::new(); // global

    println!("Starting fn main() Nscript {}",NSCRIPT_VERSION);
    println!("____________________________________");

    let serverscriptfilename = SCRIPT_DIR.to_owned() + &scriptloc;
    nscript_execute_script(&serverscriptfilename,"","","","","","","","","",&mut vmap);

    loop {
        nscript_loops(&mut vmap);

    }
}


fn main_http() -> std::io::Result<()>  {
    let mut vmap = Varmap::new(); // global

    println!("Starting fn main() Nscript {}",NSCRIPT_VERSION);
    println!("____________________________________");

    // run Nscript:server.nc ,define pre logic here, this runs before the stream starts.
    vmap.setvar("self".to_owned(),"server");//<- set self in nscript during scope
    let serverscriptfilename = SCRIPT_DIR.to_owned() +"system/init.nc";
    nscript_execute_script(&serverscriptfilename,"","","","","","","","","",&mut vmap);
    // retrieve the prop's set for class server in nscript:server.nc
    let server_addres_nc = nscript_checkvar("server.ip", &mut vmap);
    let server_port_nc = nscript_checkvar("server.port", &mut vmap);

    let  listener: TcpListener;
    if server_port_nc != "" && server_addres_nc != ""{
        // when the server.nc is run class server.ip and server.port be checked!
        listener = TcpListener::bind(format!("{}:{}", &server_addres_nc, &server_port_nc)).unwrap();
        println!("Server started at http://{}:{}", &server_addres_nc, &server_port_nc);
    }
    else{
        // if missing serverclass or something, use the constants
        listener = TcpListener::bind(format!("{}:{}", SERVER_ADDRESS, SERVER_PORT)).unwrap();
        println!("Server started at http://{}:{}", SERVER_ADDRESS, SERVER_PORT);
    }
    // sets the
    // acceptsocketlisterns to continue and not hold on the script
    #[cfg(windows)]
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    #[cfg(not(windows))]
    listener.set_nonblocking(true)?;


    // this checks your /domains/ folder for subfolders
    // you can name a folder to your dns-domain
    // all http to this domain be rerouted to this folders
    let domaindir = SCRIPT_DIR.to_owned() +"domains/";
    let domdir = Nfile::dirtolist(&domaindir,false);
    let domaindirarr = split(&domdir,NC_ARRAY_DELIM);
    for domainscript in domaindirarr {
        if domainscript != ""{
            vmap.setvar("___domainname".to_owned(),&domainscript);
            let domainscript = SCRIPT_DIR.to_owned() + "domains/"+domainscript.trim() + "/http.nc";
            nscript_execute_script(&domainscript,"","","","","","","","","",&mut vmap);
            println!("Loading domain script:[{}]",&domainscript);
        }
    }


    loop {
        nscript_loops(&mut vmap);
        match listener.accept() {
            Ok((stream, _)) => {
                let remote_ip = stream.peer_addr().unwrap().ip();
                vmap.setvar("___thissocketip".to_owned(),&remote_ip.to_string());
                let onconfunc = "server.onconnect(\"".to_owned() + &remote_ip.to_string()+ "\")";
                nscript_checkvar(&onconfunc,&mut vmap);
                handle_connection(stream,&mut vmap);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No incoming connections yet,
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
