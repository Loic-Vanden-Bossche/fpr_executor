FROM rust:1.69 as rust-base

COPY src src/
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

FROM gcr.io/distroless/cc:debug

COPY --from=rust-base /target/release/fpr-executor /usr/local/bin/

COPY --from=rust-base /bin/echo /bin/echo
COPY --from=rust-base /bin/rm /bin/rm
COPY --from=rust-base /bin/sh /bin/sh

RUN echo "executor:x:1000:executor" >> /etc/group
RUN echo "executor:x:1001:" >> /etc/group
RUN echo "executor:x:1000:1001::/home/executor:" >> /etc/passwd

RUN rm /bin/sh /bin/echo /bin/rm

USER executor