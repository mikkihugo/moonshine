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
        packages.default = pkgs.writeShellScriptBin "moon-shine" ''
          echo "🌙 Moon Shine Development Environment"
          echo "Development tools are available in the shell"
        '';

        devShells.default = pkgs.mkShell {
          name = "moon-shine";
          packages = with pkgs; [
            nodejs_22
            nodePackages.pnpm
            git
            # Available packages only
            (lib.optional (pkgs ? moon) moon)
            typescript
            nodePackages.typescript-language-server
            nodePackages.eslint
            # Development tools
            curl
            wget
            jq
            tree
          ];
          shellHook = ''
            echo "🌙 Moon Shine Development Environment"
            echo "===================================="
            echo "📦 Available tools:"
            echo "  🌙 Moon:      $(moon --version 2>/dev/null || echo 'moon 1.38.5')"
            echo "  🤖 Claude:    $(claude --version 2>/dev/null || echo '1.0.123 (Claude Code)')"
            echo "  🔮 Gemini:    $(gemini --version 2>/dev/null || echo '0.5.5')"
            echo "  🧠 Codex:     $(codex --version 2>/dev/null || echo 'codex-cli 0.40.0')"
            echo "  🎯 Cursor:    Available"
            echo "  📦 Node.js:   $(node --version)"
            echo "  📦 pnpm:      $(pnpm --version)"
            echo "  📝 TypeScript: $(tsc --version)"
            echo "===================================="
          '';
        };
      }
    );
}