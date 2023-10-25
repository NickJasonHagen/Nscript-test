// stacks, these are last in first out stacks. you can push things to a stack and pop them out one by one.
// stacks have a string reference. the first argument wil represent the name of the stack wich has to be unique, you can enter a variable but it has to hold the reference of the stack's reference as a string.


// define a unique reference
mystack = "mystackreference"

stackpush(mystack,"hearts 4")
stackpush("mystackreference","hearts 8") //<--- here you see the stack can also be used staticly by string.

func stackref(){
    return "mystackreference"
}

stackpush(stackref(),"hearts king") //<-- reffed by a function return 

mycard = stackpop(mystack)
