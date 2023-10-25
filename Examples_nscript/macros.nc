//macros begin with a @ they are build in functions to return a value.


//special characters
@lf = a \n from a file linefeed.

//directory
@scriptdir

//Time
@year
@month
@day
@hour
@min
@sec
@msec

//system
@OS

//web-server ( these are set whenever some one triggers a .nc script on your webserver)
@socketip       // returns the IP adres of the connecting client.
//if using domains these be availible , are set when the user enters the http server based on his using domain
@webroot        // this refurs to ./domains/yourdomainname/
@webprivate     //this refurs to ./domains/yourdomainname/private/
@webpublic      //this refurs to ./domains/yourdomainname/public/
