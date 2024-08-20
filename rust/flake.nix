{
  description = "devshell for unreleased tauri app";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        {
          devShells.default = pkgs.mkShell rec {
            packages = with pkgs; [
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
            ];
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

            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath libraries}";
            OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include/openssl";
            OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib";
            OPENSSL_ROOT_DIR="${pkgs.openssl.out}";
          };
        }
      );
}
