{ pkgs ? import <nixpkgs> {} }:    
  let
    libPath = with pkgs; lib.makeLibraryPath [
      libGL
      libGLU
      libxkbcommon
      wayland

      xorg.libX11

      vulkan-loader
    ];
  in {
    devShell = with pkgs; mkShell {
      buildInputs = [
        cargo
        rustc
        rust-analyzer
        pkg-config
      ];
      
      RUST_LOG = "debug";
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      LD_LIBRARY_PATH = libPath;
    };
  }
