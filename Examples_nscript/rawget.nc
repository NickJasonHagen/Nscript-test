// raw get gets the response of a get to a unsecure http webhost


curlvar = rawget("192.168.1.66:8088","nscript_arrays.rs")

print(combine("curlvar:",curlvar),"blue")


