// with this function you can split a string into a nscript arraystring and use it in loops

string = "Tommy|Karen|Herman"
array = split(string,"|")

for each in array {
    print(each)
}
