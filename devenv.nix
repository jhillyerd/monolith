{ pkgs, ... }:
let
  pgHost = "127.0.01";
  pgPort = 15432;
in
{
  env.M_DATABASE_URL = "postgresql://${pgHost}:${toString pgPort}";

  # https://devenv.sh/packages/
  packages = [
    pkgs.cargo-watch
    pkgs.nomad
    pkgs.openssl_3_0.dev
    pkgs.pkg-config
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };

  services.postgres = {
    enable = true;
    listen_addresses = pgHost;
    port = pgPort;
  };
}
