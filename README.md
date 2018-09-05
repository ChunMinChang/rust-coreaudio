# Rust CoreAudio Draft

This is a draft to play CoreAudio APIs in *Rust*. The aim for this project is to
1. practice *Rust*
2. get familiar with integrating *C* APIs into *Rust*
3. find a way to make unsafe block safer
4. catch problems in the implementation that is written with *C/C++* style
5. test the APIs we implemented in parallel threads, by running ```cargo tests```

## TO-DO
- Use **single-element** (tuple) struct to wrap all native types(e.g., ```CFStringRef```, ```AudioObjectID```).
  - references
    - [string][gist-string-wrapper]
    - [directly get inner element data from C API][gist-same-size](since their sizes are same!)
    - [CoreAudio][gist-audioobject]
- Redesign callback mechanism: Save a *callback* member variable in ```Stream``` and call it to fill buffer.
  - [reference][gist-callback]
- Use *examples* to separate the example code for *stream* module and *utils* module.
- **Add tests** for *audio_unit* and *stream* modules.

[gist-string-wrapper]: https://gist.github.com/ChunMinChang/25f3608c285f1abf2a5c289d5f758427 "Using tuple struct to wrap native C types"
[gist-same-size]: https://gist.github.com/ChunMinChang/1acf672babd4e8f79fcf83fa228d1461 "Wrap native types by tuple struct"
[gist-audioobject]: https://gist.github.com/ChunMinChang/07b806cb6a9ea1136cb3cbd8cda6c806 "Access data from CoreAudio APIs with a single-element tuple structs wrapping native CoreAudio types"
[gist-callback]: https://gist.github.com/ChunMinChang/8a22f8a1308b6e0a600e22c4629b2175 "Convert a void* buffer (from C) to a typed slice"