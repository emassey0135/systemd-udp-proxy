{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-24.11";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };
  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem(system:
      let pkgs = import nixpkgs {
        inherit system;
      };
        systemd-udp-proxy = (with pkgs;
          rustPlatform.buildRustPackage rec {
            pname = "systemd-udp-proxy";
            version = "0.1.2";
            cargoLock.lockFile = ./Cargo.lock;
            src = lib.cleanSource ./.;
          }
        );
      in rec {
        defaultApp = flake-utils.lib.mkApp {
          drv = defaultPackage;
        };
        defaultPackage = systemd-udp-proxy;
      }
    );
}
