FROM alpine:3

COPY target/x86_64-unknown-linux-musl/release/kyd-api-bridge /usr/local/bin

CMD [ "kyd-api-bridge" ]