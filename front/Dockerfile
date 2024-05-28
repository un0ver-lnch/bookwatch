FROM nixos/nix

RUN nix-env -iA devenv -f https://github.com/NixOS/nixpkgs/tarball/nixpkgs-unstable

COPY . .

RUN [ -f devenv.local.nix ] && rm devenv.local.nix
RUN devenv test && devenv shell bun run build

ENTRYPOINT ["devenv", "shell"]
CMD ["bun", "run", "start"]