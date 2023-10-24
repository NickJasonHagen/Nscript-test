
use crate::*;

pub const PROGRAM_DIR: &str = env!("CARGO_MANIFEST_DIR");
pub const NC_ARRAY_DELIM : &str = "]].n.c.arr.[[";
pub const NC_ASYNC_LOOPS_KEY : &str = "coroutine"; // async loops scopes keyword

pub struct Varmap{
    //global values of the vmap system
   pub varmap: HashMap<String,String>,
    pub codelevel: usize,
    pub parsinglevel: usize,
    pub debugmode: usize,
    pub strictness: usize
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

   pub  fn setobj(&mut self,obj: &str, toobj: &str) {
        let trimmedobj = &obj.trim();
        let trimmedtoobj = &toobj.trim();
        let getoldprops = self.inobj(&trimmedobj);
        let splitprops= split(&getoldprops,"|");
        for prop in splitprops {
            if prop != "" {
                let key = "".to_owned() + &trimmedobj + "." + &prop;
                let get = self.getvar(&key);
                let keynew = "".to_owned() + &trimmedtoobj + "." + prop;
                //println!("setting prop:{} with vallue {}",&keynew.yellow(),&get.as_str().red());
                self.setvar(keynew, get.as_str());
            }
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
        let objparenth =  "nscript_classparents__".to_owned() + &trimmedtoobj + "." + trimmedobj;
        self.setvar(objparenth, ".");
        // add child to parent obj
        let objchildh =  "nscript_classchilds__".to_owned() + &trimmedobj + "." + &trimmedtoobj;
        self.setvar(objchildh, ".");



        //vmap.setvar(functionregobj, &funcname); // reg the function to obj
    }
    pub fn inobj(&mut self,obj:&str) -> String {
        let isobj = "obj_".to_owned() + &obj.trim();
        let g = self.varmap.get_key_value(&isobj);
        match g {
            None => String::new(),
            Some((_i, k)) => k.to_owned()
        }
    }

    pub fn delobj(&mut self,obj: &str){

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
        let objparenth =  "nscript_classparents__".to_owned() + &obj;
        let parents = self.inobj(&objparenth);
        println!("parents: {}",parents);
        for eachparent in split(&parents,"|"){

            if eachparent != "" {
                // remove child from parent obj
                let objchildh =  "nscript_classchilds__".to_owned() + &eachparent.trim();
                self.delprop(&objchildh, &obj.trim());
            }
        }
    }
    pub fn delprop(&mut self,obj: &str,key: &str){
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
                        newindex = "".to_owned() + &newindex + &entree + "|";

                    }


                }
                if Nstring::fromright(&newindex, 1) == "|"{
                    newindex = Nstring::trimright(&newindex,1);
                }
                self.varmap.insert(objprops,newindex);

            }
        }
    }
    pub fn setvar(&mut self,key: String , value: &str){
        // this is the core function for storing all the data of the nscript code syntax.
        // all user script variables and classproperties go thru this
        // -----------------------------------------------------------
            if Nstring::instring(&key.trim(),".") == true { // obj property
            let spl = split(&key.trim(),".");
            let mut objname = String::new();

            let mut propname = String::new();
            if Nstring::instring(&spl[0],"*") {
                objname = self.getvar(&Nstring::replace(&spl[0],"*",""));
            }
            else {
                objname = "".to_owned() + &spl[0];
            }

            if Nstring::instring(&spl[1],"*") {
                propname = self.getvar(&Nstring::replace(&spl[1],"*",""));
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
                    let makesurecheck = "|".to_owned() + &k + "|";
                    if Nstring::instring(&makesurecheck, &propname) == false {
                        let nexindex = k.to_owned() + "|" + &propname;
                        self.varmap.insert(objprops,nexindex.to_owned());
//println!("Setvar: index= {}",&nexindex);
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
    pub fn getvar(&mut self,key: &str)->String{
        // this is the core function of nscript to get data
        // it will check for variables or class.properties
        // ----------------------------------------------------
        if Nstring::instring(&key,".") == true { // obj property
            let spl = split(&key,".");
            let mut objname = String::new();

            let mut propname = String::new();
            if Nstring::instring(&spl[0],"*") {
                objname = self.getvar(&Nstring::replace(&spl[0],"*",""));
           }
            else {
                objname = "".to_owned() + &spl[0];
            }
           if Nstring::instring(&spl[1],"*") {
                propname = self.getvar(&Nstring::replace(&spl[1],"*",""));

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
    pub fn getprop(&mut self, obj: &str, prop: &str) -> String {
        let fullkey = "obj_".to_owned() + &obj.to_string().trim() + "__" + &prop.to_string().trim();
        //println!("fullkeyobj:{}",&fullkey.red());
        let g = self.varmap.get_key_value(&fullkey);
        match g {
            None => String::new(),
            Some((_i, k)) => k.to_owned(),
        }
    }
    pub fn setprop(&mut self, obj: &str, prop: &str, value: &str) {
        // this shoulnd be used , make the syntax as var.prop and trow it to
        // setvar() / w getvar
        let fullkey = "obj_".to_owned() + &obj.trim() + "__" + &prop.trim();
        self.varmap.insert(fullkey, value.trim().to_owned());

        // set obj index !!
        let objprops = "obj_".to_owned() + &obj.trim(); // index of all the properties - managed
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
    pub fn objparents(&mut self, obj: &str) -> String {
        let key = "nscript_classparents__".to_owned() + obj;
        let g = self.inobj(&key);
        return g;
    }
    pub fn objchildren(&mut self, obj: &str) -> String {
        let key = "nscript_classchilds__".to_owned() + obj;
        let g = self.inobj(&key);
        return g;
    }

    pub fn setcode(&mut self, name: &str, code: &str) {
        // interally used to store codesheet/blocks
        // ----------------------------------------
        let codename = "code__".to_owned() + name;
        self.varmap.insert(codename, code.to_owned());
    }

 pub fn getcode(&mut self, name: &str) -> String {
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
    pub fn codelvlup(&mut self) {
        // this is very important, the code level is nessesary for the internalparams
        // when nscript recurses from one function to another the arguments/parameters
        // are set on the background internalparam1-10 they get a prefix with this number
        // making them act like a stack during recursing, so when a function goes back
        // the parameters are still there.
        // ------------------------------------------------------------------------
        self.codelevel = self.codelevel + 1
    }
    pub fn codelvldown(&mut self) {
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
    pub fn iscodelevel(&mut self) -> String {
        self.codelevel.to_string()
    }
}

pub fn nscript_execute_script(
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

    // this part sets a sheet-buffer wich is used to parse
    // it extracts classes and functions

    vmap.parsinglevel = vmap.parsinglevel + 1;
    let thisparsingsheet = "_".to_owned() + &vmap.parsinglevel.to_string() +"__interpretercode";
    let thisparsingsubsheet = "_".to_owned() + &vmap.parsinglevel.to_string() +"__interpretersubcode";
    vmap.setprop("__interpreter","parsingsheet",&thisparsingsheet);
    vmap.setprop("__interpreter","parsingsubsheet",&thisparsingsubsheet);
    let argusvec: Vec<String> = vec![param1.to_owned(),param2.to_owned(), param3.to_owned(), param4.to_owned(), param5.to_owned(), param6.to_owned(), param7.to_owned(), param8.to_owned(), param9.to_owned()];
    nscript_setparams_exec(&argusvec,vmap);
    let mut code = String::new();
    if Nstring::fromleft(&code,4) == "RAW>" {
        code = Nstring::trimleft(&file,4);
    }
    else{
        code = read_file_utf8(&file);
    }
    vmap.setcode(&thisparsingsheet,&code);
    //extract the functions and classes from the sheet.
    nscript_class_scopeextract(vmap);
    code = vmap.getcode(&thisparsingsheet);
    nscript_func_scopeextract("", vmap);
    code = vmap.getcode(&thisparsingsheet);

    // run the code after classes and functions are all loaded in
    let ret = nscript_parsesheet(&code, vmap);

    // set back the interpreter used parsing sheet
    vmap.parsinglevel = vmap.parsinglevel - 1;
    let thisparsingsheet = "_".to_owned() + &vmap.parsinglevel.to_string() +"__interpretercode";
    let thisparsingsubsheet = "_".to_owned() + &vmap.parsinglevel.to_string() +"__interpretersubcode";
    vmap.setprop("__interpreter","parsingsheet",&thisparsingsheet);
    vmap.setprop("__interpreter","parsingsubsheet",&thisparsingsubsheet);

    // return the value of the code
    ret
}

pub fn nscript_parsesheet(coderaw: &str, vmap: &mut Varmap) -> String {
    // this is the interal function to parse a prepare and parse a block
    // it is used on nscript_execute_script() / exec()
    // this function does not extract classes or function scopes!
    // if you need to run a new script see nscript_exec_script() this is where it starts
    // -------------------------------------------------------------------------------
    let argnew = "".to_owned() + &vmap.codelevel.to_string() + "__internalparam"; // form new varnames bkgrnd paramx
    let levelbellow = vmap.codelevel - 1 ;
    let argnewbroken = "".to_owned() + &levelbellow.to_string()  + "__internalparam"; // form new varnames bkgrnd paramx
    let argnewfix = "".to_owned() + &levelbellow.to_string() + "__" + &vmap.codelevel.to_string() + "__internalparam"; // form new varnames bkgrnd paramx
    let code = kill_bill(&Nstring::replace(&coderaw, "internalparam", &argnew));
    let code = Nstring::replace(&code, &argnewfix, &argnewbroken);
    vmap.codelvlup();

    let fixedcode = code.to_owned();// + LINE_ENDING;
    let fixedcode = nscript_stripcomments(&fixedcode);
    let fixedcode = trim_lines(&fixedcode);
    let fixedcode = nscript_stringextract(&fixedcode);
let fixedcode = nscript_formatargumentspaces(&fixedcode);
    let fixedcode  = nscript_scopeextract(&fixedcode);

    //let fixedcode = nscript_compilesheet(&code);
    //println!("code:{}",&fixedcode);
    let mut toreturn = String::new();
    let lines = fixedcode.split("\n");
    for line in lines {
        if line != "" {
            //let fixedline = split(&line,"//")[0].trim();
            //if line != ""{
                toreturn = nscript_parseline(&line ,vmap);
            //}
            //  when parse line sees return on word[0] it will add "RET=>"
            // this will break this loop and return the value back to callfn/nscript_func
            if Nstring::fromleft(&toreturn, 5) == "RET=>" {
                vmap.codelvldown();

                return Nstring::trimleft(&toreturn, 5);
            }
        }
    }
    vmap.codelvldown();

    return "..".to_owned();
}

pub fn nscript_parseformattedsheet(coderaw: &str, vmap: &mut Varmap) -> String {
    // this function runs a block of scope without jumping up a codelvl
    // this is used for at spot runtime blocks like For,While loops.
    // it is faster for execution then parse_sheet() but it requires nscript_compilesheet()
    // to have preparred the block for proper execution
    // ------------------------------------------------------------------------
let mut toreturn = String::new();
    let lines = coderaw.split("\n");
    for line in lines {
        if line != "" {
            toreturn = nscript_parseline(&line, vmap);
            if Nstring::instring(&toreturn, "RET=>") == true {
               return Nstring::replace(&toreturn, "RET=>", "");
            }
        }
    }

    return "..".to_owned();
}


pub fn nscript_parseline(line: &str, vmap: &mut Varmap) -> String {
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
            if words[0] == "break" || words[0] == "Break"{ // <== used in loops{}
                    return "RET=>break".to_owned();
            }
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
                "loop" => {
                    let scopeargs = Nstring::stringbetween(&words[1], "(", ")");
                    let splitscopearg = split(&scopeargs,",");
                    let loopblock = nscript_formatsheet(&nscript_unpackscopereturnclean(&splitscopearg[1],&splitscopearg[0],vmap));
                    loop {

                        let ret = nscript_parseformattedsheet(&loopblock, vmap);
                        if ret == "break" {
                            break;
                        }
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
                "match" | "Match" => {

                    return nscript_match(&words[1],&words[2],vmap);
                }
                NC_ASYNC_LOOPS_KEY => {
                    let scopeargs = Nstring::stringbetween(&words[words.len()-1], "(", ")");
                    let splitscopearg = split(&scopeargs,",");
                    let loopref = nscript_checkvar(&words[1],vmap);

                    let loopscope = nscript_formatsheet(&nscript_unpackscopereturnclean(&splitscopearg[1],&splitscopearg[0],vmap));
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
                                "var" => {
                                    let ismacro = nscript_checkvar(words[2],vmap);
                                    vmap.setvar(words[0].to_string(),&ismacro );
                                    return words[2].to_string();
                                }
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
                    // constructor function if inherented, be triggered after instantiation
                    let isconfn = "".to_owned() + &obj2 + ".construct()"; // should only execute if it exists.. else continue

                        nscript_func(&isconfn, vmap);// if empty returns else exec


                    return String::new();
                }
            }
             if words.len() > 4 { // lines that are of 5 words or more
                match words[0]{
                    // for loops
                    "elseif" =>{
                        if vmap.getvar(&"___if") == "false"{ // last if statement must be false
                            if parse_and_check_statement(&words,vmap){
                                vmap.setvar("___if".to_owned(),"true"); // <-- set it true so other
                                // elseifs wont trigger
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

                        //println!("if!");
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
                if words[2] == "match" && words[1] == "=" ||  words[2] == "Match" && words[1] == "=" {
                    // this is a switch scope with a variable set.
                    let switchreturn = nscript_match(&words[3],&words[4],vmap);
                    //println!("var = switch = {}",switchreturn);
                    vmap.setvar(words[0].to_owned(),&Nstring::trimleft(&switchreturn,5));
                    return "".to_owned();

                }

                if words[3] == "+" || words[3] == "-" || words[3] == "*" || words[3] == "/"  {
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

pub fn nscript_class_scopeextract(vmap: &mut Varmap){
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
                vmap.setvar(classname[0].trim().to_string().clone(), &classname[0].trim()); // assign classname = classname

                if classname.len() > 1 {
                    let toobjname = nscript_checkvar(&classname[0].trim(),vmap);
                    //println!("OBJ CLONE: {}",&toobjname);
                    vmap.setobj(&classname[1].trim(), &toobjname);
                }
                let block = extract_scope(&eachclass);// extract the class scope between { }
                vmap.setcode(&parsesubcode,&block);// assign the subscope

                nscript_func_scopeextract(classname[0],vmap);// extract functions from class scope
                let blocknew = vmap.getcode(&parsesubcode); // remaining when functions are removed
                //println!("Subblock::{}",&blocknew);
                vmap.setvar("self".to_owned(), &classname[0].trim());// assigning self var self.
                //println!("running class extraction assigning self:{}",&classname[0]);
                let blocknew = Nstring::replace(&blocknew, "self.", "*self.");// Reflect self!!!
                nscript_parsesheet(&blocknew, vmap);// run the remaining as classblock.
                //println!("Blc:{}",&blocknew);
                let toreplace = "class".to_owned() + &classnamepart +  &block ;
                if Nstring::instring(&toreplace, "{") && Nstring::instring(&toreplace, "}")  {
                    //println!("The replacement: {}",&toreplace);
                    vmap.setcode(&parsecode,&Nstring::replace(&code, &toreplace,"" ));
                    //println!("FoundClass:{}",&classname[0]);

                    //println!("classcode:{}",&vmap.getcode("___interpretercode"));

                }
                let isconfn = "".to_owned() + &classname[0].trim() + ".construct()"; // should only execute if it exists.. else continue

                    nscript_func(&isconfn, vmap);

            }
        }
        i +=1;
    }
    //code
}

pub fn nscript_func_scopeextract(selfvar: &str,vmap: &mut Varmap) {
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
                    vmap.setcode(&classnamefixed, &nscript_formatsheet(&block));
                    //println!("Setting func: {} \n with block: \n{}",&functionregobj, &block);
                }
                else {
                    // if not inside a classscope
                    //println!("new:{}",&block);
                     let block = trim_lines(&block);
                     let block = nscript_stringextract(&block);
                    // let block  = nscript_scopeextract(&block);

                    vmap.setcode(&classname, &nscript_formatsheet(&block));
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


pub fn nscript_checkvar(key: &str,vmap: &mut Varmap) -> String {
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

                    let mut unwrap = "".to_owned() + &key;
                        if split(&unwrap,"(").len() > 2 {
                            unwrap = nscript_funcextract(&key, vmap);

                        }


                        let rawargs = Nstring::stringbetween(&unwrap, "(", ")");
                    let mut fnname = "".to_owned() + split(&unwrap, "(")[0];
                    if Nstring::fromleft(&fnname,1) == "*"{
                        fnname = nscript_checkvar(&fnname,vmap);
                    }
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

pub fn nscript_scopeextract(text: &str) -> String {
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

pub fn nscript_stringextract(text: &str) -> String {
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

pub fn nscript_packscope(code: &str,scopeid: &str) -> String {
    // this is a internally used function to encapsulate scopes.
    // the scopes are being packed back into 1 line so its faster and easier to interpretate.
    // scopeid is a unique number wich is used to pack the scope, this way each scope can be
    // unpacked sepperatly.
    // --------------------------------------------------------------------------------------

    let newid = "%".to_owned() + scopeid + "%";
        let mut ifcodenew = Nstring::replace(&nscript_formatsheet(&code), " ", "%id%sp%");
        ifcodenew = Nstring::replace(&ifcodenew, LINE_ENDING, "%id%lf%");
        ifcodenew = Nstring::replace(&ifcodenew, "(", "%id%bo%");
        ifcodenew = Nstring::replace(&ifcodenew, ")", "%id%bc%");
        ifcodenew = Nstring::replace(&ifcodenew, "{", "%id%cbo%");
        ifcodenew = Nstring::replace(&ifcodenew, "}", "%id%cbc%");
        ifcodenew = Nstring::replace(&ifcodenew, ",", "%id%c%");

    let ret = " scope(".to_owned() + &scopeid + "," + &Nstring::replace(&ifcodenew, "%id%", &newid) + ")";
    ret
}

pub fn nscript_unpackscope(code: &str,scopeid: &str,vmap: &mut Varmap) -> String {
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
    let res = nscript_parseformattedsheet(&ifcodenew,vmap);
    if res ==".." {
        res
    }
    else {
        "RET=>".to_owned() + &res
    }
}

pub fn nscript_unpackscopereturnclean(code: &str,scopeid: &str,vmap: &mut Varmap) -> String {
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

pub fn nscript_formatsheet(code: &str) -> String{
    // this function preformats a sheet and these can be run with nscript_parsecompiledsheet()
    // used in : For / While / func
    // ------------------------------------------------------------------------------
    // let mut newcode = String::new();
    // let lines = code.split(LINE_ENDING);
    // for line in lines {
    //     let fxline = split(&line,"//")[0];
    //     newcode = "".to_owned() + &newcode + &fxline + LINE_ENDING;
    // }
    nscript_scopeextract(&nscript_formatargumentspaces(&nscript_stringextract(&trim_lines(&kill_bill(&nscript_stripcomments(&code))))))
}


pub fn nscript_func(func: &str, vmap: &mut Varmap) -> String {
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

        }

        else {
            isclass = splitfn[0].trim().to_string().clone();
        }

        let cleanfnname = split(&splitfn[1], "(");
        let mut fnname = String::from(cleanfnname[0].trim());
        let mut reg = "nscript_classfuncs__".to_owned()  + &isclass +"."+ &fnname;

        //fnname = vmap.checkvar(&fnname);
        if Nstring::fromleft(&fnname,1) == "*" {
            fnname = nscript_checkvar(&Nstring::replace(&fnname,"*",""), vmap);
            reg = "nscript_classfuncs__".to_owned()  + &isclass +"."+ &fnname;

            //println!("going to check for fn:{}",&fnname);
        }

         if Nstring::fromleft(&isclass,1) == "*" {
            isclass = nscript_checkvar(&Nstring::replace(&isclass,"*",""), vmap);
            reg = "nscript_classfuncs__".to_owned()  + &isclass +"."+ &fnname;

            //println!("going to check for class:{}",&isclass);
        }

        let rootfnobj = vmap.getvar(&reg); // get root obj where the func is located.
        let rootfnfullname = "".to_owned() + &rootfnobj + "__" + &fnname;//+ &rootfnobj + "__" + &fnname;
        iscodebblock = vmap.getcode(&rootfnfullname); // load code
        vmap.stackpush("___self", &isclass);
        vmap.setvar("self".to_owned(), &isclass);
        usedself = &isclass;
        iscodebblock = Nstring::replace(&iscodebblock, "self.", "*self."); // change all to the obj itself.
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




pub fn nscript_formatargumentspaces(code: &str) -> String{
    let mut line = String::new(); // buffer used for changes
    let mut fixed = String::new();// used to return
    let mut linebuf = String::new();
    let fixemptyargscode = Nstring::replace(&code,"()","(\"\")");
    for each in split(&fixemptyargscode,"\n"){ // loop lines
        line = each.to_string(); // set default
        linebuf = line.clone(); // create a buffer we can strip
        loop {
            let getbetween = Nstring::stringbetween(&linebuf,"(",")");
            //check if "" means its done.
            if getbetween == "" {break}
            // create a fixed string
            let fixbetween = Nstring::replace(&getbetween," ","");
            line = Nstring::replace(&line,&getbetween,&fixbetween);
            // strip the buf from what its done, and loop on.
            let  bufstrip = split(&linebuf,&getbetween);
            let tostrip = bufstrip[0].to_owned() +   &getbetween ;
            linebuf = Nstring::replace(&linebuf,&tostrip,"");

        }
        fixed = fixed + &line + "\n";
    }
    fixed
}

pub fn extract_scope(text: &str) -> String {
    // a internal function to extract the scopes
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

pub fn nscript_getprefix(s: &str) -> String {
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

pub fn nscript_setdebugmode(id: &str,vmap: &mut Varmap){
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
pub fn nscript_setrestrictionmode(id: &str,vmap: &mut Varmap){
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


pub fn nscript_setparams_exec(args: &Vec<String>,vmap: &mut Varmap){
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


pub fn nscript_runfncall(fnword: &str, vmap: &mut Varmap) -> String {
    // yeah i know right this is pretty funny, first day of rust lol
    // yeah i gotta remake this i know !
    // ----------------------------------------------------------
    let mut fnname = fnword.to_string();
    if Nstring::instring(&split(&fnname,"(")[0],"*") {
        let fnsplit = split(split(&fnname,"(")[0],".");
        if fnsplit.len() <= 2 {

            if fnsplit.len() == 1 {
                 fnname = "".to_owned() + &nscript_checkvar(&Nstring::replace(&fnsplit[0], "*", ""),vmap)  + "(" + &split(&fnname,"(")[1] + ")";
            }
            else if fnsplit.len() == 2 {
                 fnname = "".to_owned() + &nscript_checkvar(&Nstring::replace(&fnsplit[0], "*", ""),vmap) + "." + &nscript_checkvar(&Nstring::replace(&fnsplit[1], "&", ""),vmap) + "(" + &split(&fnname,"(")[1] + ")";
            }
            if vmap.getcode(&Nstring::replace(&split(&fnname,"(")[0],".","__")) != "" {
                return nscript_func(&fnname,vmap);
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


pub fn nscript_interpreterdebug(debugstr: &str,dbg_mode: usize,res_mode: usize){
    // exclude the construct/destruct functions on instantiation/deletion
    if Nstring::instring(&debugstr,"construct") || Nstring::instring(&debugstr,"destruct") {
        return;
    }
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

pub fn nscript_clearparams_handleconnections(vmap: &mut Varmap){
    // clears params without going level down
        for r in 0..10 {
            let paramx = r + 1;
            let pname = "".to_owned() + &vmap.codelevel.to_string() + "__internalparam" + &paramx.to_string();
            vmap.setvar(pname, ""); // clear all param arguments
        }
}
//----------------RegionNscript------------------\/--------------
pub fn is_number(input: &str) -> bool {
    input.parse::<f64>().is_ok()
}

pub fn is_float(input: &str) -> bool {
    input.parse::<f32>().is_ok() || input.parse::<f64>().is_ok()
}

pub fn nscript_getmacro(mac: &str,vmap: &mut Varmap) -> String {
    //this function calculated and returns macro's / starting with the @ symbol
    //functional variables.
    //----------------------------------------------------
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
        "@crlf" => String::from("\r\n"),
        "@lf" => String::from("\n"),
        "@pid" => get_own_pid().to_string(),
        "@emptystring" => String::new(),//<- internal-parser used!!

        _ => String::from(mac),
    }
}

pub fn nscript_match(entree: &str,scope:&str,vmap: &mut Varmap) -> String {
    // This is the interpreter for the match system
    // used in parse_line() internally , the return value
    // will be set if a variable caches it.
    // ---------------------------------------------------
    let scopeargs = Nstring::stringbetween(&scope, "(", ")");
    let splitscopearg = split(&scopeargs,",");
    let evalentree = nscript_checkvar(entree, vmap);
    let switchscope = nscript_unpackscopereturnclean(&splitscopearg[1],&splitscopearg[0],vmap);
    let splitcase = split(&switchscope,"\n");
    for thiscase in splitcase {
        // splitline[0] are checks /[1] return/(scope)
        let splitline = split(&thiscase," =>");
        if splitline.len() > 1 {
            // check multiple by pipes looped
            let splitstatements = split(&splitline[0]," | ");
            for eachstatement in splitstatements {
                let stateeval = nscript_checkvar(&eachstatement, vmap);
                if stateeval == evalentree || stateeval == "_" {
                    // check for scope nest
                    if Nstring::instring(&splitline[1],"scope(") {
                        // prep and run nest
                        let casescopeargs = Nstring::stringbetween(&splitline[1], "(", ")");
                        let splitcasescopearg = split(&casescopeargs,",");
                        let ret = nscript_unpackscope(&splitcasescopearg[1],&splitcasescopearg[0],vmap);
                        // check for return (if nests could have em.)
                        if Nstring::fromleft(&ret,5) == "RET=>" {
                            return ret;
                        }
                        // return the last line's result as return
                        return "".to_owned() + &ret; // ensure last return as result
                    }
                    else {
                        // if no scope, 1 word can return
                        return "".to_owned() + &nscript_checkvar(&splitline[1].trim(),vmap);
                    }
                }
            }
        }
    }
nscript_interpreterdebug("Syntax error on match scope!",vmap.debugmode,vmap.strictness);
String::new()// in case of full-derp-mode
}

pub fn nscript_array(entrees: &str,vmap: &mut Varmap ) -> String{
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

pub fn line_to_words(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

pub fn nscript_stripcomments(coderaw: &str) -> String{
    // strips off all comments per lines.
    let lines = coderaw.split("\n");
    let mut newcode = String::new();
    for line in lines {
        if line != "" {
            newcode = newcode + &split(&line,"//")[0].trim()+"\n";
        }
    }
    newcode
}

pub fn parse_and_check_statement(words: &[&str], vmap: &mut Varmap) -> bool {
    // this is how you parse a unknown lenght of statements
    // they can be mixed And/or
    // this function will return a bool.
    // -------------------------------------------------------------
    if words.len() < 4 {
            if words[0] == "if" || words[0] == "elseif"{
            nscript_interpreterdebug("There is a syntax error on a if statement", vmap.debugmode, vmap.strictness);
            return false; // Invalid syntax or empty statement
        }
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

pub fn nscript_foreach(code: &str,invar: &str,inarray: &str,vmap: &mut Varmap) {
    // this function is used for something in array {}
    //------------------------------------------------------
    if inarray == "" {
        return
    }
    let evalarray = nscript_checkvar(&inarray,vmap);
    let array = split(&evalarray,&NC_ARRAY_DELIM);
    let scopeid = &Nstring::stringbetween(&code, "scope(", ",");
    let cleancode = nscript_formatsheet(&nscript_stringextract(&nscript_unpackscopereturnclean(&code, scopeid, vmap)));
    for isin in array {
        vmap.setvar(invar.to_owned(),&isin);
        nscript_parseformattedsheet(&cleancode, vmap);
    }
}

pub fn nscript_forto(code: &str,invar: &str,inarray: &str, vmap: &mut Varmap) {
    // this is a for x to 100 {} system where x represents a number
    // -----------------------------------------------------------
    let evalarray = nscript_checkvar(&inarray,vmap);
    let scopeid = &Nstring::stringbetween(&code, "scope(", ",");

    let cleancode = nscript_formatsheet(&nscript_stringextract(&nscript_unpackscopereturnclean(&code, scopeid, vmap)));
    for isin in 1..nscript_i32(&evalarray)+1{
        vmap.setvar(invar.to_owned(),&isin.to_string());
        nscript_parseformattedsheet(&cleancode, vmap);
    }
}

pub fn nscript_checkstatement(a: &str, b: &str, c: &str, vmap: &mut Varmap) -> bool {
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

pub fn nscript_combine(a: &Vec<&str>, vmap: &mut Varmap) -> String {
    // this is a function wich comes strings concetinate.
    // -------------------------------------------------
    let mut makestring = String::new();
    for r in 3..a.len() {
        makestring = makestring + &nscript_checkvar(&a[r],vmap);
    }
    return makestring;
}

pub fn nscript_space(a: &Vec<&str>, vmap: &mut Varmap) -> String {
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

pub fn nscript_f64(intasstr: &str) -> f64 {
    // this is used on the nscript math system
    let onerr: f64 = 0.0;
    match intasstr.parse::<f64>() {
        Ok(n) => return n,
        Err(e) => return onerr,
    }
}

pub fn nscript_i32(intasstr: &str) -> i32 {
    let onerr: i32 = 0;
    match intasstr.parse::<i32>() {
        Ok(n) => return n,
        Err(e) => return onerr,
    }
}

pub fn nscript_math(a: &str, method: &str, b: &str, vmap: &mut Varmap) -> String {
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

pub fn nscript_runmath(splitline: &Vec<&str>,indexpars: usize, vmap: &mut Varmap) -> String{
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

pub fn nscript_getarguments(fnword: &str, vmap: &mut Varmap) -> (Vec<String>, usize) {
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

pub fn nscript_funcextract(text: &str,vmap: &mut Varmap) -> String {
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
                if Nstring::fromleft(&splitstr[splitstr.len()-2],1) == "*"{
                    subfunction = "".to_owned() + &nscript_checkvar(&Nstring::replace(&thisfnnamefix,"*",""), vmap) + "(" + &splitscope[0]  + ")";
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
                if Nstring::fromleft(&splitstr[splitstr.len()-2],1) == "*" {
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


pub fn trim_lines(input: &str) -> String {
    let trimmed_lines: Vec<String> = input
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    trimmed_lines.join("\n")
}



pub fn nscript_loops(vmap: &mut Varmap){
    let activeloops = vmap.inobj("nscript_loops");

    if activeloops != "" {
//println!("running loop:[{}]",&activeloops);

        let subloops = split(&activeloops, "|");
        for x in subloops {
            let d = vmap.getprop("nscript_loops", &x);
            vmap.stackpush("___self", &x);
            vmap.setvar("self".to_owned(), &x);
            nscript_parseformattedsheet(&d, vmap);
            vmap.stackpop("___self");
            vmap.setvar("self".to_owned(), &x);

        }
    }
}


pub fn sleep(milliseconds: u64) {
    let duration = Duration::from_millis(milliseconds);
    thread::sleep(duration);
}

pub fn read_to_string(filename: &str) -> String {//<<-- if IDE says its not used, its a LIE!:w

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

pub fn kill_bill(string: &str) -> String {
    // function removes the poisoness \r\n crap from satansoft systems and makes the interpreter
    // clean to run at while being free of evil..
    Nstring::replace(&string,"\r\n","\n")
}

pub fn objtojson(obj: &str,vmap: &mut Varmap) -> String{
    let mut jsonstring = String::from("{");
    for propname in split(&vmap.inobj(&obj),"|"){
        let nscriptvar = obj.to_owned() + "." + &propname;
        jsonstring = jsonstring + "\"" + &propname + "\": \"" + &vmap.getvar(&nscriptvar) + "\"," ;

    }
    if Nstring::fromright(&jsonstring,1) == "," {
        jsonstring = Nstring::trimright(&jsonstring,1);
    }
    jsonstring = jsonstring + "}";
    jsonstring
}

pub fn objfromjson(obj: &str,json: &str,vmap: &mut Varmap){
    let json = Nstring::trimright(&Nstring::trimleft(&json,1),1); // strip {}
    for each in split(&json,"\",") {
        let splitprop = split(&each,"\": \"");
        if splitprop.len() > 1 {
            let nscriptprop = "".to_owned()+ &obj.trim() + "."+ &Nstring::trimleft(&splitprop[0],1);
            println!("setting [{}] with data[{}]",&nscriptprop,&splitprop[1]);
            //vmap.setvar(nscriptprop.to_owned(),&splitprop[1].trim());
            vmap.setprop(&obj.trim(),&Nstring::trimleft(&splitprop[0],1),&splitprop[1]);
        }
    }
}

pub fn nscript_replaceparams(code: &str,thisargument: &str) -> String{
    // this can be used to make sure that there no unintended replacements,
    // these bellow should be in my view the only appliable ways to suit the var.
    let mut block = code.to_owned();
    let param = "\n".to_owned() + "internalparam"  +  " ";
    let torep = "\n".to_owned() + &thisargument +" ";
    block = Nstring::replace(&block,&torep, &param);
    let param = "(".to_owned() + "internalparam" + "";
    let torep = "(".to_owned() + &thisargument + "";
    block = Nstring::replace(&block,&torep, &param);
    let param = ",".to_owned() + "internalparam" + "";
    let torep = ",".to_owned() + &thisargument + "";
    block = Nstring::replace(&block,&torep, &param);
    //
    let param = " ".to_owned() + " internalparam" + "";
    let torep = " ".to_owned() + &thisargument + "";
    block = Nstring::replace(&block,&torep, &param);
    let param = "*".to_owned() + "internalparam" + "";
    let torep = "*".to_owned() + &thisargument + "";
    block = Nstring::replace(&block,&torep, &param);
    block
}

pub fn nscript_threadscope(code: &str){
let codeclone = "RAW>".to_owned() + code;
    thread::spawn(move || {

        let mut threadvmap = Varmap::new();

        nscript_execute_script(&codeclone,"","","","","","","","","",&mut threadvmap);
        loop {
            nscript_loops(&mut threadvmap);
            let activeloops = threadvmap.inobj("nscript_loops");

            if activeloops != "" {
                break;
            }
        }
    });
}
