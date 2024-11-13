FROM ghcr.io/cachix/devenv:latest

COPY . .

RUN [ -f devenv.local.nix ] && rm devenv.local.nix
RUN devenv test && devenv shell bun run build

ENTRYPOINT ["devenv", "shell"]
CMD ["bun", "run", "start"]