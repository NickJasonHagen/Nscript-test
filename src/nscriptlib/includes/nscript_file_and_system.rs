use crate::*;
use sysinfo::{System, SystemExt};
use psutil::process::Process;
use std::process;
pub struct Nfile {
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
    }
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


pub fn create_directory(dir_path: &str) -> String {
    match fs::create_dir(dir_path) {
        Ok(_) => format!("Directory '{}' created successfully", dir_path),
        Err(err) => format!("Error creating directory: {:?}", err),
    }
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

pub fn memorystatus()->String{
        let mut system = System::new_all();

    // Refresh the system to get the latest information
    system.refresh_all();

    // Get system memory information
    let total_memory = system.get_total_memory();
    let free_memory = system.get_free_memory();
    let used_memory = total_memory - free_memory;

    // Print the memory information
    let toreturn = "Total: ".to_owned() + &total_memory.to_string() + " KB\nUsed:"
    + &used_memory.to_string() + "KB\nFree:" + &free_memory.to_string() + "KB";
    return toreturn;
}

pub fn memoryusage() -> String{
    if let Some(memory_usage) = get_process_memory_usage() {
        //println!("Process Memory Usage: {} bytes", memory_usage);
        return memory_usage.to_string();
    } else {
        //println!("Failed to get process memory usage.");
        return "error".to_string();
    }
}
pub fn get_own_pid() -> u32 {
    // Get the current process ID using std::process::id()
    process::id()
}

fn get_process_memory_usage() -> Option<u64> {
    // Get the current process ID
    let pid = get_own_pid();

    // Try to get the process by its ID
    if let Ok(process) = Process::new(pid) {
        // Get the memory information for the process
        if let Ok(mem_info) = process.memory_info() {
            // Return the RSS (Resident Set Size) in bytes
            return Some(mem_info.rss());
        }
    }

    // Return None if there was an error or the process is not found
    None
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
