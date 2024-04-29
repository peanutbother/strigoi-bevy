{ pkgs }: pkgs.rustPlatform.buildRustPackage {
  name = "strigoi";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  buildNoDefaultFeatures = true; # disable dynamic linking for production
}
