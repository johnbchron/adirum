{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    devshell.url = "github:numtide/devshell";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, devshell, rust-overlay, flake-utils, ... }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) devshell.overlays.default ];
        };

        toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "rust-analyzer" "rustc-codegen-cranelift-preview" ];
        });

        deps = with pkgs; [
          toolchain pkg-config clang lld
        ];
      in {
        devShell = pkgs.devshell.mkShell {
          packages = deps;
          motd = "\n  Welcome to the {2}adirum{reset} shell.\n";
        };
      });
}
