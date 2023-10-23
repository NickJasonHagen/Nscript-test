class scripteditor : includes{
    func init(){
        self.nscripts = ""
        for x in listdir(combine(@scriptdir,"domains/")){
            self.nscripts = combine self.nscripts @lf combine(@scriptdir,"domains/",x,"/http.nc")
            for x2 in arraysearch(listdir(combine(@scriptdir,"domains/",x,"/public/")),".nc"){
                self.nscripts = combine self.nscripts @lf combine(@scriptdir,"domains/",x,"/public/",x2)
            }
        }
        print(self.nscripts,"red")
    }
    func webentree(){
        return "testok"
    }
    func systemls(){

    }
    //print(self.nscripts,"red")

}