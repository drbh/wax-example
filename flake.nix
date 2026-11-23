{
  description = "wax-example: a tiny metal kernel compiled and run with candle's wax";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      # metal only, so apple silicon only.
      systems = [ "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      # `nix run` compiles the kernel and runs it on the gpu.
      packages = forAllSystems (pkgs: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "wax-example";
          version = "0.1.0";
          src = self;
          # candle and cutile are git dependencies; let nix fetch them from Cargo.lock.
          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = true;
          };
          meta.mainProgram = "wax-example";
        };
      });

      # `nix develop` for cargo, clippy and rustfmt.
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [ cargo rustc clippy rustfmt ];
        };
      });
    };
}
