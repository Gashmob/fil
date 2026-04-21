{
  rustPlatform,
  lib,
  libllvm,
  libffi,
  libxml2,
  zlib,
}:

rustPlatform.buildRustPackage rec {
  name = "fil";
  src = ./.;

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = "${src}/Cargo.lock";
  };

  nativeBuildInputs = [ libllvm ];
  buildInputs = [
    libffi
    libxml2
    zlib
  ];

  doCheck = true;
  checkPhase = ''
    runHook preCheck
    cargo check
    cargo test --bins
    runHook postCheck
  '';

  meta = {
    description = "Main tool for fil language";
    homepage = "https://github.com/Gashmob/fil";
    changelog = "https://github.com/Gashmob/fil/blob/master/CHANGELOG.md";
    license = [ lib.licenses.gpl2Plus ];
    mainProgram = "fil";
  };
}
