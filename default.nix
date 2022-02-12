{ pkgs ? import <nixpkgs> { }
, naersk ? pkgs.callPackage
    (pkgs.fetchFromGitHub {
      repo = "naersk";
      owner = "nix-community";
      rev = "2fc8ce9d3c025d59fee349c1f80be9785049d653";
      sha256 = "sha256-pGsM8haJadVP80GFq4xhnSpNitYNQpaXk4cnA796Cso";
    })
    {}
,
}:
with pkgs; naersk.buildPackage ./.
