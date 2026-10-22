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

    finalShell = pkgs.rustPlatform.buildRustPackage {
      pname = "desktop-manager";
      version = "0.1.0";
      src = ./.;

      cargoLock = {
        lockFile = ./Cargo.lock;
        outputHashes = {
          "hyprland-0.4.0-beta.3" = "sha256-uhCZylXf81fvlBd6ToIOrlr3InVXLzv25LyvxxpmLWo=";
        };
      };
    };

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
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      pname = "desktop-manager-wrapped";
      version = "0.1.0";
      dontUnpack = true;

      nativeBuildInputs = [ pkgs.makeWrapper ];

      installPhase = ''
          mkdir -p $out/bin
          cp ${finalShell}/bin/desktop-manager $out/bin/desktop-manager

          wrapProgram $out/bin/desktop-manager \
            --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath [
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.libglvnd
              pkgs.mesa
              pkgs.libdrm
              pkgs.vulkan-loader
            ]}" \
            --set VK_DRIVER_FILES "/run/opengl-driver/share/vulkan/icd.d/radeon_icd.x86_64.json" \
            --set VK_ICD_FILENAMES "/run/opengl-driver/share/vulkan/icd.d/radeon_icd.x86_64.json"
        '';

    };
  };
}
