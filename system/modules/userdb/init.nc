class userdb : module{ 
    func create(username,email,password1,password2){
        if password1 == "" or username = "" {
            return "password and userfield cannot be empty"
        }
        if password1 != password2 {
            return "password mismatch"
        }
        userconfig = combine(self.db,username,"/config.njh")
        if fileexists(userconfig) = 1 {
            return "user already exists"
        }
        
        dircreate(combine(self.db,username))
        save("#name",username,userconfig)
        save("#email",email,userconfig)
        save("#password",password1,userconfig)
        save("#role","user",userconfig)
        save("#ip",userip,userconfig)
        return "created"
        
    }
    func login(username,password){
        if username == "" or password == "" {
            return "Username/password error"
        }
        userconfig = combine(self.db,username,"/config.njh")
        if fileexists(userconfig) == 0 {
            return "Unknown user"
        }
        if password != load("#password",userconfig){
            return "password incorrect"
        }

        return "logged in"
       
    }
    func init(){
        if listdir(self.db) == ""{
            print(combine("No UserDB found creating new: ",dircreate(self.db)),"y")
            username = terminalinput("Enter new admin account name:","Nscript")
            email = terminalinput("Enter new admin account email:","admin@nscript.nc")
            password = terminalinput("Enter new admin account password","Nscript")
            userconfig = combine(self.db,username,"/config.njh")
            dircreate(combine(self.db,username))
            save("#name",username,userconfig)
            save("#email",email,userconfig)
            save("#password",password,userconfig)
            save("#role","admin",userconfig)
            save("#ip","127.0.0.1",userconfig)
            print("New admin role created !","g")
        }
    }
    self.db = combine(@scriptdir,"USER_DB/")
}
