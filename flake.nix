{
  description = "Moon Shine - Development project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config = {
            allowUnfree = true;
          };
        };
      in {
        devShells.default = pkgs.mkShell {
          name = "moon-shine";
          packages = [
            pkgs.nodejs_22
            pkgs.pnpm
            pkgs.git
            pkgs.moonrepo
            pkgs.typescript
            pkgs.nodePackages.typescript-language-server
            pkgs.nodePackages.eslint
          ];
          shellHook = ''
            echo "🌙 Moon Shine Development Environment"
            echo "===================================="
            echo "📦 Available tools:"
            echo "  🌙 Moon:      $(moon --version)"
            echo "  📦 Node.js:   $(node --version)"
            echo "  📦 pnpm:      $(pnpm --version)"
            echo "  📝 TypeScript: $(tsc --version)"
            echo "===================================="
          '';
        };
      }
    );
}