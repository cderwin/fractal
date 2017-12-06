Mandelbrot
=======

A lightweight microservice that allows you to explore the mandelbrot set google maps style!

# Dependencies

To build and run the project locally as it's currently set up, you need the following: 

* a relatively recent rust compiler with cargo (anything in the last year or two *should* work)
* docker
* docker-compose
* and make

# Running

Running is simple: just `make run`, or even ust `make`.
This command will:

1. compile the program (with `cargo build --release`, otherwise it's unbearably slow)
2. build the docker container
3. start the docker container and attach it to your tty

To stop the process, `^C` will work fine.
If you just want to build the container, try `make build`.

# Feedback and Bugs

If you have any comments or would like to contribute, please feel free to submit an issue.
I'm not actively developing this but there are a couple of improvements I have in mind that I might someday get to.

Currently if you zoom in far enough, things start to get blurry.
Unfortunately higher precision floating point arithmetic currently seems to be too slow or too immature in Rust.
There are potential algorithmic solutions, but I haven't had the time to expore them fully.

# License

```
The MIT License (MIT)
Copyright © 2017 Cameron Derwin

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```
