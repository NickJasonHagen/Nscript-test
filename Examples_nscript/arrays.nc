// arrays in nscript are still variables( they contain a delimerstring.)
// to define a array
//-- 
// if you call a number outbounds the array returns a empty string ""
// this will not cause a runtime error ( by default)!
//-----------------------------------------------
array = ["1","2","3","4"]

arraysize = array[?] // <-- the questionmark used on [] in a array variable
//results in the size of the array

print(array[1])
