use base64::{encode, decode};
//use rusqlite::{Connection, Result};
//time
use chrono::{Datelike, Timelike};
use crate::NC_ARRAY_DELIM;
use crate::*;





// Function to convert a string to base64
pub fn string_to_base64(string_in: &str) -> String {
    encode(string_in)
}

// Function to convert base64 to a string
pub fn base64_to_string(string_in: &str) -> String {
    match decode(string_in) {
        Ok(decoded) => String::from_utf8_lossy(&decoded).to_string(),
        Err(_) => String::new(),
    }
}


pub fn read_file_utf8(filename: &str) -> String {
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

pub fn parse_string_to_usize(input: &str) -> usize {
    match input.parse::<usize>() {
        Ok(parsed_number) => parsed_number,
        Err(_) => 0,
    }
}

pub fn splitselect(arrayvar: &str,delim: &str,entree: usize) -> String{
    let this = split(&arrayvar,&delim);
    if entree > this.len()-1 {
        String::new()
    }
    else{
        return this[entree].to_string()
    }
}

pub fn terminal_get_user_input(message: &str, default: &str) -> String {
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

pub fn round_number(number: &str, decimals: &str) -> String {
    match (number.parse::<f64>(), decimals.parse::<usize>()) {
        (Ok(parsed_number), Ok(parsed_decimals)) => {
            let rounded = parsed_number.round();
            let formatted = format!("{:.*}", parsed_decimals, rounded);
            formatted
        }
        _ => String::new(),
    }
}
pub fn filesizebytes(file: &str) -> String {
    // returns the full byte size of a file!
    let path = Path::new(file);
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return String::new(),
    };

    let realsize = metadata.len();

    realsize.to_string()
}

pub fn filesize(file: &str) -> String {
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

pub fn arraypush(array: &str,data: &str ) -> String {
    return "".to_owned() + &array + NC_ARRAY_DELIM + &data
}

pub fn arraypushroll(array: &str,data: &str ) -> String {
    let splitsel = split(&array,NC_ARRAY_DELIM)[0];
    let striplen = splitsel.len() + NC_ARRAY_DELIM.len();
    let newarr = "".to_owned() + &array + NC_ARRAY_DELIM + &data;
    return Nstring::trimleft(&newarr,striplen);
}

pub fn arrayfilter(array: &str,tofilter: &str) -> String {
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

pub fn arraysort(array: &str) -> String {
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

pub fn arraysearch(array: &str,tosearch: &str) -> String{
    //println!("searching array:{} for {}",&array,&tosearch);
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
    } else {
        ret
    }

}

pub fn random_number_between(min: &str, max: &str, decimals: &str) -> String {
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
pub fn filemove(source: &str, destination: &str) -> String {
    match fs::rename(source, destination) {
        Ok(_) => format!("File moved successfully"),
        Err(err) => format!("Error moving file: {}", err),
    }
}

// Copy a file from the source path to the destination path
pub fn filecopy(source: &str, destination: &str) -> String {
    match fs::copy(source, destination) {
        Ok(_) => format!("File copied successfully"),
        Err(err) => format!("Error copying file: {}", err),
    }
}

// Delete a file at the specified path
pub fn filedelete(file: &str) -> String {
    match fs::remove_file(file) {
        Ok(_) => format!("File deleted successfully"),
        Err(err) => format!("Error deleting file: {}", err),
    }
}

// Delete a directory and all its contents
pub fn directory_delete(directory: &str) -> String {
    match fs::remove_dir_all(directory) {
        Ok(_) => format!("Directory deleted successfully"),
        Err(err) => format!("Error deleting directory: {}", err),
    }
}

// Move a directory from the source path to the destination path
pub fn directory_move(source: &str, destination: &str) -> String {
    match fs::rename(source, destination) {
        Ok(_) => format!("Directory moved successfully"),
        Err(err) => format!("Error moving directory: {}", err),
    }
}

pub fn call_program(command: &str) -> String {
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

pub struct Timer {

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


pub fn hours_in_ms(time: &str) -> String {
   return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"3600000")).to_string() ;
}
pub fn minutes_in_ms(time: &str) -> String {
   return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"60000")).to_string() ;
}
pub fn days_in_ms(time: &str) -> String {
   return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"86400000")).to_string() ;
}
pub fn weeks_in_ms(time: &str) -> String {
   return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"604800000")).to_string() ;
}
pub fn months_in_ms(time: &str) -> String {
   return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"2629800000")).to_string() ;
}
pub fn years_in_ms(time: &str) -> String {
   return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"31557600000")).to_string() ;
}
// pub fn perform_sql_query_get_row(query: &str, getrow: &str,database: &str) -> String {
//     let conn = Connection::open(database).unwrap();
//     let mut stmt = conn.prepare(query).unwrap();
//     let rows = stmt.query_map([], |row| {
//         // Process each row of the result set
//         // Modify this closure to extract the desired values from the row
//         // For example, if the row contains a text column called 'name':
//         let name: String = row.get(getrow)?;
//         Ok(name)
//     }).unwrap();
//
//     let mut result = String::new();
//
//     for row in rows {
//         let name: String = row.unwrap().to_string();
//         result.push_str(&name);
//         result.push('\n');
//     }
//
//     result
// }
