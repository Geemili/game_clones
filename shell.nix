let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "game-clones-env";
    buildInputs = [
      nixpkgs.rustChannels.stable.rust
      git
      pkgconfig
      openssl
      gcc
      libstdcxx5
      libGL
      xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi xorg.libxcb
      alsaLib
      cmake
      freetype
      python3
      gnumake
      expat
      ];
    shellHook = ''
      export RUST_BACKTRACE=1
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${xorg.libX11}/lib
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${xorg.libXcursor}/lib
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${xorg.libXrandr}/lib
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${xorg.libXi}/lib
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${libGL}/lib
      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${xorg.libxcb}/lib
    '';
  }
