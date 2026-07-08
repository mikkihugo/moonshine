{
  description = "Moon Shine - Development project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          config = {
            allowUnfree = true;
          };
        };

        baseTools = with pkgs; [
          nodejs_22
          pnpm
          git
          typescript
          typescript-language-server
          eslint
          curl
          wget
          jq
          tree
          rustup
          cargo
          rustc
          moon
        ];
      in {
        packages.default = pkgs.writeShellScriptBin "moon-shine" ''
          echo "🌙 Moon Shine Development Environment"
          echo "Development tools are available in the shell"
        '';

        devShells.default = pkgs.mkShell {
          name = "moon-shine";
          packages = baseTools;
          shellHook = ''
            export PATH="$HOME/.local/bin:$PATH"
            echo "🌙 Moon Shine Development Environment"
            echo "===================================="
            echo "📦 Available tools:"
            echo "  🌙 Moon:      $(moon --version 2>/dev/null || echo 'moon unavailable')"
            echo "  📦 Node.js:   $(node --version)"
            echo "  📦 pnpm:      $(pnpm --version)"
            echo "  📝 TypeScript: $(tsc --version)"
            echo "===================================="
          '';
        };
      }
    );
}
