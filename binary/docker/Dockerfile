FROM scratch

COPY ./target/x86_64-unknown-linux-musl/release/binary /binary
COPY .env /.env
CMD [ "/binary" ]