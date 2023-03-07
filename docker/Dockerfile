FROM ubuntu:20.04
LABEL name="talio-net-node"
LABEL github="https://github.com/SCV-chain/substrate-node"

ENV TZONE=Asia/Ha_noi
ENV LANG=C.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL=C.UTF-8
RUN ln -snf /usr/share/zoneinfo/$TZONE /etc/localtime && echo $TZONE > /etc/timezone

# Install build dependencies
RUN apt-get update -y \
  && apt-get install -y automake \
  build-essential \
  apt-utils \
  git clang curl libssl-dev llvm libudev-dev \
  curl \
  && apt-get clean

RUN mkdir -p /root/setup
WORKDIR /root/setup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

RUN cargo version
RUN rustup toolchain install nightly
RUN rustup update nightly & \
  rustup update stable &\
  rustup target add wasm32-unknown-unknown --toolchain nightly

WORKDIR /talio-net-node
ADD ./ ./
RUN cargo clean
RUN cargo build --release

WORKDIR /talio-net-node/target/release

EXPOSE 9944

