{
  description = "A simple desktop manager for my NixOS, inspired by user's of quickshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    configs = {
      url = "path:/configs/nixos";
      inputs.nixpkgs.follows = "nixpkgs";   # keep nixpkgs unified
    };

    nixgl = {
      url = "github:nix-community/nixGL";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, configs, nixgl }: let
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
        libglvnd
        mesa
        libdrm
        dbus
        libpulseaudio
        vulkan-loader
        vulkan-tools

        # Add the nixGL Vulkan wrapper for Mesa (AMD/Intel)
        nixgl.packages.${system}.nixVulkanIntel
      ];
      shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.libglvnd
          pkgs.mesa
          pkgs.libdrm
          pkgs.vulkan-loader
        ]}"
    '';
    };
  };
}
