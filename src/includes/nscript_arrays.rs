use std::fs;
use std::fs::File;
use std::path::{Path, PrefixComponent};
const NC_ARRAY_DELIM : &str = "]].n.c.arr.[[";

 fn split<'a>(s: &'a str, p: &str) -> Vec<&'a str> {
    let r: Vec<&str> = s.split(p).collect();
    //println!("{:?}", &r);
    return r;
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
pub fn arraypush(array: &str,data: &str ) -> String {
    return "".to_owned() + &array + NC_ARRAY_DELIM + &data
}

pub fn arraypushroll(array: &str,data: &str ) -> String {
    let splitsel = split(&array,NC_ARRAY_DELIM)[0];
    let striplen = splitsel.len() + NC_ARRAY_DELIM.len();
    let newarr = "".to_owned() + &array + NC_ARRAY_DELIM + &data;
    return Nstring::trimleft(&newarr,striplen);
}

pub fn arrayfilter(array: &str,tofilter: &str) -> String{
    let mut ret = String::new();
    for entree in split(&array,&NC_ARRAY_DELIM){
        if Nstring::instring(&entree, &tofilter) == false{
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

pub fn arraysort(array: &str) -> String {
    let mut strings = split(&array,&NC_ARRAY_DELIM);
    strings.sort();
    let mut ret = String::new();
    for each in strings {
        ret = ret + &each + &NC_ARRAY_DELIM;
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    }
    else{
        ret
    }

}

pub fn arraysearch(array: &str,tosearch: &str) -> String{
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

pub fn arrayshuffle(arraystr:&str) -> String{
    let mut array = split(&arraystr,NC_ARRAY_DELIM);
    let mut rng = rand::thread_rng();
    array.shuffle(&mut rng);
    let mut ret = String::new();
    for entrees in array{
        ret = ret + &entrees + NC_ARRAY_DELIM;
    }
    if Nstring::fromright(&ret, NC_ARRAY_DELIM.len()) == NC_ARRAY_DELIM {
        return Nstring::trimright(&ret, NC_ARRAY_DELIM.len());
    }
    else{
        ret
    }

}
