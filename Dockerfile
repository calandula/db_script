FROM ekidd/rust-musl-builder as builder
ADD --chown=rust:rust mat_view/ /mat_view
WORKDIR /mat_view
RUN cargo build --release

# Create the execution container by copying the compiled hello world to it and running it
FROM ubuntu:latest

COPY --from=builder /mat_view/target/x86_64-unknown-linux-musl/release/mat_view \
    /mat_view

ADD refresh_view.sh /root/refresh_view.sh

RUN chmod 0644 /root/refresh_view.sh
RUN apt-get update
RUN apt-get -y install cron
RUN crontab -l | { cat; echo "* * * * * bash /root/refresh_view.sh"; } | crontab -

CMD cron