{
  description = "A small game demo";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { nixpkgs,... }: 
    let
      pkgs = import nixpkgs { 
        system = "x86_64-linux"; 
      };
      deps = with pkgs; [
        vulkan-loader
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        alsa-lib
        udev
        pkg-config
        
        libxkbcommon
        zlib
        libdrm
        zstd
        xorg.libxcb
        xorg.libxshmfence
        wayland
        expat
        stdenv.cc.cc.lib
        llvm
      ];
    in 
  {
    devShell.x86_64-linux = pkgs.mkShell {
      buildInputs = (with pkgs; [
        cargo taplo rust-analyzer rustfmt
      ]) ++ deps;
      LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath deps}";
      RUST_SRC_PATH="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  };
}
