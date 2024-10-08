FROM ubuntu:latest
RUN apt update -y
RUN apt install -y curl git build-essential libasound2-dev libjack-jackd2-dev pkg-config
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /rustup.sh
RUN cat /rustup.sh | sh -s -- --default-toolchain stable -y
