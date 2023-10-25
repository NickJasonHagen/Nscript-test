// function/variable reflection/dispatching
// these can be used on functions, class-functions and variable-properties
// if is contains the * symbol itwill be the evaluated data as a string used to identify.
class a {
	func test(){
		print("oi")
	}
	self.info = "bladibla"
}

classname = "a"
functionname = "test"
prop = "info"
*classname.test() // reflecting the classnamepart
a.*functionname() // reflecting the function part
*classname.*functionname()  // reflecting both classname and function
print(a.*prop) // reflecting the property part
print(*classname.*prop) // yeah you get it !
