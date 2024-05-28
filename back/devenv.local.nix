{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.DB_USER = "fun";
  env.DB_PASSWORD = "fun";
  env.DB_HOST = "localhost";

  services.mongodb.enable = true;
  services.mongodb.initDatabaseUsername = "fun";
  services.mongodb.initDatabasePassword = "fun";
}
