// loop { break} is a scope wich will run until you say break. 
// unlike breaking a async with a reference this loop only needs 1 word

i = 0 
loop {
    i ++
    if i > 10 {
        break    
    }
}
