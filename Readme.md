Mandelbrot
=======

A lightweight microservice that allows you to explore the mandelbrot fractal google maps-style!

## Dependencies

The main build artifact of this project is a web server (a binary) inside an alpine container.  
Because alpine uses musl, we need to cross compile for `x86_64-unknown-linux-musl`.

Cross compiling is complicated.  
It can be hard to set up and works differently in different operating systems.  
We make it easy with docker: the only dependency is a working docker and make.

## Running

Running is simple: just `make run`, or just `make`.
This command will:

1. build the container in which we will cross-compile the executable
1. cross-compile the binary in the aforementioned container
2. build the alpine container
3. run the alpine container and attach it to your tty

The server will run at localhost:8000.
To stop the process, `^C` will work fine.
If you just want to build the container, try `make build`.

**NOTE**: compilation will start a docker container, but not end it.
This way cargo won't have to re-download sources for all the cargo dependencies every time we compile.
The ownside is that the build container, named `mandelbrot-build`, will be left running after compilation.
When you're done working on the project, you can run `docker stop mandelbrot-build` to stop and remove the container.

## Feedback and Bugs

If you have any comments or would like to contribute, please submit an issue.
I'm not actively developing this but there are a couple of improvements I have in mind that I might get to soon.

Currently if you zoom in far enough, things start to get blurry.
Unfortunately higher precision floating point arithmetic currently seems to be too slow.
There are potential algorithmic solutions, but I haven't had the time to expore them fully.
If you have any ideas I'd love to hear them.

## License

```
The MIT License (MIT)
Copyright © 2017 Cameron Derwin

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```
