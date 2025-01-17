let pkgs = import <nixpkgs> { };

in pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo

    libGL
    alsa-lib
    xorg.libX11
    xorg.libXi
    libxkbcommon
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
      with pkgs;
      pkgs.lib.makeLibraryPath [ libGL xorg.libX11 xorg.libXi libxkbcommon ]
    }"
  '';
}