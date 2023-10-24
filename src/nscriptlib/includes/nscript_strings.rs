
use crate::*;

pub struct Nstring {
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
    pub fn stringbetweenincludeempty<'a>(str: &'a str, a: &str, b: &str) -> String {
        // used for interal usage to extraxt scopes, if a scope is empty its still a scope.
        // iteratrs shoulnd exit then so this funtion retuns something else
        // to let the iterator know to continue instead of a empty string.
        // ---------------------------------------
        if let Some(start_pos) = str.find(a) {
        let rest = &str[start_pos + a.len()..];
        if let Some(end_pos) = rest.find(b) {
            let extracted = &rest[..end_pos];
            //return extracted.trim_start_matches(|c: char| c.is_whitespace()).trim_end_matches(|c: char| c.is_whitespace()).to_string();

                return extracted.to_string();
        }
    }
    "<nonefound!>".to_owned()
}
}


pub fn hex_to_string(hex_string: &str) -> String {
    match Vec::from_hex(hex_string) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => String::new(),
    }
}

pub fn string_to_hex(input: &str) -> String {
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
pub struct Njh {

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

pub fn string_to_eval(string_: &str) -> String {
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
