{ pkgs, treefmt-nix }:
treefmt-nix.lib.mkWrapper pkgs {
  programs.actionlint.enable = true;
  programs.beautysh.enable = true;
  programs.nixfmt.enable = true;
  programs.nixf-diagnose.enable = true;
  programs.rustfmt.enable = true;
}
