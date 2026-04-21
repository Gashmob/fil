{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix/28b19c5844cc6e2257801d43f2772a4b4c050a1b";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # <https://github.com/nix-systems/nix-systems>
    systems.url = "github:nix-systems/default-linux";
  };

  outputs =
    {
      nixpkgs,
      treefmt-nix,
      systems,
      ...
    }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
      pkgs = eachSystem (system: import nixpkgs { inherit system; });

      fil-version = (fromTOML (builtins.readFile ./Cargo.toml)).package.version;

      fil-package = eachSystem (
        system:
        pkgs.${system}.callPackage ./fil.nix {
          libllvm = pkgs.${system}.llvmPackages_22.libllvm;
        }
      );
      rpm-package = eachSystem (
        system:
        pkgs.${system}.callPackage ./tools/package/rpm.nix {
          fil = fil-package.${system};
          fil-version = fil-version;
        }
      );
      deb-package = eachSystem (
        system:
        pkgs.${system}.callPackage ./tools/package/deb.nix {
          fil = fil-package.${system};
          fil-version = fil-version;
        }
      );
    in
    {
      packages = eachSystem (system: {
        fil = fil-package.${system};
        default = fil-package.${system};

        rpm = rpm-package.${system};
        deb = deb-package.${system};
      });

      devShells = eachSystem (system: {
        default = pkgs.${system}.mkShell {
          name = "fil-development-environment";

          packages = with pkgs.${system}; [
            git
            rustup
            llvmPackages_22.libllvm
            libffi
            libxml2
            (import ./tools/nix/treefmt.nix {
              inherit treefmt-nix;
              pkgs = pkgs.${system};
            })
          ];

          LD_LIBRARY_PATH =
            with pkgs.${system};
            lib.makeLibraryPath [
              libffi
              stdenv.cc.cc
            ];

          shellHook = ''
            export ROOT_DIR=$(git rev-parse --show-toplevel)
            export PATH="$PATH:$ROOT_DIR/tools/bin"

            cp "$ROOT_DIR/tools/githooks/pre-commit.sh" "$ROOT_DIR/.git/hooks/pre-commit"
            git config commit.template "$ROOT_DIR/commit-template"
          '';
        };
      });
    };
}
