{
  inputs = {
    # github example, also supported gitlab:
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    {
      overlay = final: prev: {
        susbot = self.packages.${prev.system}.default;
      };
    }
    // flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;
        susbot = {
          rustPlatform,
          pkg-config,
          openssl,
        }:
          rustPlatform.buildRustPackage {
            name = "susbot";
            src = lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [
              pkg-config
              rustPlatform.bindgenHook
            ];
            buildInputs = [openssl];
            meta = with lib; {
              license = licenses.mpl20;
              homepage = "https://github.com/Sciencentistguy/susbot";
              platforms = platforms.all;
            };
          };
      in {
        packages.susbot = pkgs.callPackage susbot {};

        packages.default = self.packages.${system}.susbot;

        devShells.default = self.packages.${system}.default.overrideAttrs (super: {
          nativeBuildInputs = with pkgs;
            super.nativeBuildInputs
            ++ [
              cargo-edit
              clippy
              rustfmt
            ];
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        });
        nixosModules.susbot = {
          pkgs,
          config,
          lib,
          ...
        }: let
          inherit (lib) mkOption mkIf mkEnableOption types;
          cfg = config.services.susbot;
        in {
          options = {
            services.susbot = {
              enable = mkEnableOption "susbot";
              package = mkOption {
                type = types.package;
                default = self.packages.${system}.default;
                defaultText = "pkgs.susbot";
                description = "The package to use for the susbot service.";
              };
              tokenPath = mkOption {
                example = "/run/secrets/susbot_appid";
                type = types.str;
              };
              appIdPath = mkOption {
                example = "/run/secrets/susbot_appid";
                type = types.str;
              };
            };
          };

          config = mkIf cfg.enable {
            users = {
              users.susbot = {
                group = "susbot";
                description = "susbot user";
                isSystemUser = true;
              };
              groups.susbot = {};
            };

            systemd.services.susbot = {
              description = "susbot";
              wantedBy = ["multi-user.target"];
              after = ["network.target"];
              serviceConfig = {
                ExecStart = "${cfg.package}/bin/susbot ${cfg.tokenPath} ${cfg.appIdPath}";
                User = "susbot";
                Group = "susbot";
                Restart = "always";
                RestartSec = 5;
              };
            };
          };
        };
      }
    );
}
