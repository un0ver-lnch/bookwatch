FROM nixos/nix

RUN nix-env -iA devenv -f https://github.com/NixOS/nixpkgs/tarball/nixpkgs-unstable

COPY . .

RUN devenv test && devenv shell cargo b --release

ENTRYPOINT ["devenv", "shell"]
CMD ["./target/release/back"]