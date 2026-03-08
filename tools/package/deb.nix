{
  stdenvNoCC,
  dpkg,
  fil-version,
  callPackage,
  fil ? (callPackage ../../fil.nix { }),
}:
stdenvNoCC.mkDerivation {
  name = "fil";

  src = fil;
  nativeBuildInputs = [ dpkg ];

  dontUnpack = true;
  dontConfigure = true;

  buildPhase = ''
    mkdir -p fil/DEBIAN
    mkdir -p fil/usr/bin

    cp ${fil}/bin/fil fil/usr/bin
    cp ${./control} fil/DEBIAN/control
    sed -i 's/%version%/${fil-version}/' fil/DEBIAN/control
    echo '2.0' > fil/debian-binary

    dpkg-deb --build fil
  '';

  installPhase = ''
    mkdir $out/
    mv fil.deb $out/
  '';
}
