{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
      in
      {
        packages.default = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;

          buildInputs = with pkgs; [
            wayland
            libxkbcommon
            xkeyboard-config
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          # For GPU rendering (pixels needs this)
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.libGL
            pkgs.libxkbcommon
            pkgs.wayland
          ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            wayland
            libxkbcommon
            xkeyboard-config # Add this
            libGL
          ];
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.libGL
            pkgs.libxkbcommon
            pkgs.wayland
          ];
          XKB_CONFIG_ROOT = "${pkgs.xkeyboard-config}/share/X11/xkb";
        };
      }
    );
}
