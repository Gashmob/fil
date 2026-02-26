{
  stdenvNoCC,
  rpm,
  fil-version,
  callPackage,
  fil ? (callPackage ../../fil.nix { }),
}:
stdenvNoCC.mkDerivation {
  name = "fil";

  src = fil;
  nativeBuildInputs = [ rpm ];

  dontUnpack = true;
  dontConfigure = true;

  buildPhase = ''
    rpmbuild \
      --define "version ${fil-version}" \
      --define "_sourcedir ${fil}" \
      --dbpath="$(pwd)"/rpmdb \
      --define "%_topdir $(pwd)" \
      --define "%_tmppath %{_topdir}/TMP" \
      --define "_rpmdir %{_topdir}/RPMS" \
      -bb ${./fil.spec}
  '';

  installPhase = ''
    mkdir $out/
    mv RPMS/**/*.rpm $out/
  '';
}
