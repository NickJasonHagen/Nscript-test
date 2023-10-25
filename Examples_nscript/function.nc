// this example shows how to make a function wich takes arguments.
// ! the arguments names you assign inside the function scopes are local
// this ,means inside this scope those names only exist, as everything else in nscript is global by default.
// take this in mind! 

func myfirstfunction(name){
    ret = combine "hello " name " how are you?"
    return ret
} 

myreturnvar = myfirstfunction("Big John")
print(myreturnvar)
