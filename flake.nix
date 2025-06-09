{
  description = "Flake for rofi-snippets";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs@{ self, nixpkgs, ... }: let
    inherit (nixpkgs) lib;
    systems = [ "x86_64-linux" "aarch64-linux" ];
    forAllSystems = f: lib.genAttrs systems (system: f system);
  in {
    devShells = forAllSystems (system: let
      pkgs = import nixpkgs { inherit system; };
    in {
      default = pkgs.mkShell {
        packages = with pkgs; [
		      pkg-config
          glib
          cairo
          pango
          xdotool
          libxkbcommon
        ];
      };
    });

    apps = forAllSystems (system: let
      pkgs = import nixpkgs { inherit system; };
    in {
      release = {
        type = "app";
        program =  lib.getExe (pkgs.writeShellScriptBin "release" ''
          set -e

          ver=''${1:-$(git cliff --bumped-version)}
          ver=''${ver##v}

          sed -i "/name = \"rofi-snippets\"/{n;s/version = \".*\"/version = \"$ver\"/g}" Cargo.toml Cargo.lock
          git cliff --bump -o CHANGELOG.md
          git add -A
          git commit -m "chore(release): v$ver"
          git tag "v$ver"
        '');
      };
      test = {
        type = "app";
        program = lib.getExe (pkgs.writeShellScriptBin "test" ''
          set -eu

          ROFI_PLUGIN_PATH=target/debug rofi \
            -modes run,rofi-snippets \
            -show rofi-snippets
        '');
      };
    });
  };
}
