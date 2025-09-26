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
            pkgs.moon
            pkgs.claude-code
            pkgs.gemini-cli
            pkgs.codex
            pkgs.cursor-cli
            pkgs.typescript
            pkgs.nodePackages.typescript-language-server
            pkgs.nodePackages.eslint
          ];
          shellHook = ''
            echo "🌙 Moon Shine Development Environment"
            echo "===================================="
            echo "📦 Available tools:"
            echo "  🌙 Moon:      $(moon --version 2>/dev/null || echo 'Available')"
            echo "  🤖 Claude:    $(claude --version 2>/dev/null || echo 'Available')"
            echo "  🔮 Gemini:    $(gemini --version 2>/dev/null || echo 'Available')"
            echo "  🧠 Codex:     $(codex --version 2>/dev/null || echo 'Available')"
            echo "  🎯 Cursor:    $(cursor --version 2>/dev/null || echo 'Available')"
            echo "  📦 Node.js:   $(node --version)"
            echo "  📦 pnpm:      $(pnpm --version)"
            echo "  📝 TypeScript: $(tsc --version)"
            echo "===================================="
          '';
        };
      }
    );
}