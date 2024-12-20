{
  lib,
  stdenv,
  makeRustPlatform,

  buildPackages,
  # pkgsStatic,

  # arguments
  buildInputs ? [ ],
  nativeBuildInputs ? [ ],
  fullStatic ? false,
  buildNoDefaultFeatures ? false,
  buildFeatures ? [ ],
}:
let
  inherit (stdenv.hostPlatform) rust;
  toolchain = buildPackages.rust-bin.stable.latest.default.override {
    targets = [ rust.rustcTarget ];
  };
  rustPlatform = makeRustPlatform {
    inherit stdenv;
    cargo = toolchain;
    rustc = toolchain;
  };
in
rustPlatform.buildRustPackage {
  name = "cleactl";
  version = "0.1.0";

  separateDebugInfo = true;

  src = ../.;

  inherit buildNoDefaultFeatures buildFeatures;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = nativeBuildInputs ++ [ ];
  buildInputs =
    buildInputs ++ lib.optionals (!buildNoDefaultFeatures) [ ] ++ (if fullStatic then [ ] else [ ]);

  "CARGO_TARGET_${rust.cargoEnvVarTarget}_LINKER" = "${stdenv.cc.targetPrefix}ld";
  CARGO_BUILD_RUSTFLAGS = lib.optional fullStatic "-C target-feature=+crt-static";

  meta = {
    mainProgram = "cleactl";
  };
}
