{
  description = "devshell for github:lavafroth/cabinette";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs =  import nixpkgs {
            system = system;
            config = { allowUnfree = true; };
        }; in
        {
          devShells.default = pkgs.mkShell rec {
            libraries = with pkgs; [
              stdenv.cc.cc.lib
              webkitgtk
              gtk3
              cairo
              gdk-pixbuf
              glib
              dbus
              openssl
              librsvg
            ];
            packages = with pkgs;
            [
              # rust backend
              pkg-config
              dbus
              openssl
              glib
              gtk3
              libsoup
              webkitgtk
              librsvg
              cargo-tauri

              # dev tools for frontend
              tailwindcss
              vscode-langservers-extracted
            
              git
              gitRepo
              gnupg
              autoconf
              curl
              procps
              gnumake
              util-linux
              m4
              gperf
              unzip
              cudatoolkit
              linuxPackages.nvidia_x11
              libGLU libGL
              xorg.libXi xorg.libXmu freeglut
              xorg.libXext xorg.libX11 xorg.libXv xorg.libXrandr zlib 
              ncurses5
              stdenv.cc
              binutils
              python311Packages.torch-bin
              linuxPackages.nvidia_x11
              ncurses5
              stdenv.cc.cc.lib
            ];
            CUDA_PATH="${pkgs.cudatoolkit}";
            EXTRA_LDFLAGS="-L/lib -L${pkgs.linuxPackages.nvidia_x11}/lib";
            EXTRA_CCFLAGS="-I/usr/include";

            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath libraries}";
            OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include/openssl";
            OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib";
            OPENSSL_ROOT_DIR="${pkgs.openssl.out}";

          };
        }
      );
}
