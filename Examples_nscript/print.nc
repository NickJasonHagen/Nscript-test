// print has 2 arguments first is your msg
// the second can be a color as a string, "green", red ,blue , yellow
// print returns your printed value so you can use it as a set aswell
print("helloWorld","green")
print("helloWorld","blue")
print("helloWorld","yellow")
print("helloWorld","red")

// bellow you can see the print function arround a fileread
// print simply returns what you print and print it.
// you can use it to debug and remove it at anytime wihtout breaking code.
// just take care of the ()
filedata = print(fileread("./somefile.txt"),"red")  

//colors new::

colorarray = ["b","r","y","g","c","p","m"]

for color in colorarray{
    print(combine("the color =",color),color)
       // bright version br , bb ,bg etc
    print(combine("the color =",color),combine("b",color))
}


