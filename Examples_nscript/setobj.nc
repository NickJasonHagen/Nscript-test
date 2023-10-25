// set object is the function to spawn a object from a class.
// the function will automaticly trigger the construct function from the class tree, the last inherentance wich has this function scope initialized
// will clone the construct function to the spawning object, after all properties are cloned, the construct will run wich allows you to set using self.

class playerbase{
    func construct(){
        self.name = self
    }
}

setobj("playerbase","BigJohn")
