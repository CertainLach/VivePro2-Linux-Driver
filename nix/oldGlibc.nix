let nixpkgs-old = builtins.fetchTarball {
  name = "release-18.09";
  url = "https://github.com/nixos/nixpkgs/archive/925ff360bc33876fdb6ff967470e34ff375ce65e.tar.gz";
  sha256 = "1qbmp6x01ika4kdc7bhqawasnpmhyl857ldz25nmq9fsmqm1vl2s";
};
in
self: super:
let
  oldPkgs = import nixpkgs-old {
    system = super.system;
  };
in
rec {
  glibc = oldPkgs.glibc // { pname = "glibc"; };
  glibcLocales = oldPkgs.glibcLocales;
  glibcIconv = oldPkgs.glibcIconv;
  stdenv = super.stdenv // {
    overrides = self2: super2: super.stdenv.overrides self2 super2 // {
      glibc = glibc;
      linuxHeaders = glibc.linuxHeaders;
    };
  };
}
