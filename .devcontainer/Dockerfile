FROM public.ecr.aws/lambda/provided:al2023 as builder
WORKDIR /usr/src/app

RUN dnf -y install gcc
RUN dnf -y install procps
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
RUN source $HOME/.cargo/env

ENV PATH=$PATH:/root/.cargo/bin/
RUN cargo install grcov
RUN rustup component add rustfmt
RUN rustup component add llvm-tools-preview

ENTRYPOINT "/bin/bash"