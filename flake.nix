    {
      description = "Python 3.11 development environment";
      outputs = { self, nixpkgs }:
      let
        system = "x86_64-linux";
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      in {
        devShells.${system}.default = (pkgs.buildFHSUserEnv {
          name = "cuda-mamba-devenv";
          targetPkgs = pkgs: (with pkgs; [
            linuxPackages.nvidia_x11
            libGLU libGL
            xorg.libXi xorg.libXmu freeglut
            xorg.libXext xorg.libX11 xorg.libXv xorg.libXrandr zlib 
            ncurses5 stdenv.cc binutils
            ffmpeg
            fish
            micromamba
          ]);

          profile = ''
              export LD_LIBRARY_PATH="${pkgs.linuxPackages.nvidia_x11}/lib:${pkgs.openssl.dev}/lib"
              export CUDA_PATH="${pkgs.cudatoolkit}"
              export EXTRA_LDFLAGS="-L/lib -L${pkgs.linuxPackages.nvidia_x11}/lib"
              export EXTRA_CCFLAGS="-I/usr/include"
          '';

          runScript = "fish";
        }).env;
      };
    }
