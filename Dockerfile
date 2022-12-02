FROM rust:1.65.0-slim-bullseye as builder

WORKDIR /root

COPY src /root/src
COPY Cargo.toml /root
COPY Cargo.lock /root
COPY wrangler.toml /root

RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates build-essential libssl-dev pkg-config \
    && cargo install wasm-pack@0.10.3 \
    && cargo install worker-build@0.0.8 \
    && sed -i 's/command =.*$/command="true"/' wrangler.toml \
    && worker-build --release \
    && mkdir /root/node \
    && curl --silent --show-error https://nodejs.org/download/release/v16.18.1/node-v16.18.1-linux-x64.tar.xz \
       | tar --strip-components=1 -x -J -C /root/node -f -

FROM debian:bullseye-20221114-slim

RUN groupadd -r nonroot \
    && useradd -m -r -g nonroot nonroot

COPY --chown=nonroot docker-entrypoint.sh package*.json /home/nonroot/
COPY --chown=nonroot .wrangler/ /home/nonroot/.wrangler/
COPY --from=builder --chown=nonroot /root/build/ /home/nonroot/build/
COPY --from=builder --chown=nonroot /root/wrangler.toml /home/nonroot/
COPY --from=builder --chown=nonroot /root/node/ /home/nonroot/node/

USER nonroot
WORKDIR /home/nonroot
env PATH=$PATH:/home/nonroot/node/bin

RUN npm install

EXPOSE 8787
ENTRYPOINT ["/home/nonroot/docker-entrypoint.sh"]
