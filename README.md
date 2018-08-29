# Rust CoreAudio Draft

This is a draft to play CoreAudio APIs in *Rust*. The aim for this project is to
1. practice *Rust*
2. get familiar with integrating *C* APIs into *Rust*
3. find a way to make unsafe block safer
4. catch problems in the implementation that is written with *C/C++* style
5. test the APIs we implemented in parallel threads, by running ```cargo tests```

## TO-DO
- Use *tuple struct* to wrap all native types(e.g., ```CFStringRef```, ```AudioObjectID```).
- Cannot play sounds via DisplayPort. Find out why.
- Redesign callback mechanism: Save a *callback* member variable in ```Stream``` and call it to fill buffer.