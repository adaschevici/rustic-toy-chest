FROM rust:latest as builder

WORKDIR /quasi-prod-rust-api
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 AS runtime
ARG ARCH=aarch64
WORKDIR /app
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libpq.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgssapi_krb5.so* /usr/lib/${ARCH}-linux-gnu/
COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libldap-2.5.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libkrb5.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libk5crypto.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libkrb5support.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/liblber-2.5.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libsasl2.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgnutls.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libp11-kit.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libidn2.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libunistring.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libtasn1.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libnettle.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libhogweed.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgmp.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libffi.so* /usr/lib/${ARCH}-linux-gnu/

COPY --from=builder /lib/${ARCH}-linux-gnu/libcom_err.so* /lib/${ARCH}-linux-gnu/

COPY --from=builder /lib/${ARCH}-linux-gnu/libkeyutils.so* /lib/${ARCH}-linux-gnu/
COPY --from=builder /quasi-prod-rust-api/config /app/config
COPY --from=builder /quasi-prod-rust-api/target/release/quasi-prod-rust-api /app/quasi-prod-rust-api
ENTRYPOINT ["/app/quasi-prod-rust-api"]
