// following shows how to do a match (//switch) like thing

tocheck = "4"

match tocheck{
    "1" | "2" =>{
        print("first")
    }
    "3" => {
        print("second")
    }
    "4" => {
        print("fourth")
    }
    _ => {
        "if none found option underscore be used, only if used last!"
    }
}

// you can also catch a variable by a match
// on the match you can use a scope OR 1 word this be returned 
// to the variable. the last lines result is auto-returned.
myvar = match tocheck{
    "1" | "2" => print("first") //< no scope usage. print returns !
    "3" => "staticstring" //<-- static return
    "4" => {
        if @day == "01" {
            return "first day of the month" //<- use return to breakback
        }
        "not the first day"
    }
    _ => {
        "if none found option underscore be used, only if used last!"
    }
}
