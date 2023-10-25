// combine is a special line syntax and can also be used as a function
// it concetinates strings, or combines them


user = "Big John"
// the linebellow , all words of the line after combine , will be evaluated and concatinated
file = combine @scriptdir "database/" user ".db"
print(file)


ORrrrrrrrrrrrrrrrrr

// keep in mind a function cannot have spaces elsewhere of inside the "" this breaks the syntax if done!
// combine as a function call can take up to 9 arguments i believe
print(combine(@scriptdir,"database/",user,".db"))
