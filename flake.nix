{
  description = "🎁 generate beautiful landing pages for your developer tools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , fenix
    , ...
    }:
    flake-utils.lib.eachDefaultSystem
      (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            fenix.overlays.default
          ];
        };

        # Parse the local Cargo.toml so we track the usual rust workflow
        cargo_toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        package = with pkgs;
          rustPlatform.buildRustPackage {
            pname = cargo_toml.package.name;
            version = cargo_toml.package.version;
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            # Don't run checks
            doCheck = false;

            # Package metadata
            meta = with pkgs.lib; {
              description = cargo_toml.package.description;
              homepage = cargo_toml.package.repository;
              license = with licenses; [ asl20 mit ];
            };

            nativeBuildInputs = with pkgs; [
              pkg-config
              tailwindcss
            ];

            buildInputs = with pkgs; ([
              bzip2
              oniguruma
              openssl
              xz
              zstd
            ]
            ++ darwinInputs);

            RUSTONIG_SYSTEM_LIBONIG = true;
            ZSTD_SYS_USE_PKG_CONFIG = true;
            NIX_LDFLAGS = nixLdFlags;
          };

        # Darwin-specific build requirements
        frameworks = pkgs.darwin.apple_sdk.frameworks;
        darwinInputs = with pkgs; (lib.optionals stdenv.isDarwin [ libiconv frameworks.Security ]);
        nixLdFlags = with pkgs; (lib.optionalString (stdenv.isDarwin) "-F${frameworks.CoreServices}/Library/Frameworks -framework CoreServices -L${libiconv}/lib");
      in
      {
        packages = rec {
          oranda = package;
          default = oranda;
        };

        devShells = with pkgs; {
          default = mkShell {
            nativeBuildInputs = package.nativeBuildInputs;

            buildInputs =
              [
                (fenix.packages.${system}.complete.withComponents [
                  "cargo"
                  "clippy"
                  "rust-src"
                  "rustc"
                  "rustfmt"
                ])
              ]
              ++ darwinInputs;

            # Allow rust-analyzer and other tools to see rust src
            RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";

            # Fix missing OpenSSL
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

            shellHook = ''
              export NIX_LDFLAGS="${nixLdFlags}"
            '';
          };
        };
      });
}
