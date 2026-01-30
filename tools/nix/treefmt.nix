{ pkgs, treefmt-nix }:
treefmt-nix.lib.mkWrapper pkgs {
  programs.actionlint.enable = true;
  programs.nixfmt.enable = true;
  programs.nixf-diagnose.enable = true;
}
