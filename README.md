# Tangled Game

## Introduction

The goal of this project is to create a simple game using both Ada and Rust for comparison. In this game the player must untangle the balls to advance to the next level. The game uses Raylib and is written using both Ada and Rust in seperate implementations.

## Building

### Ada version

#### Prerequisites

Install Alire [https://alire.ada.dev](https://alire.ada.dev)

#### Build

Enter the the commands:

    # cd ada
    # alr build
    # cp bin/tangled ../tangled-ada
    
### Rust version

#### Prerequisites

Install Cargo [https://sh.rustup.rs](https://sh.rustup.rs)

#### Build

Enter the commands:

    # cd rust
    # cargo build
    # cp target/debug/tangled ../tangled-rust

## License

Copyright (c) 2024 N

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

## References

1. Raylib [https://www.raylib.com](https://www.raylib.com)

2. Learn Ada [https://learn.adacore.com](https://learn.adacore.com)

3. Learn Rust [https://www.rust-lang.org/learn](https://www.rust-lang.org/learn)
