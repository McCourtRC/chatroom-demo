FROM ghcr.io/railwayapp/nixpacks:ubuntu-1684957838

ENTRYPOINT ["/bin/bash", "-l", "-c"]
WORKDIR /app/


COPY .nixpacks/nixpkgs-293a28df6d7ff3dec1e61e37cc4ee6e6c0fb0847.nix .nixpacks/nixpkgs-293a28df6d7ff3dec1e61e37cc4ee6e6c0fb0847.nix
RUN nix-env -if .nixpacks/nixpkgs-293a28df6d7ff3dec1e61e37cc4ee6e6c0fb0847.nix && nix-collect-garbage -d


ARG NIXPACKS_METADATA PORT ROCKET_ADDRESS
ENV NIXPACKS_METADATA=$NIXPACKS_METADATA PORT=$PORT ROCKET_ADDRESS=$ROCKET_ADDRESS

# setup phase
# noop

# build phase
COPY . /app/.
RUN --mount=type=cache,id=xYySUSrXc5U-/root/cargo/git,target=/root/.cargo/git --mount=type=cache,id=xYySUSrXc5U-/root/cargo/registry,target=/root/.cargo/registry --mount=type=cache,id=xYySUSrXc5U-target,target=/app/target cargo build --release
RUN --mount=type=cache,id=xYySUSrXc5U-/root/cargo/git,target=/root/.cargo/git --mount=type=cache,id=xYySUSrXc5U-/root/cargo/registry,target=/root/.cargo/registry --mount=type=cache,id=xYySUSrXc5U-target,target=/app/target cp ./target/release/chatroom-server $out/chatroom-server




# start
COPY . /app
CMD ["$out/chatroom-server"]
