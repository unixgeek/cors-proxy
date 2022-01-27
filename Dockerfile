FROM debian:bullseye-slim as builder

WORKDIR /root

COPY src /root/src
COPY Cargo.toml /root
COPY Cargo.lock /root
COPY wrangler.toml /root

SHELL ["/bin/bash", "-c"]

RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates gcc-multilib \
    && curl --silent --show-error https://sh.rustup.rs | /bin/sh -s -- -y \
    && curl --silent --show-error https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | /bin/bash \
    && source /root/.bashrc \
    && curl --silent --show-error https://rustwasm.github.io/wasm-pack/installer/init.sh | /bin/sh \
    && rustup target add wasm32-unknown-unknown \
    && nvm install --lts \
    && npm install -g miniflare \
    && cargo install worker-build \
    && worker-build --release \
    && sed -i 's/command =.*$/command="true"/' wrangler.toml

FROM debian:bullseye-slim

RUN groupadd -r nonroot \
    && useradd -m -r -g nonroot nonroot

COPY docker-entrypoint.sh /home/nonroot/

COPY .mf /home/nonroot/.mf
COPY --from=builder /root/build /home/nonroot/build
COPY --from=builder /root/wrangler.toml /home/nonroot/wrangler.toml
COPY --from=builder /root/.nvm /home/nonroot/.nvm

EXPOSE 8787
USER nonroot
ENTRYPOINT ["/bin/bash", "/home/nonroot/docker-entrypoint.sh"]