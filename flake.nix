{
  description = "datapass - CLI tool to fetch and display mobile data usage from datapass.de";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
    treefmt-nix,
  }:
    flake-utils.lib.eachSystem [
      "x86_64-linux"
      "aarch64-linux"
      "i686-linux"
      "aarch64-darwin"
      "x86_64-darwin"
    ] (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer" "clippy"];
        };

        # Crane lib for building Rust projects
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Source filtering
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        # Common arguments for crane
        commonArgs = {
          inherit src;
          strictDeps = true;

          pname = "datapass";
          version = "0.1.0";

          buildInputs = with pkgs;
            [
              openssl
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };

        # Build dependencies only (for caching)
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the actual crate
        datapass = craneLib.buildPackage (commonArgs
          // {
            inherit cargoArtifacts;
            doCheck = true;
          });

        # Treefmt configuration
        treefmtEval = treefmt-nix.lib.evalModule pkgs {
          projectRootFile = "flake.nix";
          programs = {
            alejandra.enable = true; # Nix formatter
            rustfmt.enable = true; # Rust formatter
            prettier.enable = true; # JSON, YAML, Markdown formatter
            taplo.enable = true; # TOML formatter
          };
        };
      in {
        # Package outputs
        packages = {
          default = datapass;
          datapass = datapass;
        };

        # Development shell
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            # Rust tools
            rustToolchain
            cargo-watch
            cargo-edit
            cargo-audit
            cargo-outdated

            # Build dependencies
            pkg-config
            openssl

            # Formatters and linters
            treefmt
            alejandra # Nix formatter
            rustfmt
            clippy
            deadnix # Find dead Nix code
            statix # Lints and suggestions for Nix

            # Tools for formatting other files
            nodePackages.prettier
            taplo # TOML formatter

            # Cross compilation tools
            cargo-cross

            # Useful utilities
            ripgrep
            fd
            jq
          ];

          shellHook = ''
            echo "🦀 datapass development shell"
            echo "Available commands:"
            echo "  cargo build        - Build the project"
            echo "  cargo test         - Run tests"
            echo "  cargo run          - Run the application"
            echo "  cargo clippy       - Run linter"
            echo "  treefmt            - Format all files"
            echo "  nix flake check    - Run all checks"
          '';
        };

        # Formatter (run with: nix fmt)
        formatter = treefmtEval.config.build.wrapper;

        # Checks (run with: nix flake check)
        checks = {
          # Build check
          inherit datapass;

          # Clippy check
          datapass-clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          # Documentation check
          datapass-doc = craneLib.cargoDoc (commonArgs
            // {
              inherit cargoArtifacts;
            });

          # Audit dependencies (commented out - uncomment and update sha256 to enable)
          # datapass-audit = craneLib.cargoAudit {
          #   inherit src;
          #   advisory-db = pkgs.fetchFromGitHub {
          #     owner = "rustsec";
          #     repo = "advisory-db";
          #     rev = "refs/heads/main";
          #     sha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
          #   };
          # };

          # Nix-specific checks
          deadnix-check =
            pkgs.runCommand "deadnix-check" {
              nativeBuildInputs = [pkgs.deadnix];
            } ''
              deadnix --fail ${self}
              touch $out
            '';

          statix-check =
            pkgs.runCommand "statix-check" {
              nativeBuildInputs = [pkgs.statix];
            } ''
              statix check ${self}
              touch $out
            '';
        };

        # Apps (run with: nix run)
        apps.default = flake-utils.lib.mkApp {
          drv = datapass;
        };
      }
    );
}
