{
  inputs.nixpkgs.url = "github:nixos/nixpkgs";
  inputs.naersk.url = "github:nix-community/naersk";

  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, fenix, naersk }:
    let pkgs = import nixpkgs { system = "x86_64-linux"; };
        rust_tc = fenix.packages.x86_64-linux.minimal;
        rust = pkgs.makeRustPlatform {
          rustc = rust_tc.rustc; cargo = rust_tc.cargo;
        };
    in {
      defaultPackage.x86_64-linux = rust.buildRustPackage {
        name = "amp";
        version = "0.6.2-p";
        src = ./.;

        cargoSha256 = "sha256-BBEbQZxzldkzWiaGeViH/Bt02pJ3pRT2UKEgcC9FAPY=";

        nativeBuildInputs = [ pkgs.python3 ];
        buildInputs = [ pkgs.xorg.libxcb ];

        doCheck = false; # Nix has some ... issues ... with the tests.
      };

      devShell.x86_64-linux = pkgs.mkShell {
        buildInputs = [
          pkgs.python3
          pkgs.xorg.libxcb
          rust_tc.rustc rust_tc.cargo
        ];
      };
    };
}
