FROM debian:latest

RUN apt-get update && \
    apt-get upgrade && \
    apt-get install -y \
        build-essential \
        git \
        curl \
        python

RUN mkdir -p /usr/local/src && \
    curl https://static.rust-lang.org/dist/rustc-1.6.0-src.tar.gz -o /usr/local/src/rustc-1.6.0-src.tar.gz && \
    tar xzf /usr/local/src/rustc-1.6.0-src.tar.gz -C /usr/local/src && \
    rm /usr/local/src/rustc-1.6.0-src.tar.gz && \
    cd /usr/local/src/rustc-1.6.0 && \
    ./configure --prefix=/usr/local/src && \
    make && make install && \
    cd / && rm -rf /usr/local/src/rust

RUN curl https://github.com/rust-lang/cargo/archive/0.8.0.tar.gz -o /usr/local/src/cargo-0.8.0.tar.gz && \
    tar xzf /usr/local/src/cargo-0.8.0.tar.gz -C /usr/local/src && \
    rm /usr/local/src/caro-0.8.0.tar.gz && \
    cd /usr/local/src/cargo && \
    ./configure --prefix=/usr/local && \
    make && make install && \
    cd / && rm -rf /usr/local/src/cargo
