{
  description = "Development Nix flake for permutation-enum (Rust + Haskell)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        # Rust ツールチェーン本体は Nix が提供（バージョンは flake.lock で固定）。
        # 依存クレートは従来どおり Cargo（rust/Cargo.toml / rust/Cargo.lock）が管理する。
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {

        formatter = pkgs.nixfmt-tree;

        devShells = {
          # `nix develop`（既定）= Rust。`cd rust && cargo test` など。
          default = pkgs.mkShell {
            packages = with pkgs; [
              # --- toolchains ---
              rust # cargo / rustc / clippy / rustfmt + rust-src / rust-analyzer

              # --- system deps（クレートが必要とするものだけ足す。最初は空でOK）---
              # 例: pkg-config / openssl など、ビルドエラーが出てから追加する

            ];
          };

          # `nix develop .#haskell` = Haskell。`cd haskell && runghc Main.hs 3` など。
          haskell = pkgs.mkShell {
            packages = with pkgs; [
              ghc # ghc / runghc（runhaskell）
              haskell-language-server # エディタ補完（任意）
            ];
          };
        };
      }
    );
}
