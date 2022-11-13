execute binary in container

rustup target add x86_64-unknown-linux-musl

cargo build --target x86_64-unknown-linux-musl

sudo docker-compose up --build -d