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
pub fn split<'a>(s: &'a str, p: &str) -> Vec<&'a str> {
    let r: Vec<&str> = s.split(p).collect();
    //println!("{:?}", &r);
    return r;
}

pub fn cwrite(m: &str, color: &str) {
    // this is more a linux then a windows feature.
    // as for windows powershell is just lame. itl work but dont expect all colors to show!
    // --------------------------------------------
    match color {
        "bright blue" | "bb" => {
            println!("{}", m.bright_blue());
        }
        "bright green" | "bg"=> {
            println!("{}", m.bright_green());
        }
        "bright cyan" | "bc" => {
            println!("{}", m.bright_cyan());
        }
        "bright red" | "br" => {
            println!("{}", m.bright_red());
        }
        "bright magenta" | "bm" => {
            println!("{}", m.bright_magenta());
        }
        "bright yellow" | "by" => {
            println!("{}", m.bright_yellow());
        }
        "bright purple" | "bp" => {
            println!("{}", m.bright_purple());
        }
        "purple" | "p" => {
            println!("{}", m.purple());
        }
        "cyan" | "c" =>{
            println!("{}", m.cyan());
        }
        "yellow" | "y" => {
            println!("{}", m.yellow());
        }
        "red" | "r" => {
            println!("{}", m.red());
        }
        "green" | "g" => {
            println!("{}", m.green());
        }
        "blue" | "b" =>{
            println!("{}", m.blue());
        }
        "magenta" | "m" =>{
            println!("{}", m.magenta());
        }

        _ => {
            println!("{}", m);
        }
    };
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
