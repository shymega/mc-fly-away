# SPDX-FileCopyrightText: 2023 Dom Rodriguez (shymega) <shymega+mc-fly-away@shymega.org.uk>
#
# SPDX-License-Identifier: CC0-1.0

(import
  (
    let lock = builtins.fromJSON (builtins.readFile ./flake.lock); in
    fetchTarball {
      url = "https://github.com/edolstra/flake-compat/archive/${lock.nodes.flake-compat.locked.rev}.tar.gz";
      sha256 = lock.nodes.flake-compat.locked.narHash;
    }
  )
  { src = ./.; }
).defaultNix
