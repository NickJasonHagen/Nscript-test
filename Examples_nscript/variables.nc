// in nscript variables dont come with a prefix their just words.
// you can set a variabe with functions, strings, numbers, macro's switches.

myvar = "this is a variable made from a string"
myvar = 123
myvar = 0.123
myvar = listdir(@scriptdir)

// to escape the " symbol you can use \" inside a "" string. the interpreter will keep them in your string.
var = "this is a quote -> \" <-"

//multi line variables
// be aware of comments inside this !!
multilinevar = " this is a 
multiline variable // not a comment !! part of the string!
everything between the quotes
with exeptions of the escape \" 
the interpreter will format this
(tech: to 1 line of syntax for interpretation)"
