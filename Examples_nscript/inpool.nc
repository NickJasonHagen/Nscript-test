// pool system, this is the same format as a array! but using pooladd() and poolremove() 
// will make sure that each entree in this array is unique !
// if an entree is pushed twice the array is unchanged
// pooladd(array,entree) , poolremove(array,entree) < - both return the array as result

array = ""
array = pooladd(array,"Joyce")
array = pooladd(array,"Kim")
array = pooladd(array,"Alice")
array = pooladd(array,"David") //<--- now this one is already in the pool and wont show

array = poolremove(array,"David") // <-- gues the ladies dont want David to be in the pool ! 

// inpool returns 1 on found and 0 if not found! --! Added on v2.005
if inpool(array,"Kim") == 1 {
	print("Seems Kim is in the pool.")
}
