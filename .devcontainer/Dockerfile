# ===================== Stage 1: toolchain ======================
FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04 AS toolchain
ARG RUST_TOOLCHAIN=1.87.0

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        curl ca-certificates build-essential clang mold pkg-config \
        libssl-dev libclang-dev iproute2 iputils-ping wireguard-tools \
        git nftables; \
    rm -rf /var/lib/apt/lists/*

# -- Rust toolchain (root‑owned, read‑only at runtime) --
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y \
        --profile minimal --default-toolchain ${RUST_TOOLCHAIN} && \
    rustup component add clippy rustfmt --toolchain ${RUST_TOOLCHAIN}

RUN update-alternatives --install /usr/bin/ld ld /usr/bin/mold 50
RUN tar -C /usr/local -czf /toolchain.tgz rustup cargo

# ===================== Stage 2: runtime ======================
FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        clang mold libssl-dev libclang-dev kmod \
        iproute2 iputils-ping wireguard-tools \
        git nftables lsb-release sudo; \
    rm -rf /var/lib/apt/lists/*

ARG USERNAME=fukuro
# -- non‑root user --
RUN useradd -m -s /bin/bash ${USERNAME} && \
    echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# -- toolchain (read‑only) --
ENV RUSTUP_HOME=/usr/local/rustup \
    PATH=/usr/local/cargo/bin:$PATH

COPY --from=toolchain /toolchain.tgz /tmp/
RUN tar -C /usr/local -xzf /tmp/toolchain.tgz && rm /tmp/toolchain.tgz
RUN update-alternatives --install /usr/bin/ld ld /usr/bin/mold 50

# ───── ここから一般ユーザ領域 ─────
USER ${USERNAME}
WORKDIR /workspaces

# ★ ユーザ専用 CARGO_HOME ★
ENV CARGO_HOME=/home/${USERNAME}/.cargo
RUN mkdir -p $CARGO_HOME && \
    echo 'export PATH=$CARGO_HOME/bin:$PATH' >> ~/.bashrc

# 初回アクセスで index/ cache がユーザ所有で生成される
RUN cargo install boringtun-cli
