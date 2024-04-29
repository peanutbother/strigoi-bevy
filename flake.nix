{
  description = "A flake for building strigoi.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
  };

  outputs = inputs: (import ./util.nix).eachSystem (system:
    let
      pkgs = import inputs.nixpkgs { inherit system; };
      packages = with pkgs; [
        cmake
        graphviz # used by bevy editor
        iconv # rust dep
        rustup
      ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
        darwin.apple_sdk.frameworks.Cocoa # C headers for bindings
        rustPlatform.bindgenHook # fix llvm / cxx for rust 
      ];
    in
    {
      packages.default = pkgs.callPackage ./. { };
      devShells.default = pkgs.mkShell {
        inherit packages;
        shellHook = ''
          rustc --version
        '';
      };
    }
  );
}
