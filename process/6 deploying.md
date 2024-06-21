# Deploying

You could use `cargo build --release --target [...]` to get an executable for a given target. While my experience is limited, I _think_ most cloud providers just let you directly run docker containers which may be preferable and is what I do for my personal projects on my own server.

## Dockerfile

While untested I believe a docker file like this should work, its taken from what I usually use.

Running it in a `FROM scrach` gives the final container a very small size it _may_ have worse performace around things like memory allocation and _may_ have issues around making web requests so proper testing should be done to compare it to other base images.

```Dockerfile
FROM rust:1.77 as builder
WORKDIR /usr/src/companies-backend
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target=x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/companies-backend /usr/local/bin/companies-backend
EXPOSE 8080
CMD ["companies-backend"]
```
