FROM messense/rust-musl-cross:x86_64-musl AS builer
COPY . /home/rust/src
RUN cargo build --release
RUN musl-strip /home/rust/src/target/x86_64-unknown-linux-musl/release/thangail

FROM scratch
COPY --from=builer /home/rust/src/target/x86_64-unknown-linux-musl/release/thangail /thangail
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80
CMD ["/thangail"]
