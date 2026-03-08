{ rustPlatform, lib }:

rustPlatform.buildRustPackage rec {
  name = "fil";
  src = ./.;

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = "${src}/Cargo.lock";
  };

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
