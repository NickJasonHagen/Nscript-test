//classes, this is where the real power of nscript comes to play, classes are unique referenced group object they can hold properties and functions
// a class can spawn new objects, this means a new reference is being made by a clone of a object, all properties and functions of the objects current state will be cloned.
// after a object clones all the properties it will execute a function called self.construct() you can use this to post-clone change things,



// when a class scope loads it will set a variable with the classname to a string holding the classname
// nscript classes have a string as a reference , a variable holding the class is nothing more then a string with the unique
// reference of the class as a "string"
class playerbase {
    func destruct(){
        print(combine("player:",self," deleted!")
    }
    func construct(){
        print(combine("player:",self," loaded!")
    }
// inside class scopes / functions / loops you can use the self.var self will be set to the object using the method.
// ((&self is a reflected variabe))
    self.usertype = "user"
}

class admin {
    self.usertype = "admin"
}

obj pete : playerbase
obj john : playerbase
obj john : admin

getallplayers = objgetchildren(playerbase) //<--- this function will get all spawned objects that popped out of the baseclass

getallprops = inobj(playerbase) // <--- this returns a array of all the propertie references of the oject

objdel(pete) // <-- this will execute the self.destruct() function if for example used in a game
// you be able to delete models and do some stuff etc. after the .destruct is done all properties and heritiges will be
//deleted.


// !! important , class variables function calls only work when the variable
// hold the same name as a string as the variable name
// if this is not the case you need to reflect
class something{
    func some(){
    }
//someclass
}

othername = objfromjson("actualname",objtojson(something))
// here variable othervar does not hold the same name , the reference of the object
// here is actualname, to reflect the classmethod use the *symbol ( used to be & but has been changed on v2.005)
*othername.some() // <-- triggerls actualname.some()
a = "some"
*othername.*a() // <-- triggerls actualname.some()

//reflecting also works on props and varnames

someobj = "someobj"
array = ["name","age","gender"]
array2 ["BigJohn","54","superalphamale"]
for x to 3 {
// the name of the property is here by reflected, and set with the data from the other array.
    someobj.*array[x] = array2[x]
}


