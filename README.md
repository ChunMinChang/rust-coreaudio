# Rust CoreAudio Draft

This is a draft to play CoreAudio APIs in *Rust*. The aim for this project is to
1. practice *Rust*
2. get familiar with integrating *C* APIs into *Rust*
3. find a way to make unsafe block safer
4. catch problems in the implementation that is written with *C/C++* style
5. test the APIs we implemented in parallel threads, by running ```cargo tests```

## Problems

### Writing shared state at the same time
If ```set_default_device``` in *utils* module is called by two different threads, their result is undefined. Two threads try writing something on a shared state at the same time. See details on the comment above ```test_set_default_device_with_same_device``` in *utils/test.rs*.

The underlying system API of ```set_default_device``` is ```AudioObjectSetPropertyData```. We should prevent it from being called with the same ```AudioObjectID``` at the same time. In the above case, the ```set_default_device``` will call ```AudioObjectSetPropertyData(kAudioObjectSystemObject, ...)```, so the device setting for ```kAudioObjectSystemObject``` will be changed at the same time by two threads and lead to a random result.

I guess *mutable borrow* or *singleton* pattern may be a solution.

## TO-DO
- Try using *static mutable borrow* in different threads
- Know hot to write *singleton* pattern in *Rust*
