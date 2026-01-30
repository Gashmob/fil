{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";

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
    in
    {
      devShells = eachSystem (system: {
        default = pkgs.${system}.mkShellNoCC {
          name = "fil-development-environment";

          packages = with pkgs.${system}; [
            git
            (import ./tools/nix/treefmt.nix {
              inherit treefmt-nix;
              pkgs = pkgs.${system};
            })
          ];

          shellHook = ''
            export ROOT_DIR=$(git rev-parse --show-toplevel)
            export PATH="$PATH:$ROOT_DIR/tools/bin"

            git config commit.template "$ROOT_DIR/commit-template"
          '';
        };
      });
    };
}
