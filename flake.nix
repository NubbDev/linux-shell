{
  description = "A simple desktop manager for my NixOS, inspired by user's of quickshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    configs.url = "path:/configs/nixos";
    configs.inputs.nixpkgs.follows = "nixpkgs";   # keep nixpkgs unified
  };

  outputs = { self, nixpkgs, configs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    rustShell = configs.devShells.${system}.rust;
  in {
    devShells.${system}.default = with pkgs; mkShell rec {
      # Inherit everything from the global rust shell
      inputsFrom = [ rustShell ];
      buildInputs = [
        gcc
        libxkbcommon
        wayland
        wayland-protocols
        libGL
        dbus
        libpulseaudio
        vulkan-loader
      ];
      shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.vulkan-loader
        ]}"
      '';
    };


  };
}
