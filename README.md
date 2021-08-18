# Rust WASM example with Parcel

## Prerequisite

[Rust](https://www.rust-lang.org)

[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

[Parcel Bundler with zero config 🚀](https://parceljs.org/getting_started.html)

``` bash
parcel ./front-end/index.html
```

More info:
[WASM Pack Book](https://rustwasm.github.io/wasm-pack/book/introduction.html)

### Which files to see

Use algorithm from ueight lib writen in Rust to convert
number in range 0-255 to format 0bxxxxxxxx (8 bit with 0b in front)
> src/lib.rs
---
Here we import Rust compiled code and initialize application, we use byte_repr(byte: u8, reverse: bool) rust function here from javascript
> front-end/main.js
---
import main.js and minimal html with some basic styling
> front-end/index.html
