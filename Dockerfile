FROM alpine:latest

COPY ./target/x86_64-unknown-linux-musl/release/mandelbrot /mandelbrot
COPY ./static /static/

ENV ROCKET_ADDRESS 0.0.0.0
CMD ["/mandelbrot", "serve"]
