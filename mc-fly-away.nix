# SPDX-FileCopyrightText: 2023 Dom Rodriguez (shymega) <shymega+mc-fly-away@shymega.org.uk>
#
# SPDX-License-Identifier: CC0-1.0

{ lib
, pkgs ? import <nixpkgs>
, rustPlatform
,
}:
rustPlatform.buildRustPackage {
  name = "mc-fly-away";

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    # Allow dependencies to be fetched from git and avoid having to set the outputHashes manually
    allowBuiltinFetchGit = true;
  };

  nativeBuildInputs = with pkgs; [ pkg-config openssl.dev ];
  buildInputs = with pkgs; [ openssl.dev ];

  meta = with lib; {
    description = "";
    homepage = "https://github.com/shymega/mc-fly-away";
    license = licenses.mit;
  };
}
