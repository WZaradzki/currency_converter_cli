ARG RUST_VERSION=1.75.0
ARG APP_NAME=currency_converter_cli

################################################################################
# xx is a helper for cross-compilation.
# See https://github.com/tonistiigi/xx/ for more information.
FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.3.0 AS xx

################################################################################
# Create a stage for building the application.
# Note: There's no official Rust image based on Ubuntu, so we start with a base Ubuntu image and install Rust manually.
FROM --platform=$BUILDPLATFORM ubuntu:20.04 AS build
ARG APP_NAME
ARG RUST_VERSION
WORKDIR /app

ENV TZ=Europe/Warsaw
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Copy cross compilation utilities from the xx stage.
COPY --from=xx / /

# Install system dependencies.
RUN apt-get update && apt-get install -y curl build-essential clang lld libssl-dev pkg-config git file && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION} && \
    rm -rf /var/lib/apt/lists/*

# Add Rust to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# This is the architecture you’re building for, which is passed in by the builder.
ARG TARGETPLATFORM


RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    # --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    bash -c "set -e && \
    cargo build --locked --release --target-dir ./target && \
    cp ./target/$(rustc --print target-triple)/release/$APP_NAME /bin/cli && \
    file /bin/cli"  # Replaced xx-cargo and xx-verify with direct cargo call and file command.

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application.
FROM ubuntu:20.04 AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --uid "${UID}" \
    appuser

# Change the work directory to /home/appuser (or any directory your app needs to write to)
WORKDIR /home/appuser

# Make sure the appuser owns the work directory
RUN chown appuser:appuser /home/appuser

# Switch to your appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/cli /home/appuser/

CMD ["tail", "-f", "/dev/null"]
