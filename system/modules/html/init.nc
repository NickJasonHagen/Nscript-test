class html : module{
    func init(){
        self.login = fread(combine(self.modulepath,"/raw/login.html"))
    }
    func video(path,width,height,toreturn){
        toreturn = replace(self.video,"#PATH#",path)
        toreturn = replace(toreturn,"#W#",width)
        toreturn = replace(toreturn,"#H#",height)
        
        match print(fromleft(splitselect(path,".",1),3),"br"){
            "mkv" =>{
                toreturn = replace(toreturn,"#VIDEOTAG#","video/x-matroska")
            }
            "mp4" =>{
                toreturn = replace(toreturn,"#VIDEOTAG#","video/mp4")
            }
            "ogg" =>{
                toreturn = replace(toreturn,"#VIDEOTAG#","video/ogg")
            }
            _ =>{
                toreturn = replace(toreturn,"#VIDEOTAG#","video/mp4")
            }
        }
        return print(toreturn,"g")
    }
    func audio(path){
        return replace(self.audio,"#PATH#",path)
    }
    func youtube(){
        return print(self.youtube)
    }
    self.audio = "<audio controls><source src=\"#PATH#\" type=\"audio/mpeg\">Your browser does not support the audio element.</audio>"
    self.video = "<video width=\"#W#\" height=\"#H#\" controls><source src=\"#PATH#\" type=\"#VIDEOTAG#\">Your browser does not support the video tag.</video>"
    self.youtube = combine("<iframe width=\"560\" height=\"315\" src=\"https:/","/www.youtube.com/embed/4xnkjhW6LhY\" title=\"YouTube video player\" frameborder=\"0\" allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share\" allowfullscreen></iframe>")
    self.required = "userdb"
}