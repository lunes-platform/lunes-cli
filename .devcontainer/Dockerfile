FROM rust

RUN apt update -y
RUN apt install zsh git -y
RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# builder cross compile

# linux
RUN rustup target add x86_64-unknown-linux-musl
RUN apt install gcc-x86-64-linux-gnu -y

# windows
RUN rustup target add x86_64-pc-windows-gnu
RUN apt-get install gcc-mingw-w64-x86-64 -y