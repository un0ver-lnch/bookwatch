{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.DB_USER = "fun";
  env.DB_PASSWORD = "fun";
  env.DB_HOST = "localhost";

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
  services.mongodb.enable = true;
  services.mongodb.initDatabaseUsername = "fun";
  services.mongodb.initDatabasePassword = "fun";

  # https://devenv.sh/languages/
  # languages.nix.enable = true;
  languages.rust.enable = true;

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
