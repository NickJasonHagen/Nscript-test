class system_modules{
    func load(){
        scripts = listdir(combine(@scriptdir,"system/modules/"))
        for script in scripts{
            includescriptroot = combine(@scriptdir,"system/modules/",script,"/")
            exec(combine(@scriptdir,"system/modules/",script,"/init.nc"))
        }

        for module in system_modules.initialized{
            if module != ""{
               *module.checkrequired() 
            }
            
        }
    }
    self.initialized = ""
}

class module{
    func construct(){
        self.module = self
        self.modulepath = includescriptroot
        system_modules.initialized = pooladd(system_modules.initialized,self)
        print(combine("system/modules: ",self.module))
    }
    func checkrequired(){
        for required in self.required {
            if inpool(system_modules.initialized,required) = 0 and required != ""{
                print(combine("SystemModule error! module:",self," requires module: ",required)
            }
            else {
                self.init()
                print(combine("Loadingmodule:",self," success!"))
            }
        }
    }
    func init(){
        //implement this function to execute after everything is loaded in properly
        // each module will declare this function it will run after all is loaded.
    }
    self.required = self
}

class server {
    func onconnect(clientIP){
        userip = combine("ip:",clientIP)
        activitytimer = timerinit()
        makenew = ""
        getlist = ""
        url = ""
       }

    server.debugmode = debugmode( 1 )
    server.restrictedmode = restrictionmode(0)
    print("ServerScope Executed!!" ,"red" )
    self.ip = "0.0.0.0"
    self.port = 8088
    self.POSTbytesmax = 1024 * 20// post datamax size
    userip = "unset"//set default 
}

throttleMs = 50
// throttle the server so the powerusage drops. adjust if you like.
activitytimer = timerinit()
serv = "server"
    coroutine serv {
        if timerdiff(activitytimer) > 10000 {
            sleep(throttleMs)
        }

    }
system_modules.load()

