FROM ghcr.io/actions/actions-runner:latest
RUN sudo apt update -y && \
    sudo apt install -y \
    build-essential
#    libasound-dev \
#    pkg-config
#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > $HOME/rustup.sh
#RUN cat $HOME/rustup.sh | sh -s -- --default-toolchain stable -y
#WORKDIR /src/tuning-tool
