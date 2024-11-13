FROM nixos/nix

ARG NIX_INSTALLER_EXTRA_CONF
ENV NIX_INSTALLER_EXTRA_CONF=${NIX_INSTALLER_EXTRA_CONF}

RUN nix-env --option sandbox false -iA devenv -f https://github.com/NixOS/nixpkgs/tarball/nixpkgs-unstable

COPY . .

RUN [ -f devenv.local.nix ] && rm devenv.local.nix
RUN devenv test && devenv shell bun run build

ENTRYPOINT ["devenv", "shell"]
CMD ["bun", "run", "start"]