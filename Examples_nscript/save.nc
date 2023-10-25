// these are database like system to quickly store or load data to a file ( textformat )
// it saves the header on a line , and the line bellow it contains the data.
// headers must be unique per file, or it overwrites.

save("#header","data","filelocation")
data = load("#header","filelocation")
