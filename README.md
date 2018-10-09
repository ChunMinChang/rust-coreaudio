# Rust CoreAudio

This is a draft to play CoreAudio APIs in *Rust*. The aim for this project is to
1. practice *Rust*
2. get familiar with calling *C* APIs in *Rust*
3. find a way to make unsafe block safer
4. test the APIs in parallel threads, by running ```cargo tests```
5. get ideas about how to design interfaces for an audio library

## How to run
Make sure *Rust* and *Cargo* are installed.
- build progect: ```$ cargo build```
- Examples
  - Play sine wave: ```$ cargo run --example sine```
  - Show devices info: ```$ cargo run --example devices```

## TO-DO
- Use **single-element** (tuple) struct to wrap all native types(e.g., ```CFStringRef```, ```AudioObjectID```).
  - references
    - [string][gist-string-wrapper]
    - [directly get inner element data from C API][gist-same-size](since their sizes are same!)
    - [CoreAudio][gist-audioobject]
- **Tests**
  - Unit test for *audio_unit* and *stream* modules.
  - Integration test for using *stream* and *utils(devices)* at the same time.

## References
- [RustAudio/coreaudio-rs][RustAudio-coreaudio-rs]
- [djg/core-audio-rs][djg-core-audio-rs]
- [cubeb-coreaudio-rs][cubeb-coreaudio-rs]

[gist-string-wrapper]: https://gist.github.com/ChunMinChang/25f3608c285f1abf2a5c289d5f758427 "Using tuple struct to wrap native C types"
[gist-same-size]: https://gist.github.com/ChunMinChang/1acf672babd4e8f79fcf83fa228d1461 "Wrap native types by tuple struct"
[gist-audioobject]: https://gist.github.com/ChunMinChang/07b806cb6a9ea1136cb3cbd8cda6c806 "Access data from CoreAudio APIs with a single-element tuple structs wrapping native CoreAudio types"
[gist-callback]: https://gist.github.com/ChunMinChang/8a22f8a1308b6e0a600e22c4629b2175 "Convert a void* buffer (from C) to a typed slice"

[cubeb-coreaudio-rs]: https://github.com/ChunMinChang/cubeb-coreaudio-rs "C-style Rust code for audiounit backend"
[RustAudio-coreaudio-rs]: https://github.com/RustAudio/coreaudio-rs "RustAudio/coreaudio-rs"
[djg-core-audio-rs]: https://github.com/djg/core-audio-rs "djg/core-audio-rs"