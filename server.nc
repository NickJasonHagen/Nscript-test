//server begin code
class server {
    func start(){
        print("ServerStarted","green")
    }

    func onconnect(clientIP){
       activitytimer = timerinit()
makenew = ""
getlist = ""
url = ""
       }

    server.debugmode = debugmode( 1 )
    server.restrictedmode = restrictionmode(   0)
    print("ServerScope Executed!!" ,"red" )
print( "whoooooooooooooooooooooooooooooooooooooooooop" , "green"   )
}


throttleMs = 500
// throttle the server so the powerusage drops. adjust if you like.
activitytimer = timerinit()

    async "server" {

            if timerdiff(activitytimer) > 10000 {
                sleep(throttleMs)
            }

    }
kaas = "1"
kont = "1"

kont = match kont {
    "11" | "2" => print("first","blue")
    "1" | "123" | kaas => print("second","green")
    _ => print("third")

}
print(kont,"red")
