

use crate::*;




pub fn curl(url: &str) -> String {
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

pub fn raw_http_get(url: &str,fname: &str) -> Result<String, Box<dyn std::error::Error>> {

    //format for TCP socket
    let spliturl =  split(&url,":");
    let host = &spliturl[0];
    let mut port = "80";
    if spliturl.len() > 1 {
        port = spliturl[1];
    }
    let addr = format!("{}:{}", &host, &port)
        .to_socket_addrs()?
        .next()
        .ok_or("Unable to resolve the hostname")?;

    // Connect to the server
    let mut stream = TcpStream::connect(addr)?;

    // create the GET header
    let msg = "GET /".to_owned() + &fname + " HTTP/1.1
Host:" + &format!("{}:{}", &host, &port) +"
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
Accept-Encoding: gzip, deflate
Accept-Language: en-US,en;q=0.9
Connection: keep-alive
";
    // send the Get request to the server, the socket will send meta back
    // close and the next stream be the dataloop.
    let byte_slice: &[u8] = msg.as_bytes();
    stream.write_all(&byte_slice)?;

    // Receive response from the server
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let rawres = String::from_utf8_lossy(&buffer[0..buffer.len()]);
    // get the size of what we going to receive.
    let bytesize = Nstring::stringbetween(&rawres,"Content-Length: ","\n");
    let buffer_size: usize = bytesize.parse().unwrap_or(1024);
    // create a new bytebuffer with the size of the contents
    let mut buffer = vec![0; buffer_size];
    let mut receivedbytes = 0; // used as counter++
    // this string be filled with the data read as bytes.
    let mut receivedstring = String::new();
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        receivedbytes += bytes_read;
        receivedstring = receivedstring + &String::from_utf8_lossy(&buffer[0..bytes_read]);
        if bytes_read == 0 {
            // this is a socket close / end of packet / error.
            break;
        }
    }

    //Nfile::write("./testget.txt",&receivedstring);
    Ok(receivedstring.to_string())
}


pub fn decode_html_url(url: &str) -> String {
    let entities = [
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&apos;", "'"),
    ];

    let mut decoded = String::new();
    let xurl = Nstring::replace(&url,"+"," ");
    let mut iter = xurl.chars().peekable();

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
pub fn html_encode(s_txt: &str) -> String {
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



pub fn nscript_setparams_handleconnections(args: &Vec<String>,vmap: &mut Varmap){
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


pub fn handle_connection(mut stream: TcpStream,  vmap: &mut Varmap) {
    // this is the webserver part it will take a GET request and handle it.
    // text files are on the main thread for other downloads it goes to a other thread
    // .nc files are being regonised and they will return their return results to the user browser.
    // --------------------------------------------------------------------------------------
    let mut buffer = [0; 1024];
    //stream.read(&mut buffer).unwrap();


    match stream.read(&mut buffer) {
        Ok(_) => {
            // procceed the connection.

        }
        Err(_) => {
            // handle OS error on connection-reset
            //println!("stream read error ! ");
            return;
        }
    }
    let request = String::from_utf8_lossy(&buffer[..]);
    if Nstring::instring(&request, "B blob data") {
        println!("(debug->returning) Blob data entering: {}",&request);
        return ; // prevent errors , return!
    }
    if Nstring::instring(&request, "POST") == false && Nstring::instring(&request, "GET") == false{
        println!("A non POST nor GET packet entered: \n {}",&request);
        return; // clearly we aint gonna handle this (yet)

    }
    //println!("req:{}",&request);
    //let request_clone = request.clone();
    let domainname = Nstring::replace(&Nstring::stringbetween(&request,"Host: ","\r\n"),"www.","");
    let domainname = split(&domainname,":")[0];
    vmap.setvar("___domainname".to_owned(),&domainname);
    let request_parts: Vec<&str> = request.split(" ").collect();
//if request_parts[0] != "GET" {return;} // debugger to find that damn crash on b blobdata.
    let mut pathparts = Vec::new();
    let trimmedreq: String;
    if request_parts.len() > 1 {
        if request_parts[1].contains("B blob data") {
            return ; // Ignore blob data and return without processing
        }
        trimmedreq = Nstring::trimleft(&request_parts[1],1);
        pathparts = split(&trimmedreq,"?");
    }
    else{

        pathparts.push("");
    }
    if pathparts[0] == ""{
        pathparts[0] = "index.nc";
    }


    let mut url_args = Vec::new();
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

    let mut file_path = Nstring::replace(&format!("{}{}", SERVER_ROOT, &pathparts[0]),"/..","");
        let checkthis = SCRIPT_DIR.to_owned() + "domains/" + &domainname + "/http.nc";
        if Nfile::checkexists(&checkthis){
            file_path = SCRIPT_DIR.to_owned() + "domains/"  + &domainname + "/public/"+ &pathparts[0];

    }
    if request_parts[0] == "POST" {
        let mut postdata = String::new();

        let strippostdata = split(&request,"\r\n\r\n");
        if strippostdata.len() > 1 {
            postdata = "".to_owned() +strippostdata[1] ;// used for post buffer data
            //println!("strippedpostdata:{}",&postdata);
        }
        else {
            return;//some jacked up post request being done.
        }


        if let Some(extension) = Path::new(&file_path).extension().and_then(|os_str| os_str.to_str().map(|s| s.to_owned())) {
            if ["nc"].contains(&extension.as_str()) {
                //println!("Its a Post to Nc");
                let response =  "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";
                match stream.write(response.as_bytes()) {
                    Ok(bytes_written) => {
                        // Check if all bytes were successfully written.
                        if bytes_written < response.len() {
                            // Handle the situation where not all data was written if needed.
                        }

                    }
                    Err(_) => {
                        //return;
                    }
                }
                let bsize = nscript_f64(&Nstring::stringbetween(&request,"Content-Length: ","Cache").trim());
                if bsize > nscript_f64(&nscript_checkvar("server.POSTbytesmax",vmap)){
                    let response = "SERVERERROR:PostSizeExceedsLimit";
                    match stream.write(response.as_bytes()) {
                    Ok(_) => {
                            return;
                    }
                    Err(_) => {
                        return;
                    }
                }
                                    }
                //println!("bytesize:{}",&bsize);
                if bsize > 454.0 {
                    loop{
                        match stream.read(&mut buffer) {
                            Ok(bytes_read) => {

                                //println!("\nbytesRead!{}\n",bytes_read);
                                postdata = postdata + &String::from_utf8_lossy(&buffer[..]);
                                if bytes_read == 0 || bytes_read < 1024 {break;}

                                // procceed the connection.

                            }
                            Err(_) => {
                                break;
                                // handle OS error on connection-reset

                            }
                        }
                    }
                }

                let url_args = split(&postdata, "&");
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
                let scriptcode = read_file_utf8(&file_path);
                let compcode = nscript_formatsheet(&nscript_stringextract(&scriptcode));
                let response = nscript_parsesheet(&nscript_replaceparams(&compcode,"param"), vmap);
                match stream.write(response.as_bytes()) {
                    Ok(bytes_written) => {
                        // Check if all bytes were successfully written.
                        if bytes_written < response.len() {
                            // Handle the situation where not all data was written if needed.
                        }

                    }
                    Err(_) => {
                        //return;
                    }
                }
                //println!("post: {}",postdata);

            }
            return;
        }
    }
    if request_parts[0] == "GET" {
        if let Some(extension) = Path::new(&file_path).extension().and_then(|os_str| os_str.to_str().map(|s| s.to_owned())) {
        if ["nc"].contains(&extension.as_str()) {
            let _ = match File::open(&file_path) {
                Ok(_) => {},
                Err(_) => {
                    let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                    stream.write(response.as_bytes()).unwrap();
                    return;
                }
            };
            let scriptcode = read_file_utf8(&file_path);
                let compcode = nscript_formatsheet(&nscript_stringextract(&scriptcode));
                let ret = nscript_parsesheet(&nscript_replaceparams(&compcode,"param"), vmap);// <-- enables param usage param1 param2 etc.
                nscript_clearparams_handleconnections(vmap);
                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n", "text/html", &ret.len());
                match stream.write(response.as_bytes()) {
                    Ok(bytes_written) => {
                        // Check if all bytes were successfully written.
                        if bytes_written < response.len() {
                            // Handle the situation where not all data was written if needed.
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
                match stream.write(ret.as_bytes()) {
                    Ok(bytes_written) => {
                        // Check if all bytes were successfully written.
                        if bytes_written < response.len() {
                            // Handle the situation where not all data was written if needed.
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
                return;

        }
        let file_path_clone = file_path.clone(); // clone file_path
        thread::spawn(move || {
            let mut file = match File::open(&file_path_clone) {
                Ok(file) => file,
                Err(_) => {
                    let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                    stream.write(response.as_bytes()).unwrap();
                    return;
                }
            };
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap();
            let content_type = match extension.as_str() {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "gif" => "image/gif",
                "js" => "application/javascript",
                "txt" => "text/plain",
                "html" => "text/html",
                "css" => "text/css",
                _ => "application/octet-stream"
            };
            let response = format!("HTTP/2.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n", content_type, contents.len());
match stream.write(response.as_bytes()) {
    Ok(bytes_written) => {
        // Check if all bytes were successfully written.
        if bytes_written < response.len() {
            eprintln!("Not all data was written to the stream.");
            // Handle the situation where not all data was written if needed.
        }
    }
    Err(error) => {
        return;
    }
}
            match stream.write(&contents) {
    Ok(bytes_written) => {
        // Check if all bytes were successfully written.
        if bytes_written < contents.len() {
            // Handle the situation where not all data was written if needed.
        }
    }
    Err(error) => {
        return;
    }
}
        });
        return;
    }
}


}

// fn handlepost(){// <------------------------------------ Need to be worked in handle connection
// still.
//
//      if request_parts[0] == "POST" {
//         let mut stream_clone = stream.try_clone().unwrap();
//         // Check if the file path has the ".nc" extension
//         if let Some(extension) = Path::new(&file_path).extension().and_then(|os_str| os_str.to_str().map(|s| s.to_owned())) {
//             if extension == "nc" {
//                 // Spawn a new thread to handle the file upload
//                 let request_clone = request.clone();
//                 let file_path_clone = file_path.clone();
//                 thread::spawn(move || {
//                     // Read the request body (file data) until the end
//                     let mut boundary = String::new();
//                     if Nstring::stringbetween(&request_clone, "boundary=", "\r\n") != "" {
//                         let boundary_str = &Nstring::stringbetween(&request_clone, "boundary=", "\r\n");
//                         boundary = boundary_str.to_owned();
//
//                     }
//
//                     // Process each part of the multipart form data
//                     let mut part_start_delimiter = format!("\r\n--{}\r\n", &boundary).into_bytes();
//                     let mut part_end_delimiter = format!("\r\n--{}--\r\n", &boundary).into_bytes();
//
//                     let mut bufferupload = [0; 1024];
//                     let mut received_data = Vec::new();
//
//                     loop {
//                         let bytes_read = stream_clone.read(&mut bufferupload).unwrap();
//                         if bytes_read == 0 {
//                             break;
//                         }
//
//                         received_data.extend_from_slice(&bufferupload[..bytes_read]);
//
//                         // Check if the received data contains the part start or end delimiter
//                         if received_data.windows(part_start_delimiter.len()).any(|window| window == &part_start_delimiter[..])
//                         || received_data.windows(part_end_delimiter.len()).any(|window| window == &part_end_delimiter[..]) {
//                             // Process the part (file data)
//                             let part_data = received_data.clone();
//
//                             // Extract necessary information from the part data and perform custom logic
//                             let content_type = Nstring::stringbetween(&request_clone, "Content-Type: ", "\r\n");
//                             let filename = Nstring::stringbetween(&request_clone, "filename=\"", "\"");
//
//                             // Write the file data to a specific directory
//                             let output_dir = "./testfiles/uploads";
//                             let output_path = format!("{}/{}", output_dir, filename);
//                             let mut file = File::create(output_path).unwrap();
//                             file.write_all(&part_data).unwrap();
//
//                             // Perform additional logic specific to MP3 files
//                             // For example, you can extract metadata, process audio, etc.
//
//                             // Send the response back to the client
//                             let response = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n", "text/plain");
//                             let message = "File upload for .nc file completed.";
//                             let response = format!("{}{}", response, message);
//                             stream_clone.write(response.as_bytes()).unwrap();
//                             return;
//                         }
//                     }
//                 });
//                 return;
//             }
//         }
//     }
// }
