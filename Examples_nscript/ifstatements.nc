// if statements allow you to perform checks you can nest them as much as you like,

if 1 == 1 {
    print("true")
}
// the else scopes work somewhat different, they act on the last statement so you can kinnda still put code between the if and else scope.. :P well enjoy !
else {
    print("not tru")
}

// you can use and by "and" / "&&"
if 1 == 1 and 2 == 2 {
    print("also true")
}
if 1 == 1 && 2 == 2 {
    print("also true")
}

// you can use or by "or" / "||"
if 1 > 1 or 2 == 2 {
    print("also true")
}
if 1 != 1 || 2 == 2 {
    print("also true")
}

// you can also combine and or and nest them

if user.status == "loggedin" || user == "Big John" {
    if 1 == 1 {
        print("true as shit","green")
    }
}
