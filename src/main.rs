//use core::num::dec2flt::parse::parse_number;
// Nscript v2 ( remade from au3 nscript) by Nick Hagen.
use std::collections::{HashMap};
//use std::{env, array, string};
use std::fs;
use std::fs::File;
use std::path::{Path, PrefixComponent};
use std::io::{self,Read, Write};
use std::process;
use std::process::{Command, exit};
use std::time::{Duration};

use colored::Colorize;

use rand::seq::SliceRandom;
use encoding_rs::{ UTF_8};

//time
use chrono::{Datelike, Timelike};

use std::{
    net::{TcpListener, TcpStream},
};
use reqwest;
use hex::FromHex;
//use regex::Regex;
use std::thread;
//mod nscriptapilib;
mod includes {
    pub mod zip;
    pub mod nscriptapilib;
    pub mod nscript_functions;

}
use includes::zip::*;
use includes::nscriptapilib::*;
use includes::nscript_functions::*;


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

#[cfg(not(windows))]
use std::os::unix::io::AsRawFd;
const NSCRIPT_VERSION: &'static str = "v2.001";
const NSCRIPT_INFO: &'static str = "
Nscript core in Rust-language.
Created by Nick Hagen.
2022-23";
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

const PROGRAM_DIR: &str = env!("CARGO_MANIFEST_DIR");
const NC_ARRAY_DELIM : &str = "]].n.c.arr.[[";
const NC_ASYNC_LOOPS_KEY : &str = "while"; // async loops scopes keyword



fn nscript_interpreterdebug(debugstr: &str,dbg_mode: usize,res_mode: usize){
    if dbg_mode > 0 {
        let debugstring = "Nscript-Debug:".to_owned() + &debugstr ;
        cwrite(&debugstring,"yellow");
    }

    if res_mode > 0 {
        let debugstring = "Nscript-Runtime-restriction-exit:".to_owned() + &debugstr ;
        cwrite(&debugstring,"red");

        process::exit(2);
    }
}
struct Njh {

}

impl Njh {
    // a clone of the first functions i ever wrote back in 2008.
    // it saves a header with a entree to a .njh file
    // it splits by lines1, if found next line be result
    // load("#name"1,filename) / save("#name"1,namevar1,filename)
    // can be used to fastly load settings for prorgams.
    pub fn write(header: &str,data: &str,file: &str) {
        let dataf = Nfile::read(&file);
         Nfile::write(&file,&Njh::writeinvar(&header,&data,&dataf));
    }
    pub fn writeinvar(header: &str,newln:&str,data: &str) -> String{
        let mut check = false;
        let mut vec: Vec<&str> = vec![];
        let mut isfound = false;
        for line in data.lines() {
            if check == true {
                vec.push(newln);
                check = false; //done
                isfound = true;
            }else {
              vec.push(line);
            }
            if line == header {
                check = true;
            }
        }
        let mut outputdata = String::new();
        for lines in vec {
            outputdata = outputdata + lines + &LINE_ENDING;
        }
        if isfound == false{
            outputdata = outputdata  + header + &LINE_ENDING + newln+ &LINE_ENDING;
        }
        return outputdata;
    }

    pub fn read(header: &str,file: &str) -> String {
        let data = Nfile::read(file);
       return Njh::readinvar(header,&data);
    }

    pub fn readinvar(header: &str,data: &str) -> String {
        let mut check = false;
        for line in data.to_owned().lines() {
            if check == true {
                return line.to_owned();
            }
            if line == header {
                check = true;
            }
        }
        return "@error".to_owned();
    }
}
struct Varmap{
    //global values of the vmap system
    varmap: HashMap<String,String>,
    codelevel: usize,
    parsinglevel: usize,
    debugmode: usize,
    strictness: usize
}
impl Varmap {
    // this is the variable /class storage and manage structure all the functions to save load copy
    // and stuff are inside this impl
    // ------------------------------
    pub fn new() -> Varmap {
        Varmap{
            varmap: HashMap::new(),
            codelevel: 1,
            parsinglevel: 1,
            debugmode: 0,
            strictness: 0
       }
     }

    pub fn stackpush(&mut self,stackref: &str,data: &str){
        // stack push, this is for nscript stacks, ( can also be used internally)
        let thisstack = "stack__".to_owned() + stackref;
        let height = self.getprop(&thisstack, "height");
        let newheight = nscript_i32(&height) + 1;
        self.setprop(&thisstack,&newheight.to_string(),&data);
        self.setprop(&thisstack,"height",&newheight.to_string());
    }

    pub fn stackpop(&mut self,stackref: &str) -> String {
        // nscript stack pop, nscript stacks.
        let thisstack = "stack__".to_owned() + stackref;
        let height = self.getprop(&thisstack, "height");
        let data = self.getprop(&thisstack, &height.to_string());
        let mut newheight = nscript_i32(&height) - 1;
        if newheight < 0 {
            newheight = 0;
        }
        self.setprop(&thisstack,"height",&newheight.to_string());
        self.delprop(&thisstack, &height.to_string());
        return data;
    }

    fn setobj(&mut self,obj: &str, toobj: &str) {
        let trimmedobj = &obj.trim();
        let trimmedtoobj = &toobj.trim();
        let getoldprops = self.inobj(&trimmedobj);
        let splitprops= split(&getoldprops,"|");
        for prop in splitprops {
            let key = "".to_owned() + &trimmedobj + "." + &prop;
            let get = self.getvar(&key);
            let keynew = "".to_owned() + &trimmedtoobj + "." + prop;
            //println!("setting prop:{} with vallue {}",&keynew.yellow(),&get.as_str().red());
            self.setvar(keynew, get.as_str());

        }
        // copy function register

        let functionregobj = "nscript_classfuncs__".to_owned() + &trimmedobj;
        let getoldprops = self.inobj(&functionregobj);
        let splitfn = split(&getoldprops,"|");
        for prop in splitfn {
            let functionregobj = "nscript_classfuncs__".to_owned() + &trimmedobj ;//+ "__" + &prop;
            let functionregobjnew= "nscript_classfuncs__".to_owned() + &trimmedtoobj;// + "__" + &prop;

            let get = self.getprop(&functionregobj,&prop);
            self.setprop(&functionregobjnew, &prop,get.as_str()) ;
            //println!("Assigning function ( {} ) to obj: ( {} ) ",get,toobj)
        }

        // Parents and childs
        // add parent to new obj
        let objparenth =  "nscript_classparents__".to_owned() + &trimmedtoobj;
        self.setprop(&objparenth, &trimmedobj, "active");
        // add child to parent obj
        let objchildh =  "nscript_classchilds__".to_owned() + &trimmedobj;
        self.setprop(&objchildh, &trimmedtoobj, "active");

        //vmap.setvar(functionregobj, &funcname); // reg the function to obj
    }
    fn inobj(&mut self,obj:&str) -> String {
        let isobj = "obj_".to_owned() + &obj.trim();
        let g = self.varmap.get_key_value(&isobj);
        match g {
            None => String::new(),
            Some((_i, k)) => k.to_owned()
        }
    }
    fn delobj(&mut self,obj: &str){

        //delete properties
        let getallprops = self.inobj(obj.trim());
        let allprops = split(&getallprops,"|");
        for prop in allprops {
            //println!("deleting prop {}",&prop);
            self.delprop(&obj,&prop);
        }
        //delete function register
        let functionregobj = "nscript_classfuncs__".to_owned() + &obj ;
        let getallfuncs = self.inobj(&functionregobj);
        let allfuncs = split(&getallfuncs,"|");
        for prop in allfuncs {
            self.delprop(&functionregobj,&prop);
            //println!("deleting func {}",&prop);
        }
        // delete children/parents register
    }
    fn delprop(&mut self,obj: &str,key: &str){
        if key == ""{
            return
        }
        let objname = &obj.trim();
        let propname = &key.trim();
        let fullkey = "obj_".to_owned() + &objname + "__" + &propname;
        self.varmap.insert(fullkey.to_owned(),"".to_owned());  // clear vallue.. set none


        let objprops = "obj_".to_owned() + &objname; // index of all the properties - managed
        let g = self.varmap.get_key_value(&objprops);
        match g {
            None => {
                let dbgmsg = "A property is being deleted wich doesnt exist in the object; ".to_owned() + &fullkey;
                nscript_interpreterdebug(&dbgmsg,self.debugmode, self.strictness)
                // if it ever gets here then you messed up, exsisting objects&props have indexes.

            },
            Some((_i, k)) => {
                let array = split(&k,"|");
                let mut newindex = String::new();
                for entree in array {
                    if entree != key && entree != "" {
                        newindex = "".to_owned() + &entree + "|";

                    }


                }
                if Nstring::fromright(&newindex, 1) == "|"{
                    newindex = Nstring::trimright(&newindex,1);
                }
                self.varmap.insert(objprops,newindex);

            }
        }
    }
    fn setvar(&mut self,key: String , value: &str){
        // this is the core function for storing all the data of the nscript code syntax.
        // all user script variables and classproperties go thru this
        // -----------------------------------------------------------
            if Nstring::instring(&key.trim(),".") == true { // obj property
            let spl = split(&key.trim(),".");
            let mut objname = String::new();

            let mut propname = String::new();
            if Nstring::instring(&spl[0],"&") {
                objname = self.getvar(&Nstring::replace(&spl[0],"&",""));
            }
            else {
                objname = "".to_owned() + &spl[0];
            }

            if Nstring::instring(&spl[1],"&") {
                propname = self.getvar(&Nstring::replace(&spl[1],"&",""));
          }
            else {
                propname = "".to_owned() + &spl[1];
            }
            let fullkey = "obj_".to_owned() + &objname.to_string() + "__" + &propname.to_string();
           self.varmap.insert(fullkey,value.to_owned());
           let objprops = "obj_".to_owned() + &objname.to_string(); // index of all the properties - managed
            let g = self.varmap.get_key_value(&objprops);
            match g {
                None => {
                    // add new prop as first index to obj's properties index
                    self.varmap.insert(objprops,propname.to_owned());
                },
                Some((_i, k)) => {
                    let tosearch = propname.to_string() + "|";// make sure for search
                    if Nstring::instring(&k, &propname) == false {
                        let nexindex = k.to_owned() + "|" + &propname;
                        self.varmap.insert(objprops,nexindex.to_owned());
                    }

                }
            }

        }
        else{
            let keyname = "v_".to_string() + &key.trim();

            // if Nstring::instring(&keyname,"_internalparam") {
            //    println!("setvar() fullkeyobj:{} with value {}",&keyname.yellow(),&value.red());
            // }
            self.varmap.insert(keyname,value.to_owned());
        }

      }
    fn getvar(&mut self,key: &str)->String{
        // this is the core function of nscript to get data
        // it will check for variables or class.properties
        // ----------------------------------------------------
        if Nstring::instring(&key,".") == true { // obj property
            let spl = split(&key,".");
            let mut objname = String::new();

            let mut propname = String::new();
            if Nstring::instring(&spl[0],"&") {
                objname = self.getvar(&Nstring::replace(&spl[0],"&",""));
           }
            else {
                objname = "".to_owned() + &spl[0];
            }
           if Nstring::instring(&spl[1],"&") {
                propname = self.getvar(&Nstring::replace(&spl[1],"&",""));

            }
            else {
                propname = "".to_owned() + &spl[1];
            }


            //let propname = self.checkvar(&spl[1]);
            let fullkey = "obj_".to_owned() + &objname.to_string() + "__" + &propname.to_string();
            //println!(" getvar() fullkeyobj:{}",&fullkey.red());
            let g = self.varmap.get_key_value(&fullkey);
            match g {
                None =>{
                let dbgmsg = "Undeclared property being called; ".to_owned() + &fullkey;
                    nscript_interpreterdebug(&dbgmsg,self.debugmode, self.strictness);
                    String::new()
                } ,
                Some((_i, k)) => k.to_owned()
            }
        }
        else { // else normal var
            let keyname = "v_".to_string() + &&key;
            let g = self.varmap.get_key_value(&keyname);
            match g {
                None => {
                let dbgmsg = "Undeclared variable being called; ".to_owned() + &keyname;
                    nscript_interpreterdebug(&dbgmsg,self.debugmode, self.strictness);
                    String::new()
                },
                Some((_i, k)) => k.to_owned(),
            }
        }
    }
    fn getprop(&mut self, obj: &str, prop: &str) -> String {
        let fullkey = "obj_".to_owned() + &obj.to_string().trim() + "__" + &prop.to_string().trim();
        //println!("fullkeyobj:{}",&fullkey.red());
        let g = self.varmap.get_key_value(&fullkey);
        match g {
            None => String::new(),
            Some((_i, k)) => k.to_owned(),
        }
    }
    fn setprop(&mut self, obj: &str, prop: &str, value: &str) {
        let fullkey = "obj_".to_owned() + &obj.to_string().trim() + "__" + &prop.to_string().trim();
        self.varmap.insert(fullkey, value.trim().to_owned());

        // set obj index !!
        let objprops = "obj_".to_owned() + &obj.trim().to_string(); // index of all the properties - managed
        let g = self.varmap.get_key_value(prop.trim());
        match g {
            None => {
                // add new prop as first index to obj's properties index
                self.varmap.insert(objprops, prop.trim().to_owned());
            }
            Some((_i, k)) => {
                //let isind = k.to_owned() + "|"; // make sure for search
                //let tosearch = prop.to_string() + "|";
                if Nstring::instring(&k, &prop.trim()) == false {
                    let nexindex = k.trim().to_owned() + "|" + &prop.trim();
                    self.varmap.insert((&prop.trim()).to_string(), nexindex.to_owned());
                }
            }
        }
    }
    fn objparents(&mut self, obj: &str) -> String {
        let key = "nscript_classparents__".to_owned() + obj;
        let g = self.inobj(&key);
        return g;
    }
    fn objchildren(&mut self, obj: &str) -> String {
        let key = "nscript_classchilds__".to_owned() + obj;
        let g = self.inobj(&key);
        return g;
    }

    fn setcode(&mut self, name: &str, code: &str) {
        // interally used to store codesheet/blocks
        // ----------------------------------------
        let codename = "code__".to_owned() + name;
        self.varmap.insert(codename, code.to_owned());
    }

fn getcode(&mut self, name: &str) -> String {
    // retrieves a nscript block ( interally used )
   // -----------------------------------------
    let codename = "code__".to_owned() + name;
    let g = self.varmap.get_key_value(&codename);
    //println!("GetCode() {}", &codename);
    let result = match g {
        None => {
            //println!("Result is None={}",&codename);
            String::new()
        }
        Some((_i, k)) => {
            let result = k.to_owned();
            //println!("Result is Some: {}", result);
            result
        }
    };
    result
}
    fn codelvlup(&mut self) {
        // this is very important, the code level is nessesary for the internalparams
        // when nscript recurses from one function to another the arguments/parameters
        // are set on the background internalparam1-10 they get a prefix with this number
        // making them act like a stack during recursing, so when a function goes back
        // the parameters are still there.
        // ------------------------------------------------------------------------
        self.codelevel = self.codelevel + 1
    }
    fn codelvldown(&mut self) {
        // this happends when parsesheet() is done.
        // when you called a function and it finished the function will go back level
        // resetting the arguments you had used during this function.
        // -------------------------------------------------------------------------
        if self.codelevel == 0 {
            return
        }
        for r in 0..10 {
            let paramx = r + 1;
            let pname = "".to_owned() + &self.codelevel.to_string() + "__internalparam" + &paramx.to_string();
            self.setvar(pname, ""); // clear all param arguments
        }
        self.codelevel = self.codelevel - 1
    }
    fn iscodelevel(&mut self) -> String {
        self.codelevel.to_string()
    }
}
fn nscript_clearparams_handleconnections(vmap: &mut Varmap){
    // clears params without going level down
        for r in 0..10 {
            let paramx = r + 1;
            let pname = "".to_owned() + &vmap.codelevel.to_string() + "__internalparam" + &paramx.to_string();
            vmap.setvar(pname, ""); // clear all param arguments
        }
}
//----------------RegionNscript------------------\/--------------
fn is_number(input: &str) -> bool {
    input.parse::<f64>().is_ok()
}

fn is_float(input: &str) -> bool {
    input.parse::<f32>().is_ok() || input.parse::<f64>().is_ok()
}

fn nscript_checkvar(key: &str,vmap: &mut Varmap) -> String {
    // this function can evaluate a words as nscript syntax and returns the evaluated value
    // -----------------------------------------------------------------
    //
    let mut checkvar_toreturn = String::new();
    if key == "" || key == "\"\""{
        return checkvar_toreturn;
    }
    //println!("key={}",&key);
    if is_float(&key) || is_number(&key) || key == "2"{
       // println!("Isnumber checkvar() {}",&key);
        return key.to_string();
    }
    match &key[0..1] {
        "\"" => {
            // this one shoulnd be here1, stringextract should work but somehow it can bug if so
            // this should return the string as is to the parser1,
            checkvar_toreturn = Nstring::trimright(&Nstring::trimleft(&key, 1),1);
        }
        "$" => {
            checkvar_toreturn = vmap.getvar(key);
        }
        "-" => {
            checkvar_toreturn = key.to_string();
        }

        "@" => {
            checkvar_toreturn = nscript_getmacro(&key,vmap);
        }
        "_" => {
            if Nstring::instring(&key,"(") && Nstring::instring(&key,")") {
                checkvar_toreturn = nscript_func(&nscript_funcextract(&key, vmap),vmap);

            }
            else {
                checkvar_toreturn = key.to_string();
            }
        }
        "^" => {
            checkvar_toreturn = hex_to_string(&Nstring::replace(&key,"^",""));
        }
        "%" => {
            checkvar_toreturn = key.to_string();
        }
        _ => {
            if Nstring::instring(&key,"(") && Nstring::instring(&key,")") {
                if vmap.getcode(  &Nstring::replace(&split(&key,"(")[0],".","__")) != "" {
                    //println!("a func found on a call");
                    checkvar_toreturn = nscript_func(&nscript_funcextract(&key, vmap),vmap);
                }
                else {
                    let rawargs = Nstring::stringbetween(&key, "(", ")");
                    let fnname = split(&key, "(")[0];
                    let argsplit = split(&rawargs, ",");
                    let mut argvec = Vec::new();
                    for r in 0..10 {
                        if argsplit.len() > r {
                            let evalvar = nscript_checkvar(&argsplit[r], vmap);
                            argvec.push(evalvar);
                        } else {
                            argvec.push("".to_owned());
                        }
                    }

                    checkvar_toreturn = nscript_callfn(&fnname,&argvec[0],&argvec[1],&argvec[2],&argvec[3],&argvec[4],&argvec[5],&argvec[6],&argvec[7],&argvec[8],vmap);
                }
            }
            else {
                //checkvar_toreturn = key.to_string();
                if Nstring::instring(&key, "[") && Nstring::instring(&key, "]") {
                    let getref = split(&key,"[")[0];
                    let isref: Vec<&str> = getref.split("[").collect();
                    let arrid = Nstring::stringbetween(&key, "[", "]");
                    let getthisarray = vmap.getvar(&getref);
                    let thisarray : Vec<&str> = getthisarray.split(NC_ARRAY_DELIM).collect();
                    if arrid == "?" {
                        return "".to_owned() + &thisarray.len().to_string();
                    }
                    if let Ok(number) = arrid.parse::<usize>() {
                        if number > thisarray.len() {
                            return String::new();
                        }
                        return "".to_owned() + thisarray[number];
                    }
                    return String::new();
                }
                else {
                    checkvar_toreturn= vmap.getvar(key);
                    return checkvar_toreturn;
                }



            }
        }
    }
    checkvar_toreturn
}

fn nscript_getmacro(mac: &str,vmap: &mut Varmap) -> String {
    let time = chrono::Utc::now();
    match mac {
        "@webpublic" => SCRIPT_DIR.to_owned() +"domains/" +&split(&vmap.getvar("___domainname"),":")[0]+"/public/",
        "@webprivate" => SCRIPT_DIR.to_owned() +"domains/" + &split(&vmap.getvar("___domainname"),":")[0]+"/private/",
        "@webroot" => SCRIPT_DIR.to_owned() +"domains/" + &split(&vmap.getvar("___domainname"),":")[0]+"/",
        "@year" => time.year().to_string(),
        "@month" => time.month().to_string(),
        "@day" => time.day().to_string(),
        "@hour" => time.hour().to_string(),
        "@min" => time.minute().to_string(),
        "@OS" => MACRO_OS.to_string(),
        "@scriptdir" => SCRIPT_DIR.to_string(),
        "@programdir" => PROGRAM_DIR.to_string(),
        "@sec" => time.second().to_string(),
        "@msec" => time.timestamp_millis().to_string(),
        "@socketip" => nscript_checkvar("___socketip",vmap),
        "@nscriptversion" => String::from(NSCRIPT_VERSION),
        "@crlf" => String::from(LINE_ENDING),
        "@lf" => String::from(LINE_ENDING),
        "@emptystring" => String::new(),

        _ => String::from(mac),
    }
}

fn nscript_switch(entree: &str,scope:&str,vmap: &mut Varmap) -> String {
    let scopeargs = Nstring::stringbetween(&scope, "(", ")");
    let splitscopearg = split(&scopeargs,",");
    let evalentree = nscript_checkvar(entree, vmap);

    let switchscope = nscript_unpackscopereturnclean(&splitscopearg[1],&splitscopearg[0],vmap);
    let splitcase = split(&switchscope,"case ");
    for thiscase in splitcase {
        let casescope = split(&thiscase," ");
        if casescope.len() > 1 {
            let evalvarname = nscript_checkvar(&casescope[0],vmap);
            //println!("Evalcheck:{} evalcase:{}",&evalvarname,&evalentree);
            if evalvarname == evalentree {
//println!("Switch True! scope={}",&casescope[1]);

                 let casescopeargs = Nstring::stringbetween(&casescope[casescope.len() -1], "(", ")");
                let splitcasescopearg = split(&casescopeargs,",");
                return nscript_unpackscope(&splitcasescopearg[1],&splitcasescopearg[0],vmap);

            }
        }
    }

String::new()
}


fn nscript_array(entrees: &str,vmap: &mut Varmap ) -> String{
    if Nstring::fromleft(&entrees,1) == "[" && Nstring::fromright(&entrees,1) == "]" {
        let parseall = Nstring::stringbetween(&entrees,"[", "]");
        let delimiter = ",";

     let parsed: Vec<&str> = parseall.split(delimiter).collect();
        let mut returnstring = String::new();
        for each in &parsed {
            if returnstring == "" {
                returnstring ="".to_owned() + &nscript_checkvar(&each,vmap);

            }
            else{
                returnstring = "".to_owned() + &returnstring + &NC_ARRAY_DELIM + &nscript_checkvar(&each,vmap);
            }

        }
        return returnstring;
    }
    return String::new();

}

fn line_to_words(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}
fn nscript_parsecompiledsheet(coderaw: &str, vmap: &mut Varmap) -> String {
    // this function runs a block of scope without jumping up a codelvl
    // this is used for at spot runtime blocks like For,While loops.
    // it is faster for execution then parse_sheet() but it requires nscript_compilesheet()
    // to have preparred the block for proper execution
    // ------------------------------------------------------------------------
    let lines = coderaw.split("\n");
    for line in lines {
        if line != "" {
            let toreturn = &nscript_parseline(&split(&line,"//")[0].trim(), vmap);
            if Nstring::instring(toreturn, "RET=>") == true {
               return Nstring::replace(toreturn, "RET=>", "");
            }
        }
    }

    return String::from("..");
}

fn nscript_parsesheet(coderaw: &str, vmap: &mut Varmap) -> String {
    // this is the interal function to parse a prepare and parse a block
    // it is used on nscript_execute_script() / exec()
    // this function does not extract classes or function scopes!
    // if you need to run a new script see nscript_exec_script() this is where it starts
    // -------------------------------------------------------------------------------
    let argnew = "".to_owned() + &vmap.codelevel.to_string() + "__internalparam"; // form new varnames bkgrnd paramx
    let levelbellow = vmap.codelevel - 1 ;
    let argnewbroken = "".to_owned() + &levelbellow.to_string()  + "__internalparam"; // form new varnames bkgrnd paramx
    let argnewfix = "".to_owned() + &levelbellow.to_string() + "__" + &vmap.codelevel.to_string() + "__internalparam"; // form new varnames bkgrnd paramx
    //println!("rawcode:{}",&coderaw);
    let code = kill_bill(&Nstring::replace(&coderaw, "internalparam", &argnew));

    let code = Nstring::replace(&code, &argnewfix, &argnewbroken);
    //println!("newcode:{}",&code);
    vmap.codelvlup();

    let fixedcode = code.to_owned();// + LINE_ENDING;
     let fixedcode = trim_lines(&fixedcode);
     let fixedcode = nscript_stringextract(&fixedcode);
     let fixedcode  = nscript_scopeextract(&fixedcode);
    // println!("parsingcode:{}",&fixedcode);
    let mut toreturn = String::new();
    let lines = fixedcode.split("\n");
    for line in lines {
        if line != "" {
            let fixedline = split(&line,"//")[0].trim();
            if fixedline != ""{
                toreturn = nscript_parseline(&fixedline ,vmap);
            }
            //  when parse line sees return on word[0] it will add "RET=>"
            // this will break this loop and return the value back to callfn/nscript_func
            if Nstring::fromleft(&toreturn, 5) == "RET=>" {
                vmap.codelvldown();

                return Nstring::trimleft(&toreturn, 5);
            }
        }
    }
    vmap.codelvldown();
     // if the loop does not break early and the end of file is reached with no return value
    return String::from("..");
}

fn nscript_parseline(line: &str, vmap: &mut Varmap) -> String {
   // allright this be the most core mechanic function of them all this is the core interpreter
    // this function parses a line and evaluate its logic
    // array words[] represents a split by white spaces of the script
    // the script is trimmed and all double whitespaces should be removed at this point.
    //  the first match will see how many words the line has.
    //  well its a very complex nested structed of match checks!
    //  --------------------------------------------------------------------------------
    let mut parseline_toreturn = String::new(); // result of the line change if required
    let words = line_to_words(&line);
    //words = split(&line," ");
    // println!("line lenght in words:{}",&words.len());
    match words.len() {
        1 => {
            // 1 word lines
            let pref = nscript_getprefix(&words[0]);
            match pref.as_str() {
                // these are checks for 1 word lines ( internally this can be
                // triggered without parsesheet()
                "call" => {
                    if Nstring::instring(&words[0],"scope(") { // <----------- interally used

                        let scopeargs = Nstring::stringbetween(&words[0], "(", ")");
                        let splitscopearg = split(&scopeargs,",");
                        return nscript_unpackscope(&splitscopearg[1],&splitscopearg[0],vmap);
                    }
                    else {
                        if split(&words[0],"(").len() > 2 {
                           let unwrap = nscript_funcextract(&words[0], vmap);
                            return nscript_runfncall(&unwrap, vmap);
                        }
                        return nscript_runfncall(&words[0], vmap);
                    }

                }
                "function" => {
                    if split(&words[0],"(").len() > 2 {
                        let unwrap = nscript_funcextract(&words[0], vmap);
                        return nscript_func(&unwrap, vmap);
                    }
                    return nscript_func(&words[0], vmap);
                }
                "int" => {
                    return words[0].to_string();
                }
                "string" => {
                    return hex_to_string(&Nstring::replace(&words[0],"^",""))
                }
                "exit" => {
                    process::exit(1);
                }
                _ => {
                    //unknown
                }
            };
        }
        2 => {
            // 2 word lines
            match words[0]{
                "break"|"Break" => {
                    let loopid = nscript_checkvar(&words[1], vmap);
                    vmap.delprop("nscript_loops", &loopid);
                    return String::new();
                }
                "return"|"Return" => {
                    return "RET=>".to_owned() + &nscript_checkvar(words[1],vmap);

                }
                "else" => {
                    if vmap.getvar(&"___if") == "false"{
                        let scopeargs = Nstring::stringbetween(&words[1], "(", ")");
                        let splitscopearg = split(&scopeargs,",");
                        return nscript_unpackscope(&splitscopearg[1],&splitscopearg[0],vmap);
                    }
                    return String::new();
                }

                _ => {
                    //return String::new();
                }
            }
            match words[1] {
                "++" => {
                    let newnumber = nscript_math(&words[0],"+","1",vmap);
                    vmap.setvar(words[0].to_string(),&newnumber );
                    return "".to_owned();
                }
                "--" => {
                    let newnumber = nscript_math(&words[0],"-","1",vmap);
                    vmap.setvar(words[0].to_string(),&newnumber );
                    return "".to_owned();
                }
                _ => {
                    return "".to_owned();
                }
            }

        }
        3 => {
            // lines that be 3 word
            match words[0] {
                "switch" => {

                    return nscript_switch(&words[1],&words[2],vmap);
                }
                NC_ASYNC_LOOPS_KEY => {
                    let scopeargs = Nstring::stringbetween(&words[words.len()-1], "(", ")");
                    let splitscopearg = split(&scopeargs,",");
                    let loopref = nscript_checkvar(&words[1],vmap);

                    let loopscope = nscript_compilesheet(&nscript_unpackscopereturnclean(&splitscopearg[1],&splitscopearg[0],vmap));
                    vmap.setvar("nscript_loops".to_owned() + "." + &loopref.trim(), &loopscope);
                    return "".to_owned();

                }
                _ => {
                    //..
                }
            }
            let pref = nscript_getprefix(&words[0]);
            match pref.as_str() {
                "var" => {

                    //---------------------------------
                    match words[1]{
                    "=" =>{
                        // checked $var = *
                            let pref2 = nscript_getprefix(&words[2]);
                            match pref2.as_str() {
                                // checking the *
                                "macro" => {
                                    let ismacro = nscript_getmacro(words[2],vmap);
                                    vmap.setvar(words[0].to_string(),&ismacro );
                                    return words[2].to_string();
                                }
                                "int" => {
                                    vmap.setvar(words[0].to_string(), &words[2].to_string());
                                    return words[2].to_string();
                                }
                                "array" => {
                                    let isret = nscript_array(&words[2], vmap);
                                    vmap.setvar(words[0].to_string(), &isret);
                                    return isret;
                                }
                                "call" => {
                                    if split(&words[2],"(").len() > 2 {
                                        let unwrap = nscript_funcextract(&words[2], vmap);
                                        let res = nscript_runfncall(&unwrap, vmap);
                                        vmap.setvar(words[0].to_string(), &res.to_string());
                                        return res;
                                    }
                                    let res = nscript_runfncall(&words[2], vmap);
                                    vmap.setvar(words[0].to_string(), &res.to_string());
                                    return  res;

                                }
                                "string" => {
                                    vmap.setvar(words[0].to_string(), &hex_to_string(&Nstring::replace(&words[2],"^","")) );
                                    return "".to_owned();
                                }
                                "function" => {
                                    if split(&words[2],"(").len() > 2 {
                                        let unwrap = nscript_funcextract(&words[2], vmap);
                                        let ret = nscript_func(&unwrap, vmap);
                                        vmap.setvar(words[0].to_string(),&ret);
                                    }
                                    else{
                                        let funcret = nscript_func(&words[2], vmap);
                                        vmap.setvar(words[0].to_string(),&funcret);
                                        return funcret
                                    }
                                }
                                _ => {
                                    //more opt
                                }
                            }
                        }
                        // math functions, bellow you will see easified syntax
                        //  var += 10 will add 10 to var.
                        "+=" => {
                            let newnumber = nscript_math(&words[0],"+",&words[2],vmap);
                            vmap.setvar(words[0].to_string(),&newnumber );
                            return "".to_owned();
                        }
                        "-=" => {
                            let newnumber = nscript_math(&words[0],"-",&words[2],vmap);
                            vmap.setvar(words[0].to_string(),&newnumber );
                            return "".to_owned();
                        }
                        "/=" => {
                            let newnumber = nscript_math(&words[0],"/",&words[2],vmap);
                            vmap.setvar(words[0].to_string(),&newnumber );
                            return "".to_owned();
                        }
                        "*=" => {
                            let newnumber = nscript_math(&words[0],"*",&words[2],vmap);
                            vmap.setvar(words[0].to_string(),&newnumber );
                            return "".to_owned();
                        }


                        _ => {
                            //rror--return "".to_owned();
                        }

                    }
                    //---------------------------------

                    let result = nscript_runfncall(&words[0], vmap);
                    return result;
                }
                _ => {
                    //undone

                }
            };
        }
       _ => { // all stuff bigger then 3 words goes bellow here, we cant check on the words anymore
            // as the syntax gets more complex. we do some new checks to see what to do.
           // ---------------------------------------------------------------------------
            if words.len() > 2 {
                match words[2] { // <-- we check the 3th word
                    "math" => {
                        let res = nscript_runmath(&words,3, vmap);
                        //println!("Mathresult:{}",res);
                        vmap.setvar(words[0].to_string(), &res);
                        return res;
                    }
                    "combine" => {
                        let res = nscript_combine(&words, vmap);
                        //println!("Combine:{}", res);
                        vmap.setvar(words[0].to_string(), &res);
                        return res;
                    }
                    "space" => {
                        let res = nscript_space(&words, vmap);
                        //println!("Combine:{}", res);
                        vmap.setvar(words[0].to_string(), &res);
                        return res;
                    }
                    "string" => {
                        let res = nscript_space(&words, vmap);
                        //println!("Combine:{}", res);
                        vmap.setvar(words[0].to_string(), &res);
                        return res;
                    }
                    _ => {
                        // multi syntax lines.
                    }
                }
            }
             if words.len()  > 3 {
                // syntax for object spawning1,
                if words[0] == "obj" && words[2] == ":" {
                    let obj1 = nscript_checkvar(&words[3],vmap);
                    let obj2 = nscript_checkvar(&words[1],vmap);
                    if obj2 == "" {
                        vmap.setobj(&obj1,&words[1]);
                    }
                    else{
                        vmap.setobj(&obj1,&obj2);

                    }

                    return String::new();
                }
            }
             if words.len() > 4 { // lines that are of 5 words or more
                match words[0]{
                    // for loops
                    "elseif" =>{
                        if vmap.getvar(&"___if") == "false"{ // last if statement must be false
                            if parse_and_check_statement(&words,vmap){
                                let scopeargs = Nstring::stringbetween(&words[words.len()-1], "(", ")");
                                let splitscopearg = split(&scopeargs,",");
                                return nscript_unpackscope(&splitscopearg[1],&splitscopearg[0],vmap);
                            }
                        }
                        return String::new();
                    }
                    "for" => {
                        match words[2]{
                            "in" => {
                                nscript_foreach(&words[4], &words[1],&words[3], vmap)
                            }
                            "to" => {
                                nscript_forto(&words[4], &words[1],&words[3], vmap)
                            }
                            _ =>{
                                println!("Syntax error on a for loop; cannot determine method, check [for x to|in array]");
                                return "".to_owned();
                            }
                        }

                    }

                    "if" => {
                        // this handles a statement !
                        if parse_and_check_statement(&words,vmap){
                            //println!("newstatement true !");
                            vmap.setvar("___if".to_owned(),"true"); // <-- this is used for else
                            return nscript_parseline(&words[words.len()-1], vmap);
                        }
                        else {
                            //println!("newstatement false !");
                            vmap.setvar("___if".to_owned(),"false");//<- if false else{} can trigger!
                            return "".to_owned();

                        }
                    }
                    _ => {
                       //well not sure yet.
                    }
                }
                if words[2] == "switch" && words[1] == "=" {
                    // this is a switch scope with a variable set.
                    let switchreturn = nscript_switch(&words[3],&words[4],vmap);
                    println!("var = switch = {}",switchreturn);
                    vmap.setvar(words[0].to_owned(),&Nstring::trimleft(&switchreturn,5));
                    return "".to_owned();

                }

                if words[3] == "+" || words[3] == "-" || words[3] == "*" || words[3] == "/" || words[3] == "*" {
                    // this checks the 4th word to be any of the math syntax
                    // if so it will run the line as math,
                    // run_math(array,beginmathfromentree)
                    let res = nscript_runmath(&words,2, vmap);
                    vmap.setvar(words[0].to_string(), &res);
                    return res;

                }

            }
        }
    };
    // return parseline_toreturn;
    return String::new();
}
fn parse_and_check_statement(words: &[&str], vmap: &mut Varmap) -> bool {
    // this is how you parse a unknown lenght of statements
    // they can be mixed And/or
    // this function will return a bool.
    // -------------------------------------------------------------
    if words.len() < 4 || words[0] != "if" {
        nscript_interpreterdebug("There is a syntax error on a if statement", vmap.debugmode, vmap.strictness);
        return false; // Invalid syntax or empty statement
    }

    let conditions = &words[3..words.len() - 1];
    let mut index = 0;
    let mut result = nscript_checkstatement(words[1], words[2], words[3], vmap);
    if result{
        return result;
    }
    while index + 4 < conditions.len() {
        let operator = conditions[index];
        let a = conditions[index + 1];
        let b = conditions[index + 2];

        let c = conditions[index + 3];

        if operator == "and" || operator == "&&" {
            result = result && nscript_checkstatement(a, b, c, vmap);
        } else if operator == "or" || operator == "||" {
            result = result || nscript_checkstatement(a, b, c, vmap);
        } else {
            return false; // Unknown operator or invalid syntax
        }

        index += 4;
    }

    result
}


fn nscript_foreach(code: &str,invar: &str,inarray: &str,vmap: &mut Varmap) {
    // this function is used for something in array {}
    //------------------------------------------------------
    if inarray == "" {
        return
    }
    let evalarray = nscript_checkvar(&inarray,vmap);
    let array = split(&evalarray,&NC_ARRAY_DELIM);
    let scopeid = &Nstring::stringbetween(&code, "scope(", ",");
    let cleancode = nscript_compilesheet(&nscript_stringextract(&nscript_unpackscopereturnclean(&code, scopeid, vmap)));
    for isin in array {
        vmap.setvar(invar.to_owned(),&isin);
        nscript_parsecompiledsheet(&cleancode, vmap);
    }
}


fn nscript_forto(code: &str,invar: &str,inarray: &str, vmap: &mut Varmap) {
    // this is a for x to 100 {} system where x represents a number
    // -----------------------------------------------------------
    let evalarray = nscript_checkvar(&inarray,vmap);
    let scopeid = &Nstring::stringbetween(&code, "scope(", ",");

    let cleancode = nscript_compilesheet(&nscript_stringextract(&nscript_unpackscopereturnclean(&code, scopeid, vmap)));
    for isin in 1..nscript_i32(&evalarray)+1{
        vmap.setvar(invar.to_owned(),&isin.to_string());
        nscript_parsecompiledsheet(&cleancode, vmap);
    }
}



fn nscript_checkstatement(a: &str, b: &str, c: &str, vmap: &mut Varmap) -> bool {
    // this is used to check a single statement in nscript.
    // ---------------------------------------------------------------
    let mut ret = false;

        match b {
            "=" | "=="=> {
                if &nscript_checkvar(&a,vmap) == &nscript_checkvar(&c,vmap)  {

                    ret = true;
                    return ret;
                }
            }
            "!=" | "<>" => {
                if &nscript_checkvar(&a,vmap) != &nscript_checkvar(&c,vmap)  {

                    ret = true;
                    return ret;
                }

            }
            ">" => {
                if nscript_f64(&nscript_checkvar(&a,vmap) ) > nscript_f64(&nscript_checkvar(&c,vmap) ) {
                    ret = true;
                    return ret;
                }
            }
            ">=" => {
                if nscript_f64(&nscript_checkvar(&a,vmap) ) >= nscript_f64(&nscript_checkvar(&c,vmap) ) {
                    ret = true;
                    return ret;
                }
            }
            "<=" => {
                if nscript_f64(&nscript_checkvar(&a,vmap) ) <= nscript_f64(&nscript_checkvar(&c,vmap) ) {
                    ret = true;
                    return ret;
                }
            }

            "<" => {
                if nscript_f64(&nscript_checkvar(&a,vmap) ) < nscript_f64(&nscript_checkvar(&c,vmap) ) {
                    ret = true;
                    return ret;
                }
            }
            _ => {
                // error msg will be made.
            }
        }


    return ret;
}

fn nscript_combine(a: &Vec<&str>, vmap: &mut Varmap) -> String {
    // this is a function wich comes strings concetinate.
    // -------------------------------------------------
    let mut makestring = String::new();
    for r in 3..a.len() {
        makestring = makestring + &nscript_checkvar(&a[r],vmap);
    }
    return makestring;
}

fn nscript_space(a: &Vec<&str>, vmap: &mut Varmap) -> String {
    // everything is combined with a additional whitespace between them,
    // ---------------------------------------------------
    let mut makestring = String::new();
    for r in 3..a.len() {
        makestring = makestring + &nscript_checkvar(&a[r],vmap) + " ";
    }
    return Nstring::trimright(&makestring, 1);
}

// fn nscript_string(a: &Vec<&str>, vmap: &mut Varmap) -> String {
//     // kinda depreciated now theres a string "" system..
//     // yeah this comes from the old au3nc..
//     // -------------------------------------------
//     let mut makestring = String::new();
//     for r in 3..a.len() {
//         makestring = makestring + &a[r] + " ";
//     }
//     return Nstring::trimright(&makestring, 1);
// }

fn nscript_f64(intasstr: &str) -> f64 {
    // this is used on the nscript math system
    let onerr: f64 = 0.0;
    match intasstr.parse::<f64>() {
        Ok(n) => return n,
        Err(e) => return onerr,
    }
}

fn nscript_i32(intasstr: &str) -> i32 {
    let onerr: i32 = 0;
    match intasstr.parse::<i32>() {
        Ok(n) => return n,
        Err(e) => return onerr,
    }
}

fn nscript_math(a: &str, method: &str, b: &str, vmap: &mut Varmap) -> String {
    // this handles math operations from nscript. this is being looped in nscript_runmath()
    // in case of variables or calls return vallues be used.
    // ----------------------------------------------------------
    let a_val = &nscript_checkvar(&a,vmap);
    let b_val =  &nscript_checkvar(&b,vmap);
    let mut res: f64 = 0.0;

    match method {
        "+" => {
            res = nscript_f64(&a_val) + nscript_f64(&b_val);
        }
        "-" => {
            res = nscript_f64(&a_val) - nscript_f64(&b_val);
        }
        "/" => {
            res = nscript_f64(&a_val) / nscript_f64(&b_val);
        }
        "*" => {
            res = nscript_f64(&a_val) * nscript_f64(&b_val);
        }
        _ => {
            //
            let debugmsg = "Unexpected operator in math method".to_owned() + &a +" " + &method + " " + &b;
            nscript_interpreterdebug(&debugmsg,vmap.debugmode,vmap.strictness);
        }
    };
    //println!("math = {a} {b} {c} with result={r}",a = &a_val,b = &method, c = &b_val,r = &res);
    return res.to_string();
}
fn nscript_runmath(splitline: &Vec<&str>,indexpars: usize, vmap: &mut Varmap) -> String{
    // this will perform a line calculation
    // indexpars = where the math begins var = x + 1 mea word[2] is the beginning
    //----------------------------------------

    let mut index = indexpars; // begin after var =
    let mut result = nscript_math(&splitline[index] ,&splitline[index+1],&splitline[index+2],vmap);
    index +=2;
    while index < splitline.len()-1{
        result = nscript_math(&result ,&splitline[index+1] ,&splitline[index+2],vmap);
        index +=2;
    }
    result
}

fn nscript_getarguments(fnword: &str, vmap: &mut Varmap) -> (Vec<String>, usize) {
    // interall use for evaluating arguments passed on to functions
    // -------------------------------------------------------
    let cleaned = Nstring::replace(&fnword, "(", " ");
    let cleaned2 = Nstring::replace(&cleaned, ")", "           "); // additional whitespaces to ensure the size of vec split
    let cleaned3 = Nstring::replace(&cleaned2, ",", " ");
    let cmdlineraw = line_to_words(&cleaned3);
    if cmdlineraw.len() == 0 {
        return (Vec::new(),0)
    }
    let mut cmdline = Vec::new();
    let mut temp = String::new();
    let mut indx = 1;
    let mut fnresult = String::new();
    cmdline.push(cmdlineraw[0].to_string().clone());
    if cmdlineraw.len() > 1 {
        for _ in 1..cmdlineraw.len() {
            temp = nscript_checkvar(&cmdlineraw[indx],vmap);
            cmdline.push(String::from(temp));
            indx = indx + 1;
        }
    }
    (cmdline, cmdlineraw.len())
}

fn nscript_runfncall(fnword: &str, vmap: &mut Varmap) -> String {
    // yeah i know right this is pretty funny, first day of rust lol
    // yeah i gotta remake this i know !
    // ----------------------------------------------------------
    let  fnname = &fnword.to_string();
    if Nstring::instring(&split(&fnname,"(")[0],"&") {
        let fnsplit = split(split(&fnname,"(")[0],".");
        if fnsplit.len() <= 2 {

            if fnsplit.len() == 1 {
                let fnname = "".to_owned() + &nscript_checkvar(&Nstring::replace(&fnsplit[0], "&", ""),vmap)  + "(" + &split(&fnname,"(")[1] + ")";
            }
            if fnsplit.len() == 2 {
                let fnname = "".to_owned() + &nscript_checkvar(&Nstring::replace(&fnsplit[0], "&", ""),vmap) + "." + &nscript_checkvar(&Nstring::replace(&fnsplit[1], "&", ""),vmap) + "(" + &split(&fnname,"(")[1] + ")";
            }

        }
    }
    else {
        if vmap.getcode(&Nstring::replace(&split(&fnname,"(")[0],".","__")) != "" {
            return nscript_func(&fnname,vmap);
        }
    }
    //
    let mut fnresult = String::new();
    let (cmdline, numberargs) = &nscript_getarguments(&fnname, vmap);

    //println!("testarg:{a1} , {a2}", a1 = &cmdline[0],a2 = &cmdline[1]);

    match numberargs {
        1 => fnresult = nscript_callfn(&cmdline[0], "", "", "", "", "", "", "", "", "", vmap),
        2 => {
            fnresult = nscript_callfn(
                &cmdline[0],
                &cmdline[1],
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                vmap,
            )
        }
        3 => {
            fnresult = nscript_callfn(
                &cmdline[0],
                &cmdline[1],
                &cmdline[2],
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                vmap,
            )
        }
        4 => {
            fnresult = nscript_callfn(
                &cmdline[0],
                &cmdline[1],
                &cmdline[2],
                &cmdline[3],
                "",
                "",
                "",
                "",
                "",
                "",
                vmap,
            )
        }
        5 => {
            fnresult = nscript_callfn(
                &cmdline[0],
                &cmdline[1],
                &cmdline[2],
                &cmdline[3],
                &cmdline[4],
                "",
                "",
                "",
                "",
                "",
                vmap,
            )
        }
        6 => {
            fnresult = nscript_callfn(
                &cmdline[0],
                &cmdline[1],
                &cmdline[2],
                &cmdline[3],
                &cmdline[4],
                &cmdline[5],
                "",
                "",
                "",
                "",
                vmap,
            )
        }
        7 => {
            fnresult = nscript_callfn(
                &cmdline[0],
                &cmdline[1],
                &cmdline[2],
                &cmdline[3],
                &cmdline[4],
                &cmdline[5],
                &cmdline[6],
                "",
                "",
                "",
                vmap,
            )
        }
        8 => {
            fnresult = nscript_callfn(
                &cmdline[0],
                &cmdline[1],
                &cmdline[2],
                &cmdline[3],
                &cmdline[4],
                &cmdline[5],
                &cmdline[6],
                &cmdline[7],
                "",
                "",
                vmap,
            )
        }
        9 => {
            fnresult = nscript_callfn(
                &cmdline[0],
                &cmdline[1],
                &cmdline[2],
                &cmdline[3],
                &cmdline[4],
                &cmdline[5],
                &cmdline[6],
                &cmdline[7],
                &cmdline[8],
                "",
                vmap,
            )
        }
        10 => {
            fnresult = nscript_callfn(
                &cmdline[0],
                &cmdline[1],
                &cmdline[2],
                &cmdline[3],
                &cmdline[4],
                &cmdline[5],
                &cmdline[6],
                &cmdline[7],
                &cmdline[8],
                &cmdline[9],
                vmap,
            )
        }
        _ => fnresult = nscript_callfn("", "", "", "", "", "", "", "", "", "", vmap),
    };

    //println!("runfncall result:{}",&fnresult);
    return fnresult;
}

fn nscript_getprefix(s: &str) -> String {
    // this is used on the parse_line() to get what a syntax is instead of evaluating it.
    // this returns a string with the type of the nscript syntax
    // -----------------------------------------------------------------------------
    if is_float(&s) || is_number(&s) {
        return String::from("int");
    }
    let fchk = &split(&s,"(");
    if Nstring::instring(&fchk[0],".") && fchk.len() > 1 {
        return String::from("function");
    }
    //let mut ret = String::new();
    match &s[0..1] {
        "$" => return String::from("var"),
        "-" => return String::from("int"),

        "[" => return String::from("array"),

        "_" => return String::from("function"),
        "^" => return String::from("string"),
        "@" => return String::from("macro"),
        _ => {
          if Nstring::instring(&s, "(") == true && Nstring::instring(&s, ")") == true {
                return String::from("call");
            } else {
                return String::from("var");
            }
        }
    };
}

fn nscript_setdebugmode(id: &str,vmap: &mut Varmap){
    // this functions sets the console to show potentially unwanted behaviours of syntax.
    // if you are unfamilliar with reflecting functions with dynamic variable name references, and
    // want to code in a static style enable this it will show undeclared requests.
    // however ! nscript comes with the power to call or request unset things this can offer you
    // a flexible coding style and can lead to less if checks.
    // --------------------------------------------------------------
    match id {
        "1" => {
            vmap.debugmode = 1;
        }
        _ => {
            vmap.debugmode = 0;
        }
    }
}
fn nscript_setrestrictionmode(id: &str,vmap: &mut Varmap){
    // this sets nscript to exit the code if any unset var func class prop or thing is being used
    // before its declared. 0 = ignore line return "" 1 - exit 2
    //--------------------------------------------------------------------
    match id {
        "1" => {
            vmap.strictness = 1;
        }
        _ => {
            vmap.strictness = 0;
        }
    }
}

fn nscript_callfn(
    func: &str,
    param1: &str,
    param2: &str,
    param3: &str,
    param4: &str,
    param5: &str,
    param6: &str,
    param7: &str,
    param8: &str,
    param9: &str,
    vmap: &mut Varmap,
) -> String {
    // translate nscript calls towards the runtime functions and return vallues as string
    // all calls must be from String and back to String new adds be required to do so aswell
    // !! this is where you add your functions to the nscript syntaxis, if you have this included in your rust project
    // --------------------------------------------------------------------------------------
    match func {
        // "scope" => {
        //     return "RET=>".to_owned() + &nscript_unpackscope(param2,param1,vmap)
        // }
        "restrictionmode" => {
            nscript_setrestrictionmode(&param1,vmap);
            return "".to_owned() + &param1;
        }
        "debugmode" => {
            nscript_setdebugmode(&param1,vmap);
            return "".to_owned() + &param1;
        }
        "math" | "calc" => {
            let res = nscript_runmath(&split(&param1," "),0, vmap);
            return res;
        }
        "run" => {
            return call_program(&param1);
        }
        // "decrypt" => {
        //     return encrypt_string(&param1,&param2);
        // }
        //
        // "encrypt" => {
        //     return decrypt_string(&param1,&param2);
        // }
        "stringtobase64" => {
            return string_to_base64(&param1);
        }

        "base64tostring" => {
            return base64_to_string(&param1);
        }
        "unzip" => {
            return unzip_file(&param1,&param2);
        }
        "zip" => {
            return zip_directory(&param1,&param2);
        }

        "dirdelete" => {
            return directory_delete(&param1);
        }
        "dirmove" => {
            return directory_move(&param1,&param2);
        }
        "arraypush" => {
            return arraypush(&param1,&param2);
        }
        "arraypushroll" => {
            return arraypushroll(&param1,&param2);
        }
        "terminalinput" => {
            return terminal_get_user_input(&param1,&param2);
        }
        "discordmsg" => {
            send_message_to_discord_api(&param1, &param2);
            return String::new();
        }
        "filecopy" => {
            return filecopy(&param1,&param2);
        }
        "filedelete" => {
            return filedelete(&param1);
        }

        "filemove" => {
            return filemove(&param1,&param2);
        }
        "round" => {
            return round_number(&param1,&param2);
        }
        "random" => {
            return random_number_between(&param1,&param2,&param3);
        }
        "dircreate" => {
            return create_directory(&param1);
        }
        "filesizebytes" => {
            return filesizebytes(&param1);
        }

        "filesize" => {
            return filesize(&param1);
        }
        "curl" => {
            return curl(&param1);

        }
        "iscode" => {
            let ret = vmap.getcode(&param1);
            cwrite(&ret,"red");
            return String::from(&ret);

        }
        "pooladd" => {
            return pooladd(&param1,&&param2);
        }
        "poolremove" => {
            return poolremove(&param1,&param2);
        }
        "arraysort" => {
            return arraysort(&param1);
        }
        "stackpush" => {
            vmap.stackpush(param1, param2);
            return String::new();
        }
        "sleep" => {
            if let Ok(duration) = param1.parse::<u64>() {
                std::thread::sleep(std::time::Duration::from_millis(duration));
            } else {
                // Invalid argument, handle the error
                return String::from("Invalid argument for sleep function");
            }
           return String::from("") // Return an empty string as the result
        }
        "hextostring" => {
            //vmap.stackpush(param1, param2);
            return hex_to_string(param1);
        }
        "stringtohex" => {
            //vmap.stackpush(param1, param2);
            return string_to_hex(param1);
        }
        "stackpop" => {
            return vmap.stackpop(param1);
        }
        "delobj"  | "objdel" => {
            // execute deconstruct function (if is has it)
            let isdeconfn = "_".to_owned() + &param1 + ".deconstruct()"; // should only execute if it exists.. else continue
            let deconstructfunc = vmap.getcode(&isdeconfn);
            nscript_parsesheet(&deconstructfunc, vmap);
            vmap.delobj(param1);
            return String::new();
        }
        "objparents" => {
            return vmap.objparents(param1);
        }
        "objchildren" => {
            return vmap.objchildren(param1);
        }
        "setobjprop" => {
            vmap.setprop(param1, param2, param3);
            return String::new();
        }
        "getobjprop" => {
            let get = vmap.getprop(param1, param2);
            return get;
        }
        "setobj" => {
            vmap.setobj(param1, param2);
            return String::new();
        }
        "inobj" => {
            return Nstring::replace(&vmap.inobj(param1),"|",NC_ARRAY_DELIM);
        }

        "delobjprop" => {
            vmap.delprop(param1, param2);
            return String::new();
        }
        "stringtoeval" => {
            return Nstring::stringtoeval(param1);
        }
        "isfunction" => {
            let testc = vmap.getcode(param1);
            //println!("isfunction:{}",testc);
            return nscript_parsesheet(&testc, vmap);
        }
        "exec" => {
            nscript_execute_script(
                param1, param2, param3, param4, param5, param6, param7, param8, param9, "", vmap,
            );
            return "ok".to_owned();
        }
        "sheet" => {
            return nscript_parsesheet(&Nfile::read(param1), vmap);
        }
        "cin" => {
            return param1.to_string();
        }
        "cwrite" | "print" => {
            cwrite(param1, param2);
            return String::new();
        }
        "timerinit" => {
            return Timer::init().to_string();
        }
        "timerdiff" => {
            return Timer::diff(param1.parse::<i64>().unwrap()).to_string();
        }
        "fread" | "fileread" => {
            return Nfile::read(param1);
        }
        "fwrite" | "filewrite" => {
            Nfile::write(param1, param2);
            return String::new();
        }
        "splitselect" => {

            return splitselect(&param1,&param2,parse_string_to_usize(&param2))
        }
        "file_read_utf8" =>{
            return read_file_utf8(&param1).to_owned();
        }
        "fexists" | "fileexists"=> {
            if Nfile::checkexists(param1) == true {
                return String::from("1");
            } else {
                return String::from("0");
            }
        }
        "listdir" | "dirtolist" | "dirlist" => {
            if param2 == "" {
                return Nfile::dirtolist(param1, false);
            } else {
                return Nfile::dirtolist(param1, true);
            }
        }
        "split" => {
            return Nstring::split(param1, param2);
        }
        "instring" => {
            if Nstring::instring(param1, param2) == true {
                return String::from("1");
            } else {
                return String::from("0");
            }
        }
        "replace" => {
            //println!("replace a{} b{} c{}",&param1,&param2,&param3);
//            let ret = param1.to_owned().replace(param2,param3);
            let ret = Nstring::replace(param1,param2,param3);
            //println!("replaced: [{}]",&ret);

            return ret;            //return Nstring::replace(param1, param2, param3);
        }
        "trimleft" => {
            return Nstring::trimleft(param1, param2.parse::<usize>().unwrap());
        }
        "trimright" => {
            return Nstring::trimright(param1, param2.parse::<usize>().unwrap());
        }

        "fromleft" => {
            return Nstring::fromleft(param1, param2.parse::<usize>().unwrap());
        }
        "fromright" => {
            return Nstring::fromright(param1, param2.parse::<usize>().unwrap());
        }
        "save" => {
            Njh::write(param1, param2, param3);
            return String::new();
        }
        "load" => {
            return Njh::read(param1, param2);
        }
        "setvar" => {
            vmap.setvar(param1.to_string(), param2);
            return String::new();
        }
        "getvar" => {
            return vmap.getvar(param1);
        }

        "exit" => {
            return String::from("exit");
        }
        "arrayfilter" => {
            return arrayfilter(param1,param2);
        }
        "arraysearch" => {
            return arraysearch(param1,param2);
        }
        "arrayshuffle" => {
            return arrayshuffle(param1);
        }
        "" => {
            return String::new();
            //required?!
        }
        "decode_html_url" => {
       return decode_html_url(&param1).to_string();
        }
        "html_encode" => {
            return html_encode(&param1);
        }
        "stringbetween" => return Nstring::stringbetween(param1, param2, param3),
        "combine" | "cmb"=> {
            // combines a string, honestly i just added this so the compiler wont bitch over unused param vars lol..
            let nothing = param1.to_owned()
                + param2
                + param3
                + param4
                + param5
                + param6
                + param7
                + param8
                + param9;
            return nothing;
        }
        _ => {
            let error = "".to_owned() + "A non declared function call is done:" + &func;
            nscript_interpreterdebug(&error,vmap.debugmode,vmap.strictness);
            return String::new();
            // debug broken/non existing call
        }
    };
}

fn cwrite(m: &str, color: &str) {
    // this is more a linux then a windows feature.
    // as for windows powershell is just lame. itl work but dont expect all colors to show!
    // --------------------------------------------
    match color {
        "cyan" => {
            println!("{}", m.cyan());
        }
        "yellow" => {
            println!("{}", m.yellow());
        }
        "red" => {
            println!("{}", m.red());
        }
        "green" => {
            println!("{}", m.green());
        }
        "blue" => {
            println!("{}", m.blue());
        }
        _ => {
            println!("{}", m);
        }
    };
}


fn nscript_func(func: &str, vmap: &mut Varmap) -> String {
    // this is the function wich executes a nscript user made function !
    // -----------------------------------------------------------------
   let (args, id) = nscript_getarguments(&func, vmap); // get all argument params
    let func = func.trim();
    for r in 1..id {
        //let paramx = &r + 1

        let pname = "".to_owned() + &vmap.codelevel.to_string() + "__internalparam" + &r.to_string();
       vmap.setvar(pname, &args[r]); // set all param arguments
    }
    let mut fname = String::from(&args[0]);

    if Nstring::fromleft(&args[0], 1) == "_".to_owned() {
        fname = Nstring::trimleft(&args[0], 1); // strip away the _ prefix
    }


    let mut iscodebblock: String; //= vmap.getcode(&fname); // load code

    // set self and classfunction registers
    let mut isclass = String::new();
    let mut usedself = "__functioninternal";// set to make sure the while extract will parse this
    // block only
    if Nstring::instring(&func, ".") == true {
        let splitfn = split(&func, ".");
        if Nstring::fromleft(&splitfn[0], 1) == "_".to_owned() {//<-- this kinda is also from the
            //old syntax old nc begin with _ for udf.

            isclass = Nstring::trimleft(&splitfn[0].trim(), 1);

        } else {
            isclass = splitfn[0].trim().to_string().clone();
        }
       let cleanfnname = split(&splitfn[1], "(");
        let mut fnname = String::from(cleanfnname[0].trim());
        let mut reg = "nscript_classfuncs__".to_owned()  + &isclass +"."+ &fnname;

        //fnname = vmap.checkvar(&fnname);
        if Nstring::fromleft(&fnname,1) == "&" {
            fnname = nscript_checkvar(&Nstring::replace(&fnname,"&",""), vmap);
            reg = "nscript_classfuncs__".to_owned()  + &isclass +"."+ &fnname;

            //println!("going to check for fn:{}",&fnname);
        }
         if Nstring::fromleft(&isclass,1) == "&" {
            isclass = nscript_checkvar(&Nstring::replace(&isclass,"&",""), vmap);
            reg = "nscript_classfuncs__".to_owned()  + &isclass +"."+ &fnname;

            //println!("going to check for class:{}",&isclass);
        }



        let rootfnobj = vmap.getvar(&reg); // get root obj where the func is located.

        let rootfnfullname = "".to_owned() + &rootfnobj + "__" + &fnname;//+ &rootfnobj + "__" + &fnname;
        iscodebblock = vmap.getcode(&rootfnfullname); // load code

        vmap.stackpush("___self", &isclass);
        vmap.setvar("self".to_owned(), &isclass);
        //println!("setting self to:[{}]",&isclass);
        usedself = &isclass;
        iscodebblock = Nstring::replace(&iscodebblock, "self.", "&self."); // change all to the obj itself.
   } else {
        iscodebblock = vmap.getcode(&fname); // load code
   }
   let internalcoderef = vmap.getprop("__interpreter","parsingsubsheet");
       let get = nscript_parsesheet(&nscript_stringextract(&iscodebblock), vmap); // run code
        let isclass = vmap.stackpop("___self");
        vmap.setvar("self".to_owned(), &isclass);

    //println!("setting self to:[{}]",&isclass);
    get
}

fn nscript_execute_script(
    file: &str,
    param1: &str,
    param2: &str,
    param3: &str,
    param4: &str,
    param5: &str,
    param6: &str,
    param7: &str,
    param8: &str,
    param9: &str,
    vmap: &mut Varmap,
) -> String {

    //---------------------------------------------------------------------------------------
    //This is where you begin to load a .nc sheet this will exclude and load all classes and
    //functions aswell1, unlike parseshee() this is actually the full deal1, allias in nscript
    //syntax : exec(filename)
    //----------------------------------------------------------------------------------------

    vmap.parsinglevel = vmap.parsinglevel + 1;
    let thisparsingsheet = "_".to_owned() + &vmap.parsinglevel.to_string() +"__interpretercode";

    let thisparsingsubsheet = "_".to_owned() + &vmap.parsinglevel.to_string() +"__interpretersubcode";
    vmap.setprop("__interpreter","parsingsheet",&thisparsingsheet);
    vmap.setprop("__interpreter","parsingsubsheet",&thisparsingsubsheet);
    let  mut code = read_file_utf8(&file);
    vmap.setcode(&thisparsingsheet,&code);
    nscript_class_scopeextract(vmap);
    let code = vmap.getcode(&thisparsingsheet);
    nscript_func_scopeextract("", vmap);
    let code = vmap.getcode(&thisparsingsheet);
    vmap.parsinglevel = vmap.parsinglevel - 1;
    let thisparsingsheet = "_".to_owned() + &vmap.parsinglevel.to_string() +"__interpretercode";
    let thisparsingsubsheet = "_".to_owned() + &vmap.parsinglevel.to_string() +"__interpretersubcode";
    let ret = nscript_parsesheet(&code, vmap);
    vmap.setprop("__interpreter","parsingsheet",&thisparsingsheet);
    vmap.setprop("__interpreter","parsingsubsheet",&thisparsingsubsheet);
    // println!("newsheet:{}",code);
    ret
}

fn nscript_packscope(code: &str,scopeid: &str) -> String {
    // this is a internally used function to encapsulate scopes.
    // the scopes are being packed back into 1 line so its faster and easier to interpretate.
    // scopeid is a unique number wich is used to pack the scope, this way each scope can be
    // unpacked sepperatly.
    // --------------------------------------------------------------------------------------
    let newid = "%".to_owned() + scopeid + "%";
        let mut ifcodenew = Nstring::replace(&code, " ", "%id%sp%");
        ifcodenew = Nstring::replace(&ifcodenew, LINE_ENDING, "%id%lf%");
        ifcodenew = Nstring::replace(&ifcodenew, "(", "%id%bo%");
        ifcodenew = Nstring::replace(&ifcodenew, ")", "%id%bc%");
        ifcodenew = Nstring::replace(&ifcodenew, "{", "%id%cbo%");
        ifcodenew = Nstring::replace(&ifcodenew, "}", "%id%cbc%");

        ifcodenew = Nstring::replace(&ifcodenew, ",", "%id%c%");

    let ret = " scope(".to_owned() + &scopeid + "," + &Nstring::replace(&ifcodenew, "%id%", &newid) + ")";
 ret
}

fn nscript_unpackscope(code: &str,scopeid: &str,vmap: &mut Varmap) -> String {
    // this unpacks a packed scope and runs it.
    // related: nscript_unpackscopereturnclean()
    // ---------------------------------------
    let newid = "%".to_owned() + scopeid + "%";
    let mut ifcodenew = Nstring::replace(&code,&newid,  "%id%");
    ifcodenew = Nstring::replace(&ifcodenew, "%id%sp%", " ");
    ifcodenew = Nstring::replace(&ifcodenew, "%id%lf%", CODE_LINE_ENDING);
    ifcodenew = Nstring::replace(&ifcodenew,  "%id%bo%","(");
    ifcodenew = Nstring::replace(&ifcodenew,  "%id%bc%",")");
    ifcodenew = Nstring::replace(&ifcodenew, "%id%c%",",");
    ifcodenew = Nstring::replace(&ifcodenew, "%id%cbo%","{");
    ifcodenew = Nstring::replace(&ifcodenew, "%id%cbc%","}");
    let res = nscript_parsecompiledsheet(&nscript_compilesheet(&ifcodenew),vmap);
    if res ==".." {
        res
    }
    else {
        "RET=>".to_owned() + &res
    }

}
fn nscript_unpackscopereturnclean(code: &str,scopeid: &str,vmap: &mut Varmap) -> String {
    // this unpacks a scope and returns the scope's code instead.
    // --------------------------------------------------------
    let newid = "%".to_owned() + &scopeid + "%";
    let mut ifcodenew = Nstring::replace(&code,&newid,  "%id%");
    let pref = "scope(".to_owned() + &scopeid + ",";
    ifcodenew = Nstring::replace(&ifcodenew,&pref,  "");
    ifcodenew = Nstring::replace(&ifcodenew, "%id%sp%", " ");
    ifcodenew = Nstring::replace(&ifcodenew, "%id%lf%", CODE_LINE_ENDING);
    ifcodenew = Nstring::replace(&ifcodenew,  "%id%bo%","(");
    ifcodenew = Nstring::replace(&ifcodenew,  "%id%bc%",")");
    ifcodenew = Nstring::replace(&ifcodenew, "%id%c%",",");
    //println!("BlockUnpack:{}",&ifcodenew);
    ifcodenew

}

fn nscript_scopeextract(text: &str) -> String {
    // internal used in : class_scopeextract() func_scopeextr..
    // this is used to extract scopes like classes, functions.
    // it takes the full sheet of code, extract the scopes so that the internal interpreter doesnt
    // run them after the cleaning is done.
    // ------------------------------------------------------------------------------------
    let mut parsingtext = text.to_string();
    let mut toreturn = String::new();
    loop {
        let splitstr = split(&parsingtext,"{");

        if splitstr.len() > 1 {
            let isscope = split(&splitstr[splitstr.len()-1],"}")[0];
            let scopeid = "s".to_owned() + &splitstr.len().to_string();
            let packed = nscript_packscope(&isscope, &scopeid);
            let toreplace = "{".to_owned() + &isscope+ "}";
            parsingtext = Nstring::replace(&parsingtext, &toreplace, &packed)
        }
        else {
             toreturn = split(&splitstr[0],"}")[0].to_string();
            break;
        }
    }
    toreturn
}

fn nscript_stringextract(text: &str) -> String {
    // this will convert all static strings to a ^hexnumber
    // ive used this so that the spaces woulnd interfere with the syntax.
    // nscript_checkvar() will regonise ^3131 formats and unhex them where needed.
    // ------------------------------------------------------------------------
    let mut parsingtext = Nstring::replace(&text.to_string(),"\\\"","#!@NSCRIPTQUOTE#@!");
parsingtext = Nstring::replace(&parsingtext,"\"\"","@emptystring");

    loop {
        let splitstr = Nstring::stringbetween(&parsingtext,"\"","\"");
        if splitstr != "" {
            let packed = "^".to_owned() + &string_to_hex(&Nstring::replace(&splitstr,"#!@NSCRIPTQUOTE#@!","\""))  ;
            let toreplace = "\"".to_owned() + &splitstr+ "\"";
            parsingtext = Nstring::replace(&parsingtext, &toreplace, &packed);
        }
        else {
           break;
        }
    }
    parsingtext
}


fn nscript_class_scopeextract(vmap: &mut Varmap){
    // this function will at the beginning of executing a script extract and load
    // all class scopes, all functions inside these scopes will be linked giving access to self var
    // usage.
    //  - special: function .construct() will be triggered if a class spawns of a class wich has
    //  this declared. func .construct() on obj a : b   and on delobj(a) .destruct() will be
    //  triggered.
    // -----------------------------------------------------------------------
    //let mut parsingtext = text.to_string();
    let mut toreturn = String::new();
    let parsecode = vmap.getprop("__interpreter","parsingsheet");
    let parsesubcode = vmap.getprop("__interpreter","parsingsubsheet");
    let code = vmap.getcode(&parsecode);
    let mut i = 0; //<-- serves to filter first split wich isnt if found but default.
    let classes = split(&code,"class");
    for eachclass in classes {
        if i > 0 {
            let code = vmap.getcode(&parsecode);
            if eachclass != "" {
                let classnamepart = split(&eachclass, "{")[0];
                let classname = split(&classnamepart,":");
                if classname.len() > 1 {
                    vmap.setvar(classname[1].to_string().clone(), &classname[1]); // assign classname = classname
                    vmap.setobj(&classname[1], &classname[0]);
                }
                let block = extract_scope(&eachclass);// extract the class scope between { }
                vmap.setcode(&parsesubcode,&block);// assign the subscope

                nscript_func_scopeextract(classname[0],vmap);// extract functions from class scope
                let blocknew = vmap.getcode(&parsesubcode); // remaining when functions are removed
                //println!("Subblock::{}",&blocknew);
                vmap.setvar("self".to_owned(), &classname[0].trim());// assigning self var self.
                //println!("running class extraction assigning self:{}",&classname[0]);
                let blocknew = Nstring::replace(&blocknew, "self.", "&self.");// Reflect self!!!
                nscript_parsesheet(&blocknew, vmap);// run the remaining as classblock.
                //println!("Blc:{}",&blocknew);
                let toreplace = "class".to_owned() + &classnamepart +  &block ;
                if Nstring::instring(&toreplace, "{") && Nstring::instring(&toreplace, "}")  {
                    //println!("The replacement: {}",&toreplace);
                    vmap.setcode(&parsecode,&Nstring::replace(&code, &toreplace,"" ));
                    //println!("FoundClass:{}",&classname[0]);

                    //println!("classcode:{}",&vmap.getcode("___interpretercode"));

                }
            }
        }
        i +=1;
    }
    //code
}

fn nscript_func_scopeextract(selfvar: &str,vmap: &mut Varmap) {
    // this will extraxt all the function scopes from a code sheet.
    //
    // -------------------------------------------------------
    //let mut parsingtext = text.to_string();
    let parsecode = vmap.getprop("__interpreter","parsingsheet");
    let parsesubcode = vmap.getprop("__interpreter","parsingsubsheet");
    let mut internalcoderef = &parsecode; // <- used on normal functions
    if selfvar != "" {

         internalcoderef = &parsesubcode; //<-  to run class block after func remov
    }


    let code = vmap.getcode(&internalcoderef);
    let mut toreturn = String::new();
    let classnamefixed = String::new();

    let classes = split(&code,"func ");
    // if classes.len() < 2 {
    //     return;
    // }
 //let argumentsraw = split(&classes[0],"(")[1];
 let mut i = 0;
    for eachclass in classes {
        if i > 0 {
            let code = vmap.getcode(&internalcoderef);
            if eachclass.trim() != "" && Nstring::fromleft(&eachclass.trim(),1) != "{" {
                //println!("class:[{}]",eachclass);

                let firstline = split(&eachclass,"{")[0];
                let classname = split(&firstline, "(")[0];
                let classnamefixed = &classname.to_owned().clone();
                let mut block = extract_scope(&eachclass);
                let cleanblock = block.clone();
                let argumentsraw = split(&firstline, "(");


                if argumentsraw.len() > 1 {
                    let argumentsraw  = split(&argumentsraw[1], ")");
                    let splitarguments = split(&argumentsraw[0],",");
                    if splitarguments.len() > 1 {
                        let mut i = 0;
                        for thisargument in splitarguments {
                            if thisargument != "" {

                                //println!("this-argument:{}[{}]",&i,&thisargument);
                                i += 1; // bellow  we replace the given argument/parameters back
                                // towars internalparamx
                                let param = "\n".to_owned() + "internalparam" + &i.to_string() +  " ";
                                let torep = "\n".to_owned() + &thisargument +" ";
                                block = Nstring::replace(&block,&torep, &param);
                                let param = "(".to_owned() + "internalparam" + &i.to_string() + "";
                                let torep = "(".to_owned() + &thisargument + "";
                                block = Nstring::replace(&block,&torep, &param);
                                let param = ",".to_owned() + "internalparam" + &i.to_string() + "";
                                let torep = ",".to_owned() + &thisargument + "";
                                block = Nstring::replace(&block,&torep, &param);
                                //
                                let param = " ".to_owned() + " internalparam" + &i.to_string() + "";
                                let torep = " ".to_owned() + &thisargument + "";
                                block = Nstring::replace(&block,&torep, &param);

                            }

                        }
                    }
                    else{
                        if splitarguments[0] != "" {

                                let param = "\n".to_owned() + "internalparam1" + " ";
                                let torep = "\n".to_owned() + &splitarguments[0] + " ";
                                block = Nstring::replace(&block,&torep, &param);
                                let param = "(".to_owned() + "internalparam1";
                                let torep = "(".to_owned() + &splitarguments[0];
                                block = Nstring::replace(&block,&torep, &param);
                                let param = ",".to_owned() + "internalparam1" ;
                                let torep = ",".to_owned() + &splitarguments[0];
                                block = Nstring::replace(&block,&torep, &param);
                                //
                                let param = " ".to_owned() + " internalparam1";
                                let torep = " ".to_owned() + &splitarguments[0] ;
                                block = Nstring::replace(&block,&torep, &param);

                        }
                    }
                }
                if selfvar != "" {
                    // used to parse functions inside classcopes
                    let classnamefixed = "".to_owned() + &selfvar.trim() + "__" + &classname.trim();
                    let functionregobj = "nscript_classfuncs__".to_owned() + &selfvar.trim() + "." + &classname; //+ "." + &funcname; // internal obj
                    vmap.setvar(functionregobj.clone(), &selfvar.trim());
                    let block = Nstring::trimleft(&block, 1);
                    let block = Nstring::trimright(&block, 1);
                    // let block = trim_lines(&block);
                     let block = nscript_stringextract(&block);
                     let block  = nscript_scopeextract(&block);
                    vmap.setcode(&classnamefixed, &nscript_compilesheet(&block));
                    //println!("Setting func: {} \n with block: \n{}",&functionregobj, &block);
                }
                else {
                    // if not inside a classscope
                    //println!("new:{}",&block);
                     let block = trim_lines(&block);
                     let block = nscript_stringextract(&block);
                    // let block  = nscript_scopeextract(&block);

                    vmap.setcode(&classname, &nscript_compilesheet(&block));
                }
                let toreplace = "func ".to_owned() + & split(&eachclass, "{")[0] +  &cleanblock ;

                // set the modified code

                if Nstring::instring(&toreplace, "{") && Nstring::instring(&toreplace, "}") {// extraxt
                    // the functions from the class/script to clean it out.
                    vmap.setcode(&internalcoderef,&Nstring::replace(&code.trim(), &toreplace.trim(),"" ));

                }
            }
        }
        i +=1;
}

}
fn nscript_compilesheet(code: &str) -> String{
    // this function preformats a sheet and these can be run with nscript_parsecompiledsheet()
    // used in : For / While / func
    // ------------------------------------------------------------------------------
    let mut newcode = String::new();
    let lines = code.split(LINE_ENDING);
    for line in lines {
        let fxline = split(&line,"//")[0];
        newcode = "".to_owned() + &newcode + &fxline + LINE_ENDING;
    }
    nscript_scopeextract(&nscript_stringextract(&trim_lines(&kill_bill(&newcode))))
}
fn nscript_loop_scopeextract(selfvar: &str,vmap: &mut Varmap) {
    let parsecode;
    if selfvar == "" {

         parsecode = vmap.getprop("__interpreter","parsingsheet");

    }
    else {
        parsecode = vmap.getprop("__interpreter","parsingsubsheet");
        //println!("Code=={}",&parsecode)

    }

    let code = vmap.getcode(&parsecode);
    let keyword = "".to_owned() + NC_ASYNC_LOOPS_KEY +" ";
    let classes = split(&code,&keyword);
    let mut i = 0; //<-- serves to filter first split wich isnt if found but default.
    if classes.len() > 1 {
        for eachclass in classes {
            if i > 0 {
                if eachclass != "" {
                    let mut classname = split(&eachclass, "{")[0].to_owned();
                    let block = extract_scope(&eachclass);

                    if Nstring::instring(&classname,"&") {
                        classname = nscript_checkvar(&Nstring::replace(&classname, "&", ""), vmap);
                    }

                    let toreplace = keyword.to_owned() + &classname +  &block ;
                    //parsingtext = Nstring::replace(&parsingtext, &toreplace,"" );
                    if selfvar != "" {
                        vmap.setvar("nscript_loops".to_owned() + "." + &classname.trim(), &Nstring::replace(&block,"self","&self"));

                    }
                    else{
                        vmap.setvar("nscript_loops".to_owned() + "." + &classname.trim(), &block);


                    }
                    vmap.setcode(&parsecode,&Nstring::replace(&code, &toreplace,"" ));
                    //println!("funccode:{}",&toreplace);
                }
            }
            i += 1;
        }
    }

}

fn extract_scope(text: &str) -> String {
    // a internal function to remove the scopes
    // -------------------------------
    let mut stack = Vec::new();
    let mut start = None;
    let mut end = None;
    let mut depth = 0;

    for (index, ch) in text.char_indices() {
        match ch {
            '{' => {
                if stack.is_empty() {
                    start = Some(index);
                }
                stack.push(ch);
                depth += 1;
            }
            '}' => {
                stack.pop();
                depth -= 1;
                if stack.is_empty() && depth == 0 {
                    end = Some(index + 1);
                    break;
                }
            }
            _ => {}
        }
    }

    match (start, end) {
        (Some(start), Some(end)) => text[start..end].to_string(),
        _ => String::new(),
    }
}

fn nscript_funcextract(text: &str,vmap: &mut Varmap) -> String {
    // this function will extract and run nested functions from inner to outer
    // it will return 1 function back with all the arguments as evaluated nscript syntaxis,
    // --> funca(funcb(func()),funcd())
    // ----------------------------------------------------------
    let mut resultstring =text.to_string();
    let mut packed = String::new();
    let mut subfunction = String::new();

    loop {
        // get the last find in the string using (
        let splitstr = split(&resultstring,"(");
        // make sure its inside the main function so bigger>2
        if splitstr.len() > 2 {
            //take that substring and split up to the first )
            let splitscope = split(&splitstr[splitstr.len()-1],")");
            if splitscope.len() > 0 {
                // important one, if a variable or string is infron it
                // messes up the syntax so we split using comma
                let splitargus = split(&splitstr[splitstr.len()-2],",");
                // here we set thisfnname to the last part of the comma split
                let thisfnnamefix = splitargus[splitargus.len() -1];// make sure the function
                // here we check if the function given is reflected if so we evaluate the value of
                // the var and executre the function of the data from that var as a string
                if Nstring::fromleft(&splitstr[splitstr.len()-2],1) == "&"{
                    subfunction = "".to_owned() + &nscript_checkvar(&Nstring::replace(&thisfnnamefix,"&",""), vmap) + "(" + &splitscope[0]  + ")";
                }
                else {
                    // if its a normal funcion we run it.
                    subfunction = "".to_owned() + &thisfnnamefix + "(" + &splitscope[0]  + ")";
                }
                // here we evaluate the none function types.
                packed = "^".to_owned() + &string_to_hex( &nscript_runfncall(&subfunction, vmap));
            }
            else{
                // this also evaluates variables macros strings etc
                subfunction = "".to_owned() + &splitscope[0]; //&splitstr[splitstr.len()-1];
                packed = "".to_owned() + &nscript_checkvar(&splitscope[0], vmap);
            }
            let mut reflect = false;
            if splitscope.len() > 0 {
                // so this replaces the evaluated values in the word's() when
                // its all done it will return 1 function to parseline() wich be used to set the
                // variable
                if Nstring::fromleft(&splitstr[splitstr.len()-2],1) == "&" {
                    subfunction = "".to_owned() + &splitstr[splitstr.len()-2] + "(" + &splitscope[0]  + ")";
                    resultstring = Nstring::replace(&resultstring, &subfunction, &packed);
                    reflect = true
                }
            }
            if reflect == false{
                // very important! this reforms the strings till its made back to 1 function with
                // all evaluated data types. when this is done theres no double (( )) insde the
                // code and this function will exit and return the 1-function to parse_line()
                resultstring = Nstring::replace(&resultstring, &subfunction, &packed);
            }
        }
        else {
            break;
        }
    }
    resultstring
}

fn nscript_funcextract_bk(text: &str,vmap: &mut Varmap) -> String {
    // this function will extract and run nested functions from inner to outer
    // it will return 1 function back with all the arguments as evaluated nscript syntaxis,
    // --> funca(funcb(func()),funcd())
    // ----------------------------------------------------------
    let mut resultstring =text.to_string();// contains the string, we reform it back to 1 function
    // with evaluted things.
    let mut packed = String::new();
    let mut subfunction = String::new();

    loop {
        println!("Resultstring:{}",&resultstring);
        let splitstr = split(&resultstring,"(");
        if splitstr.len() > 2 {
            let splitscope = split(&splitstr[splitstr.len()-1],")");
            if splitscope.len() > 0 {
                if Nstring::fromleft(&splitstr[splitstr.len()-2],1) == "&"{
                    subfunction = "".to_owned() + &nscript_checkvar(&Nstring::replace(&splitstr[&splitstr.len()-2],"&",""), vmap) + "(" + &splitscope[0]  + ")";
                }
                else {
                    subfunction = "".to_owned() + &splitstr[splitstr.len()-2] + "(" + &splitscope[0]  + ")";
                }
                packed = "^".to_owned() + &string_to_hex( &nscript_runfncall(&subfunction, vmap));
            }
            else{
                subfunction = "".to_owned() + &splitscope[0]; //&splitstr[splitstr.len()-1];
                packed = "".to_owned() + &nscript_checkvar(&splitscope[0], vmap);
            }
            //parsingtext = Nstring::replace(&parsingtext, &subfunction, &ending);
            let mut reflect = false;
            if splitscope.len() > 0 {
                if Nstring::fromleft(&splitstr[splitstr.len()-2],1) == "&" {
                    subfunction = "".to_owned() + &splitstr[splitstr.len()-2] + "(" + &splitscope[0]  + ")";
                    resultstring = Nstring::replace(&resultstring, &subfunction, &packed);
                    reflect = true
                }
            }
            if reflect == false{
                resultstring = Nstring::replace(&resultstring, &subfunction, &packed);
            }
        }
        else {
            break;
        }
    }
    //println!("ResultCode:{}",&resultstring);
    resultstring
}



fn nscript_setparams_handleconnections(args: &Vec<String>,vmap: &mut Varmap){
// this function sets parameters when jumping functions. used on htmlserver
// because of the code level these params are differently set then functions.
let id = args.len();
    if id > 0 {
        //println!("codelevle = {}",&vmap.codelevel);
        let codelevelabove = &vmap.codelevel +0;// <- yeah seems neccesary for vmap.
        for r in 0..id {
            //let paramx = &r + 1;
            let paramid = r + 1;
            let pname = "".to_owned() + &codelevelabove.to_string() + "__internalparam" + &paramid.to_string();
            vmap.setvar(pname, &args[r]); // set all param arguments
        }
    }

}

fn handle_connection(mut stream: TcpStream,  vmap: &mut Varmap) {
    // this is the webserver part it will take a GET request and handle it.
    // text files are on the main thread for other downloads it goes to a other thread
    // .nc files are being regonised and they will return their return results to the user browser.
    // --------------------------------------------------------------------------------------
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    //println!("http={}",&request);
    let domainname = Nstring::stringbetween(&request,"Host: ","\n");
    vmap.setvar("___domainname".to_owned(),&domainname);
    let request_parts: Vec<&str> = request.split(" ").collect();
    let pathparts = split(&request_parts[1][1..],"?");
    let mut url_args = Vec::new();
    //let mut newparams = Vec::new();
    if pathparts.len() > 1 {
        url_args = split(pathparts[1], "&");
    }

    let mut newparams: Vec<String> = Vec::new();

    for i in 1..10 {
       if url_args.len()  > i - 1 {
            newparams.push(decode_html_url(&url_args[i-1].to_owned()));
        }
        else {
            newparams.push(String::from(""));
        }
    }

    nscript_setparams_handleconnections(&newparams,vmap);
    let mut file_path = format!("{}{}", SERVER_ROOT, &pathparts[0]);


    let checkthis = SCRIPT_DIR.to_owned() + "domains/" + &split(&vmap.getvar("___domainname"),":")[0]+"/http.nc";
    if Nfile::checkexists(&checkthis){
        file_path = SCRIPT_DIR.to_owned() + "domains/"  + &split(&vmap.getvar("___domainname"),":")[0]+"/public/"+&pathparts[0];

    }
    //println!("filepath = [{}]",&file_path);

    if let Some(extension) = Path::new(&file_path).extension().and_then(|os_str| os_str.to_str().map(|s| s.to_owned())) {
        if ["jpg", "jpeg", "png", "gif"].contains(&extension.as_str()) {
            let file_path_clone = file_path.clone(); // clone file_path
            thread::spawn(move || {
                let mut file = File::open(&file_path_clone).unwrap();
                let mut contents = Vec::new();
                file.read_to_end(&mut contents).unwrap();
                let content_type = match extension.as_str() {
                    "jpg" | "jpeg" => "image/jpeg",
                    "png" => "image/png",
                    "gif" => "image/gif",
                    _ => "application/octet-stream"
                };
                let response = format!("HTTP/2.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n", content_type, contents.len());
                stream.write(response.as_bytes()).unwrap();
                stream.write(&contents).unwrap();
            });
            return;
        }
    }
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => {
            let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
            stream.write(response.as_bytes()).unwrap();
            //println!("is 404");
            return;
        }
    };
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
   let isnc = Nstring::instring(&file_path,".nc");
    let content_type = match Path::new(&file_path).extension().unwrap().to_str().unwrap() {
        "html" => "text/html",
        "css" => "text/css",
        "nc" => "text/html",
        "js" => "application/javascript",
        "txt" => "text/plain",
        _ => "application/octet-stream"
    };

    if isnc {

        let scriptcode = read_file_utf8(&file_path);
        //println!("scriptcode:{}",&scriptcode);
        //let ret = nscript_execute_script(&scriptcode,newparams[0],newparams[1],newparams[2],newparams[3],newparams[4],newparams[5],newparams[6],newparams[7],newparams[8],vmap);
        let compcode = nscript_compilesheet(&nscript_stringextract(&scriptcode));

        let ret = nscript_parsesheet(&compcode, vmap);
        nscript_clearparams_handleconnections(vmap);
        //cwrite(&ret,&"red");
        //
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n", content_type, &ret.len());
        stream.write(response.as_bytes()).unwrap();
        stream.write(&ret.as_bytes()).unwrap();
    }
    else {
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n", content_type, contents.len());
        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();

    }


}
#[cfg(windows)]
fn set_nonblocking<T: AsRawSocket>(socket: &T) -> Result<(), io::Error> {
    use std::os::windows::raw::c_ulong;
    use std::os::windows::io::FromRawSocket;

    let raw_socket = socket.as_raw_socket() as SOCKET;
    let mut nonblocking: c_ulong = 1;
    let result = unsafe { ioctlsocket(raw_socket, FIONBIO, &mut nonblocking) };
    if result != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(windows)]
fn ioctlsocket(socket: SOCKET, cmd: c_long, argp: *mut c_ulong) -> i32 {
    unsafe {
        ioctlsocket::ioctlsocket(socket, cmd, argp)
    }
}

#[cfg(not(windows))]
fn set_nonblocking(socket: &TcpListener) -> Result<(), io::Error> {
    // Linux-specific implementation
    socket.set_nonblocking(true)?;
    Ok(())
}
//----------------RegionNscript------------------/\--------------
fn main() -> std::io::Result<()>  {
//send_message_to_discord_api()
    let mut vmap = Varmap::new(); // global

    println!("Starting fn main() Nscript v2.00Wip");
    println!("____________________________________");

    let listener = TcpListener::bind(format!("{}:{}", SERVER_ADDRESS, SERVER_PORT)).unwrap();
    set_nonblocking(&listener)?;
    println!("Server started at http://{}:{}", SERVER_ADDRESS, SERVER_PORT);
    let serverscriptfilename = SCRIPT_DIR.to_owned() +"server.nc";

    let domaindir = SCRIPT_DIR.to_owned() +"domains/";
    let domdir = Nfile::dirtolist(&domaindir,false);
    let domaindirarr = split(&domdir,NC_ARRAY_DELIM);


    nscript_execute_script(&serverscriptfilename,"","","","","","","","","",&mut vmap);





    for domainscript in domaindirarr {
        if domainscript != ""{
            vmap.setvar("___domainname".to_owned(),&domainscript);
            let domainscript = SCRIPT_DIR.to_owned() + "domains/"+domainscript + "/http.nc";


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
                println!("Client connecting:{}",remote_ip.to_string());

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

fn decode_html_url(url: &str) -> String {
    let entities = [
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&apos;", "'"),
    ];

    let mut decoded = String::new();
    let mut iter = url.chars().peekable();

    while let Some(ch) = iter.next() {
        if ch == '%' {
            // Check if it's a valid percent-encoded sequence
            if let (Some(h1), Some(h2)) = (iter.next(), iter.next()) {
                if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                    decoded.push(byte as char);
                    continue;
                }
            }
        }

        decoded.push(ch);
    }

    for (entity, replacement) in &entities {
        decoded = decoded.replace(entity, replacement);
    }

    decoded
}
fn html_encode(s_txt: &str) -> String {
    let entities: [(u32, &str); 246] = [
        (34, "quot"), (38, "amp"), (39, "apos"), (60, "lt"), (62, "gt"), (160, "nbsp"), (161, "iexcl"),
        (162, "cent"), (163, "pound"), (164, "curren"), (165, "yen"), (166, "brvbar"), (167, "sect"), (168, "uml"),
        (169, "copy"), (170, "ordf"), (171, "laquo"), (172, "not"), (173, "shy"), (174, "reg"), (175, "macr"),
        (176, "deg"), (177, "plusmn"), (180, "acute"), (181, "micro"), (182, "para"), (183, "middot"), (184, "cedil"),
        (186, "ordm"), (187, "raquo"), (191, "iquest"), (192, "Agrave"), (193, "Aacute"), (194, "Acirc"), (195, "Atilde"),
        (196, "Auml"), (197, "Aring"), (198, "AElig"), (199, "Ccedil"), (200, "Egrave"), (201, "Eacute"), (202, "Ecirc"),
        (203, "Euml"), (204, "Igrave"), (205, "Iacute"), (206, "Icirc"), (207, "Iuml"), (208, "ETH"), (209, "Ntilde"),
        (210, "Ograve"), (211, "Oacute"), (212, "Ocirc"), (213, "Otilde"), (214, "Ouml"), (215, "times"), (216, "Oslash"),
        (217, "Ugrave"), (218, "Uacute"), (219, "Ucirc"), (220, "Uuml"), (221, "Yacute"), (222, "THORN"), (223, "szlig"),
        (224, "agrave"), (225, "aacute"), (226, "acirc"), (227, "atilde"), (228, "auml"), (229, "aring"), (230, "aelig"),
        (231, "ccedil"), (232, "egrave"), (233, "eacute"), (234, "ecirc"), (235, "euml"), (236, "igrave"), (237, "iacute"),
        (238, "icirc"), (239, "iuml"), (240, "eth"), (241, "ntilde"), (242, "ograve"), (243, "oacute"), (244, "ocirc"),
        (245, "otilde"), (246, "ouml"), (247, "divide"), (248, "oslash"), (249, "ugrave"), (250, "uacute"), (251, "ucirc"),
        (252, "uuml"), (253, "yacute"), (254, "thorn"), (255, "yuml"), (338, "OElig"), (339, "oelig"), (352, "Scaron"),
        (353, "scaron"), (376, "Yuml"), (402, "fnof"), (710, "circ"), (732, "tilde"), (913, "Alpha"), (914, "Beta"),
        (915, "Gamma"), (916, "Delta"), (917, "Epsilon"), (918, "Zeta"), (919, "Eta"), (920, "Theta"), (921, "Iota"),
        (922, "Kappa"), (923, "Lambda"), (924, "Mu"), (925, "Nu"), (926, "Xi"), (927, "Omicron"), (928, "Pi"), (929, "Rho"),
        (931, "Sigma"), (932, "Tau"), (933, "Upsilon"), (934, "Phi"), (935, "Chi"), (936, "Psi"), (937, "Omega"), (945, "alpha"),
        (946, "beta"), (947, "gamma"), (948, "delta"), (949, "epsilon"), (950, "zeta"), (951, "eta"), (952, "theta"), (953, "iota"),
        (954, "kappa"), (955, "lambda"), (956, "mu"), (957, "nu"), (958, "xi"), (959, "omicron"), (960, "pi"), (961, "rho"),
        (962, "sigmaf"), (963, "sigma"), (964, "tau"), (965, "upsilon"), (966, "phi"), (967, "chi"), (968, "psi"), (969, "omega"),
        (977, "thetasym"), (978, "upsih"), (982, "piv"), (8194, "ensp"), (8195, "emsp"), (8201, "thinsp"), (8204, "zwnj"),
        (8205, "zwj"), (8206, "lrm"), (8207, "rlm"), (8211, "ndash"), (8212, "mdash"), (8216, "lsquo"), (8217, "rsquo"),
        (8218, "sbquo"), (8220, "ldquo"), (8221, "rdquo"), (8222, "bdquo"), (8224, "dagger"), (8225, "Dagger"), (8226, "bull"),
        (8230, "hellip"), (8240, "permil"), (8242, "prime"), (8243, "Prime"), (8249, "lsaquo"), (8250, "rsaquo"), (8254, "oline"),
        (8260, "frasl"), (8364, "euro"), (8465, "image"), (8472, "weierp"), (8476, "real"), (8482, "trade"), (8501, "alefsym"),
        (8592, "larr"), (8593, "uarr"), (8594, "rarr"), (8595, "darr"), (8596, "harr"), (8629, "crarr"), (8656, "lArr"),
        (8657, "uArr"), (8658, "rArr"), (8659, "dArr"), (8660, "hArr"), (8704, "forall"), (8706, "part"), (8707, "exist"),
        (8709, "empty"), (8711, "nabla"), (8712, "isin"), (8713, "notin"), (8715, "ni"), (8719, "prod"), (8721, "sum"),
        (8722, "minus"), (8727, "lowast"), (8730, "radic"), (8733, "prop"), (8734, "infin"), (8736, "ang"), (8743, "and"),
        (8744, "or"), (8745, "cap"), (8746, "cup"), (8747, "int"), (8764, "sim"), (8773, "cong"), (8776, "asymp"), (8800, "ne"),
        (8801, "equiv"), (8804, "le"), (8805, "ge"), (8834, "sub"), (8835, "sup"), (8836, "nsub"), (8838, "sube"), (8839, "supe"),
        (8853, "oplus"), (8855, "otimes"), (8869, "perp"), (8901, "sdot"), (8968, "lceil"), (8969, "rceil"), (8970, "lfloor"),
        (8971, "rfloor"), (9001, "lang"), (9002, "rang"), (9674, "loz"), (9824, "spades"), (9827, "clubs"), (9829, "hearts"),
        (9830, "diams")
    ];

    let mut s_txt_encoded = String::new();
    for c in s_txt.chars() {
        let entity = entities.iter().find(|&&(code, _)| code == c as u32);
        if let Some((_, name)) = entity {
            s_txt_encoded.push_str(&format!("&{};", name));
        } else {
            s_txt_encoded.push(c);
        }
    }
    s_txt_encoded
}

fn trim_lines(input: &str) -> String {
    let trimmed_lines: Vec<String> = input
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    trimmed_lines.join("\n")
}



fn nscript_loops(vmap: &mut Varmap) {
    let activeloops = vmap.inobj("nscript_loops");

    if activeloops != "" {
//println!("running loop:[{}]",&activeloops);

        let subloops = split(&activeloops, "|");
        for x in subloops {
            let d = vmap.getprop("nscript_loops", &x);
            vmap.stackpush("___self", &x);
            vmap.setvar("self".to_owned(), &x);
            nscript_parsecompiledsheet(&d, vmap);
            vmap.stackpop("___self");
            vmap.setvar("self".to_owned(), &x);

        }
    }
}

fn split<'a>(s: &'a str, p: &str) -> Vec<&'a str> {
    let r: Vec<&str> = s.split(p).collect();
    //println!("{:?}", &r);
    return r;
}

struct Timer {

}

impl Timer {
    pub fn diff(timerhandle: i64) -> i64 {
        let getnow = Timer::init();
        let diff = getnow - timerhandle;
        return diff;
    }
    pub fn init() -> i64 {
        let time = chrono::Utc::now();
        let mut timerstring = String::from(&time.year().to_string());
        if &time.month() < &10 {
            timerstring = timerstring + "0" + &time.month().to_string();
        } else {
            timerstring = timerstring + &time.month().to_string();
        }
        // check day for 2 characters
        if &time.day() < &10 {
            timerstring = timerstring + "0" + &time.day().to_string();
        } else {
            timerstring = timerstring + &time.day().to_string();
        }
        // check hour for 2 characters
        if &time.hour() < &10 {
            timerstring = timerstring + "0" + &time.hour().to_string();
        } else {
            timerstring = timerstring + &time.hour().to_string();
        }
        // check minute for 2 characters
        if &time.minute() < &10 {
            timerstring = timerstring + "0" + &time.minute().to_string();
        } else {
            timerstring = timerstring + &time.minute().to_string();
        }
        // check second for 2 characters
        if &time.second() < &10 {
            timerstring = timerstring + "0" + &time.second().to_string();
        } else {
            timerstring = timerstring + &time.second().to_string();
        }
        // check milisecond for 3 characters
        if &time.timestamp_subsec_millis() < &100 {
            if &time.timestamp_subsec_millis() < &10 {
                timerstring = timerstring + "00" + &time.timestamp_subsec_millis().to_string();
            } else {
                timerstring = timerstring + "0" + &time.timestamp_subsec_millis().to_string();
            }
        } else {
            timerstring = timerstring + &time.timestamp_subsec_millis().to_string();
        }
        return timerstring.parse::<i64>().unwrap();
    }
    // pub fn stamp() -> String {
    //     let time = chrono::Utc::now();
    //     let formatstring = time.year().to_string()
    //         + &"-".to_owned()
    //         + &time.month().to_string()
    //         + &"-".to_owned()
    //         + &time.day().to_string()
    //         + &" /".to_owned()
    //         + &time.hour().to_string()
    //         + &":".to_owned()
    //         + &time.minute().to_string()
    //         + &":".to_owned()
    //         + &time.second().to_string()
    //         + &"(ms".to_owned()
    //         + &time.timestamp_subsec_millis().to_string()
    //         + &")".to_owned();
    //     return formatstring;
    // }
}

struct Nfile {
    // nscript filesystem
}

impl Nfile {
pub fn dirtolist(readpath: &str, fullpathnames: bool) -> String {
    let mut output = String::new();

    let paths = match fs::read_dir(readpath) {
        Ok(paths) => paths,
        Err(error) => {
            println!("<error>: Cannot read directory: {}", error);
            return String::new();
        }
    };

    for path in paths {
        match path {
            Ok(entry) => {
                let unwraped = entry.path().display().to_string();
                if !unwraped.is_empty() {
                    output.push_str(&unwraped);
                    output.push_str(NC_ARRAY_DELIM);
                }
            }
            Err(error) => {
                println!("<error>: Cannot access directory entry: {}", error);
                return String::new();
            }
        }
    }

    if !fullpathnames {
        output = output.replace(readpath, "");
    }

    if Nstring::fromright(&output, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&output, NC_ARRAY_DELIM.len());
    }

    output
}    // Nscript file stuff
    // pub fn dirtolist(readpath: &str, fullpathnames: bool) -> String {
    //     // error handling moet nog gefixed worden.. : als dir niet bestaat.
    //     let mut output = String::new();
    //     let paths = fs::read_dir(readpath).unwrap();
    //     for path in paths {
    //         match &path {
    //             Ok(_) => {
    //                 let unwraped = path.unwrap().path().display().to_string();
    //                 if &unwraped != ""{
    //                     output = output + &unwraped + NC_ARRAY_DELIM;
    //                 }
    //
    //             }
    //             Err(_) => {
    //                 println!("<error>:Cannot find or access directory fn dirtolist()");
    //                 return String::new();
    //             }
    //         }
    //     }
    //     if fullpathnames == false {
    //         output = output.replace(readpath, "");
    //     }
    //     if Nstring::fromright(&output, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
    //         return Nstring::trimright(&output, NC_ARRAY_DELIM.len());
    //     }
    //     return output;
    // }
    pub fn checkexists(fp: &str) -> bool {
        return std::path::Path::new(fp).exists();
    }
    pub fn write(path: &str, data: &str) -> String {
        let mut f = match File::create(path) {
            Ok(file) => file,
            Err(err) => return err.to_string(),
        };

        if let Err(err) = f.write_all(data.as_bytes()) {
            return err.to_string();
        }

        if let Err(err) = f.sync_all() {
            return err.to_string();
        }

        String::new()
    }
    pub fn read(floc: &str) -> String {
    let mut file = match File::open(floc) {
        Ok(file) => file,
        Err(_) => return String::new(), // Return empty string on error
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return String::new(); // Return empty string on error
    }

    kill_bill(&contents)
    }
    pub fn readraw(floc: &str) -> String {
        let contents = fs::read_to_string(floc);
        match &contents {
            Err(_e) => String::new(),
            Ok(t) => String::from(&read_to_string(&t)),
        }
    }
}

struct Nstring {
    // Nscript String stuff
}

impl Nstring {

    pub fn replace(s: &str, f: &str, r: &str) -> String {
        if f == "" || s == ""{
            //println!("debugger cannot replace none?{} by none?{} ",&s,&f);
            return s.to_string();
        }
        // i know slaat nergens op.. :P
        return s.replace(f, r);
    }
    pub fn split<'a>(s: &'a str, p: &str) -> String {
        // usses in callfn this is the nscript split function not the internally one.
        let r: Vec<&str> = s.split(p).collect();
        let mut result = String::new();
        for is in &r {
            result = result + is + NC_ARRAY_DELIM;
        }

        if Nstring::fromright(&result,NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
            return Nstring::trimright(&result,NC_ARRAY_DELIM.len())
        }
        return String::from(&result);
    }
    pub fn instring(s: &str, f: &str) -> bool {
        let mut r = false;
        match s.find(f) {
            Some(_) => r = true,
            None => r = false,
        }
        return r;
    }
    pub fn trimleft(s: &str, f: usize) -> String {
        let len = s.len();
        if f < len+1 {
            return String::from(&s[f..len]);
        }
        else {

            return s.to_string();
        }
        //return String::from(&s[f..len]);
    }
    pub fn trimright(s: &str, f: usize) -> String {
        let len = s.len();
        if s.len() > f {
            return String::from(&s[0..len - f]);
        }
        else {

            return s.to_string();
        }

    }
    pub fn fromleft(s: &str, f: usize) -> String {
        let len = s.len();
        if f < len {
            return String::from(&s[0..f]);
        } else {
            return String::new();
        }
    }
    pub fn fromright(s: &str, f: usize) -> String {
        let len = s.len();
        if f < len {
            return String::from(&s[len - f..len]);
        } else {
            return String::new();
        }
    }
    pub fn stringtoeval(s: &str) -> String {
        // saver for hashmap keys usages
        let mut r = s.replace("-", "_");
        let all = [
            "~", "!", "#", "%", "^", "&", "*", "(", ")", "\\", "{", "}", "[", "]", ".", ",", "?",
            "'", "$", "/",
        ];
        for c in all {
            r = r.replace(c, "_");
        }
        r
    }
pub fn stringbetween<'a>(str: &'a str, a: &str, b: &str) -> String {
    if let Some(start_pos) = str.find(a) {
        let rest = &str[start_pos + a.len()..];
        if let Some(end_pos) = rest.find(b) {
            let extracted = &rest[..end_pos];
            //return extracted.trim_start_matches(|c: char| c.is_whitespace()).trim_end_matches(|c: char| c.is_whitespace()).to_string();

                return extracted.to_string();
        }
    }
    "".to_owned()
}

}

fn sleep(milliseconds: u64) {
    let duration = Duration::from_millis(milliseconds);
    thread::sleep(duration);
}

fn read_to_string(filename: &str) -> String {//<<-- if IDE says its not used, its a LIE!:w

    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return String::new(), // Return empty string on error
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return String::new(); // Return empty string on error
    }

    contents
}
fn hex_to_string(hex_string: &str) -> String {
    match Vec::from_hex(hex_string) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => String::new(),
    }
}

fn string_to_hex(input: &str) -> String {
    let hex_chars: Vec<char> = "0123456789ABCDEF".chars().collect();
    let bytes = input.as_bytes();
    let mut hex_string = String::new();

    for byte in bytes {
        let high_nibble = (byte & 0xF0) >> 4;
        let low_nibble = byte & 0x0F;
        hex_string.push(hex_chars[high_nibble as usize]);
        hex_string.push(hex_chars[low_nibble as usize]);
    }

    hex_string
}

fn string_to_eval(string_: &str) -> String {
    let mut return_val = string_.to_string();

    let replacements = [
        ("#", ""), ("%", ""), ("-", "_"), (" ", "_"), (":", "_"), ("\\", "_"), ("/", "_"),
        (".", "_"), ("@", "_"), ("&", "_"), ("!", ""), ("'", ""), ("[", "_"), ("]", "_"),
        ("(", "_"), (",", "_"), ("^", "_"), (")", "_"), ("|", "_")
    ];

    for (search, replace) in replacements {
        return_val = return_val.replace(search, replace);
    }

    return return_val;
}

fn pooladd(pool: &str,entree: &str) -> String{
    // nscript arrays wich work with unique entrees,
    // adding some thats already there wont be added.
    let array = split(&pool,NC_ARRAY_DELIM);
    let mut newstring = String::new();
    let mut found = false ;
    for entr in array{
        if entr == entree {
            found = true;
        }
        newstring = newstring + &entr + NC_ARRAY_DELIM;
    }
    if found == false{
        newstring = newstring + &entree + NC_ARRAY_DELIM;
    }
    if Nstring::fromright(&newstring,NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        newstring = Nstring::trimright(&newstring, NC_ARRAY_DELIM.len());
    }

    newstring
}

fn poolremove(pool: &str,entree: &str)-> String{

    let array = split(&pool,NC_ARRAY_DELIM);
    let mut newstring = String::new();
    for entr in array{
        if entr != entree {
            newstring = newstring + &entr + NC_ARRAY_DELIM;
        }
    }
    if Nstring::fromright(&newstring,NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        newstring = Nstring::trimright(&newstring, NC_ARRAY_DELIM.len());
    }
    newstring
}

fn curl(url: &str) -> String {
    match get(url) {
        Ok(mut response) => {
            let mut content = String::new();
            if let Ok(_) = response.read_to_string(&mut content) {
                return content;
            }
        }
        Err(err) => eprintln!("Error: {:?}", err),
    }
    String::new()
}

fn create_directory(dir_path: &str) -> String {
    match fs::create_dir(dir_path) {
        Ok(_) => format!("Directory '{}' created successfully", dir_path),
        Err(err) => format!("Error creating directory: {:?}", err),
    }
}

fn kill_bill(string: &str) -> String {
    // function removes the poisoness \r\n crap from satansoft systems and makes the interpreter
    // clean to run at while being free of evil..
    Nstring::replace(&string,"\r\n","\n")
}

fn read_file_utf8(filename: &str) -> String {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return String::new(),
    };

    let mut contents = Vec::new();
    if let Err(_) = file.read_to_end(&mut contents) {
        return String::new();
    }

    let (decoded, _, _) = UTF_8.decode(&contents);
    decoded.into_owned()
}

fn parse_string_to_usize(input: &str) -> usize {
    match input.parse::<usize>() {
        Ok(parsed_number) => parsed_number,
        Err(_) => 0,
    }
}

fn splitselect(arrayvar: &str,delim: &str,entree: usize) -> String{
    let this = split(&arrayvar,&delim);
    if entree > this.len()-1 {
        String::new()
    }
    else{
        return this[entree].to_string()
    }
}

fn terminal_get_user_input(message: &str, default: &str) -> String {
    print!("{} default:[{}]: ", message, default);
    io::stdout().flush().unwrap(); // Flushes the output to ensure the message is displayed immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove trailing newline character
    input = input.trim_end().to_string();

    if input.is_empty() {
        default.to_string()
    } else {
        input
    }
}

fn round_number(number: &str, decimals: &str) -> String {
    match (number.parse::<f64>(), decimals.parse::<usize>()) {
        (Ok(parsed_number), Ok(parsed_decimals)) => {
            let rounded = parsed_number.round();
            let formatted = format!("{:.*}", parsed_decimals, rounded);
            formatted
        }
        _ => String::new(),
    }
}
fn filesizebytes(file: &str) -> String {
    // returns the full byte size of a file!
    let path = Path::new(file);
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return String::new(),
    };

    let realsize = metadata.len();

    realsize.to_string()
}

fn filesize(file: &str) -> String {
    // returns a fancy calculated string of the size rounded GB/MB/KB
    let path = Path::new(file);
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return String::new(),
    };

    let realsize = metadata.len();
    if realsize >= 1_000_000_000 {
        return format!("{:.2} GB", realsize as f64 / 1_000_000_000.0);
    }
    if realsize >= 1_000_000 {
        return format!("{:.2} MB", realsize as f64 / 1_000_000.0);
    }
    if realsize >= 1_000 {
        return format!("{:.2} KB", realsize as f64 / 1_000.0);
    }

    format!("{} B", realsize)
}

fn arraypush(array: &str,data: &str ) -> String {
    return "".to_owned() + &array + NC_ARRAY_DELIM + &data
}

fn arraypushroll(array: &str,data: &str ) -> String {
    let splitsel = split(&array,NC_ARRAY_DELIM)[0];
    let striplen = splitsel.len() + NC_ARRAY_DELIM.len();
    let newarr = "".to_owned() + &array + NC_ARRAY_DELIM + &data;
    return Nstring::trimleft(&newarr,striplen);
}

fn arrayfilter(array: &str,tofilter: &str) -> String {
    let mut ret = String::new();
    for entree in split(&array,&NC_ARRAY_DELIM) {
        if Nstring::instring(&entree, &tofilter) == false {
            ret = "".to_owned() + &ret + &entree+ NC_ARRAY_DELIM;
        }
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    }
    else{
        ret
    }
}

fn arraysort(array: &str) -> String {
    let mut strings = split(&array,&NC_ARRAY_DELIM);
    strings.sort();
    let mut ret = String::new();
    for each in strings {
        ret = ret + &each + &NC_ARRAY_DELIM;
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    } else {
        ret
    }
}

fn arraysearch(array: &str,tosearch: &str) -> String{
    println!("searching array:{} for {}",&array,&tosearch);
    let mut ret = String::new();
    for entree in split(&array,&NC_ARRAY_DELIM){
        if Nstring::instring(&entree, &tosearch){
            ret = "".to_owned() + &ret + &entree+ NC_ARRAY_DELIM;
        }
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    }
    else{
        ret
    }
}

fn arrayshuffle(arraystr:&str) -> String{
    let mut array = split(&arraystr,NC_ARRAY_DELIM);
    let mut rng = rand::thread_rng();
    array.shuffle(&mut rng);
    let mut ret = String::new();
    for entrees in array{
        ret = ret + &entrees + NC_ARRAY_DELIM;
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    } else {
        ret
    }

}

fn random_number_between(min: &str, max: &str, decimals: &str) -> String {
    let min_num = match min.parse::<f64>() {
        Ok(parsed_num) => parsed_num,
        Err(_) => return String::new(),
    };

    let max_num = match max.parse::<f64>() {
        Ok(parsed_num) => parsed_num,
        Err(_) => return String::new(),
    };

    if min_num > max_num {
        return String::new();
    }

    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(min_num..=max_num);

    if decimals.is_empty() {
        return random_num.to_string();
    }

    let rounded_num = match decimals.parse::<usize>() {
        Ok(num_decimals) => format!("{:.*}", num_decimals, random_num),
        Err(_) => return String::new(),
    };

    rounded_num
}

// Move a file from the source path to the destination path
fn filemove(source: &str, destination: &str) -> String {
    match fs::rename(source, destination) {
        Ok(_) => format!("File moved successfully"),
        Err(err) => format!("Error moving file: {}", err),
    }
}

// Copy a file from the source path to the destination path
fn filecopy(source: &str, destination: &str) -> String {
    match fs::copy(source, destination) {
        Ok(_) => format!("File copied successfully"),
        Err(err) => format!("Error copying file: {}", err),
    }
}

// Delete a file at the specified path
fn filedelete(file: &str) -> String {
    match fs::remove_file(file) {
        Ok(_) => format!("File deleted successfully"),
        Err(err) => format!("Error deleting file: {}", err),
    }
}

// Delete a directory and all its contents
fn directory_delete(directory: &str) -> String {
    match fs::remove_dir_all(directory) {
        Ok(_) => format!("Directory deleted successfully"),
        Err(err) => format!("Error deleting directory: {}", err),
    }
}

// Move a directory from the source path to the destination path
fn directory_move(source: &str, destination: &str) -> String {
    match fs::rename(source, destination) {
        Ok(_) => format!("Directory moved successfully"),
        Err(err) => format!("Error moving directory: {}", err),
    }
}

fn call_program(command: &str) -> String {
    let mut parts = command.split_whitespace();
    let program = parts.next().expect("No program provided");
    let args: Vec<_> = parts.collect();

    let output = Command::new(program)
        .args(&args)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("Program executed successfully.\nStdout: {}\nStderr: {}", stdout, stderr)
            } else {
                format!("Program execution failed with exit code: {:?}", output.status.code())
            }
        }
        Err(err) => {
            format!("Failed to execute program: {}", err)
        }
    }
}
// fn convert_array<T: std::fmt::Display>(array: &[T]) -> String {
//     let elements: Vec<String> = array.iter().map(|element| element.to_string()).collect();
//     format!("{{{}}}", elements.join(", "))
// }
//
// fn dll_open(path: &str) -> Result<*mut c_void, Box<dyn Error>> {
//     let path_cstring = CString::new(path)?;
//     let handle = unsafe { libc::dlopen(path_cstring.as_ptr(), libc::RTLD_NOW) };
//     if handle.is_null() {
//         Err(format!("Failed to open DLL: {:?}", path).into())
//     } else {
//         Ok(handle)
//     }
// }
//
// fn dll_close(handle: *mut c_void) {
//     unsafe {
//         libc::dlclose(handle);
//     }
// }
//
// fn dllcall(handle: *mut c_void, func_name: &str, args: &[(&str, &str)]) -> Result<String, Box<dyn Error>> {
//     let func_name_cstring = CString::new(func_name)?;
//     let func_ptr = unsafe { libc::dlsym(handle, func_name_cstring.as_ptr()) };
//     if func_ptr.is_null() {
//         return Err(format!("Failed to find function: {:?}", func_name).into());
//     }
//
//     let func: extern "C" fn(c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_int, c_int) -> *mut c_int =
//         unsafe { std::mem::transmute(func_ptr) };
//
//     let mut converted_args: [c_int; 10] = [0; 10];
//     for (i, &(arg_type, arg_value)) in args.iter().enumerate() {
//         let converted_arg = match arg_type {
//             "int" => convert_arg(arg_value)?,
//             _ => return Err(format!("Unsupported argument type: {:?}", arg_type).into()),
//         };
//         converted_args[i] = converted_arg;
//     }
//
//     let result_ptr = func(converted_args[0], converted_args[1], converted_args[2], converted_args[3], converted_args[4],
//                           converted_args[5], converted_args[6], converted_args[7], converted_args[8], converted_args[9]);
//
//     let result = unsafe { *result_ptr };
//     if result == 0 {
//         return Err("Function call failed".into());
//     }
//
//     let result_array: &[c_int] = unsafe { std::slice::from_raw_parts(result_ptr, result as usize + 1) };
//
//     if result_array.len() > 1 {
//         let array_string = convert_array(&result_array[1..]);
//         Ok(array_string)
//     } else {
//         Ok(result.to_string())
//     }
// }
