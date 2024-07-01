{ pkgs, lib, config, inputs, ... }:

{
  name = "bookwatch-front";
  # https://devenv.sh/basics/

  # https://devenv.sh/packages/
  packages = [  ];

  # https://devenv.sh/scripts/

  enterShell = ''
  '';

  # https://devenv.sh/tests/
  enterTest = ''
  '';

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/languages/
  # languages.nix.enable = true;
  languages.javascript = {
    enable = true;
    bun = {
      enable = true;
      install.enable = true;
    };
  };

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
 containers."prod".startupCommand = "bun run start";
 containers."prod".entrypoint = ["bun" "run" "start"];
}
