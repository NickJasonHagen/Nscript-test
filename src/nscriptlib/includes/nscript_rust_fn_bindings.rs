
use crate::*;


pub fn nscript_callfn(
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
        "rawget" => {
    match raw_http_get(param1,param2) {
        Ok(response) => return response ,
        Err(err) => return String::new(),
    }
        }
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
        // "sqlgetrow" => {
        //     return perform_sql_query_get_row(&param1,&param2,param3);
        // }
        // "decrypt" => {
        //     return encrypt_string(&param1,&param2);
        // }
        //
        // "encrypt" => {
        //     return decrypt_string(&param1,&param2);
        // }
        "minutes_in_ms" => {
            return Ntimer::minutes_in_ms(&param1);
        }
        "hours_in_ms" => {
            return Ntimer::hours_in_ms(&param1);
        }
        "days_in_ms" => {
            return Ntimer::days_in_ms(&param1);
        }
        "weeks_in_ms" => {
            return Ntimer::weeks_in_ms(&param1);
        }
        "months_in_ms" => {
            return Ntimer::months_in_ms(&param1);
        }
        "years_in_ms" => {
            return Ntimer::years_in_ms(&param1);
        }

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
        "memusage" => {
            return memoryusage();
        }
        "memstats" | "memorystatus" => {
            return memorystatus();
        }
        "objtojson" => {
            return objtojson(&param1,vmap);
        }
        "objfromjson" => {
            objfromjson(&param1,param2,vmap);
            return param1.trim().to_owned();
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
        "inpool" => {
            return inpool(&param1,&param2);
        }
        "pooladd" => {
            return pooladd(&param1,&param2);
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
            let isdeconfn = "".to_owned() + &param1 + ".deconstruct()"; // should only execute if it exists.. else continue
            nscript_func(&isdeconfn, vmap);
            vmap.delobj(param1);
            return String::new();
        }
        "objparents" => {
            return vmap.objparents(param1);
        }
        "objchildren" | "getobjchildren"=> {
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
            let isconfn = "_".to_owned() + &param1 + ".construct()"; // should only execute if it exists.. else continue
            nscript_func(&isconfn, vmap);
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
            return param1.to_owned();
        }
        "timerinit" => {
            return Ntimer::init().to_string();
        }
        "timerdiff" => {
            return Ntimer::diff(param1.parse::<i64>().unwrap()).to_string();
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
            if Nfile::checkexists(&param1) == true {
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


