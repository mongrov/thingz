FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV APP=thingz
ENV USER=mason
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY ./ .

RUN cargo build --release
RUN strip -s /app/target/release/$APP

####################################################################################################
## Final image
####################################################################################################
FROM debian:bookworm-slim

ENV APP=thingz

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/$APP ./
COPY --from=builder /app/config ./

ENV APP_ARGS="-c /app/config/default"

# Use an unprivileged user.
USER mason:mason

CMD /app/$APP $APP_ARGS mqtt

