FROM ghcr.io/cachix/devenv:latest

COPY . .

RUN [ -f devenv.local.nix ] && rm devenv.local.nix
RUN devenv test && devenv shell cargo b -- --release

ENTRYPOINT ["devenv", "shell"]
CMD ["./target/release/back"]