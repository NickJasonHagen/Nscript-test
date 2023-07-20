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
self.ip = "0.0.0.0"
self.port = 8088

}


throttleMs = 50
// throttle the server so the powerusage drops. adjust if you like.
activitytimer = timerinit()

    async "server" {

            if timerdiff(activitytimer) > 10000 {
                sleep(throttleMs)
            }

    }

