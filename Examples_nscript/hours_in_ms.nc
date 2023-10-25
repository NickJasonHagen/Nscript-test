// this function sets a timestamp in miniseconds in a format wich would always be the same lenght,
// is can be combed with timerdiff(var) to get the time elapased in miliseconds

timer = timerinit()

loop {
    if timerdiff(timer) > 999 {
        exit
    }
}

//special functions

if timerdiff(timer) > minutes_in_ms(2) {
}
if timerdiff(timer) > hours_in_ms(2) {
}
if timerdiff(timer) > weeks_in_ms(2) {
}
if timerdiff(timer) > months_in_ms(2) {
}

 
