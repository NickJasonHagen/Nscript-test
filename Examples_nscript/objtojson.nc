// objtojson(obj) - objfromjson(objname,jsonstring)
// with these functions you can convert all properties of a obj/class
// to a json string and load it back in or sent it to other server to load it in,


class myclass{
    self.name = "superclass"
    self.descriptiuon = "im gonna be used for json"
}


// convert a class to json string
jsonstring = objtojson(myclass)

// load a new class name (arg 1) with a jsonstring (arg2) 
// and trow the objref back to the variable.
copiedclass = objfromjson("newuniqueclassreferencestring",jsonstring)

// you can now use the properties on this variable
// it will be performing on class newuniqueclassreferencestring
print(copiedclass.name,"green")
