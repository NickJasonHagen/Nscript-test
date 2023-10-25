// async loops these are somewhat like how go routines work.
// the difference is you have to identify a loop by a string.
// this reference is important when you want to break the loop
// the loop can be broken from anywhere as long as you reference it correctly

// Async loops set the self variable to their reference, using properties on this automaticly converts the refernce to an object.
// you can break the loop whitin the scope "Break self" or break it from elsewhere with "break myloopref"

myloopref = "mainloop"

async myloopref{
    break self
}
//this can be done outside of the scope.
break myloopref

