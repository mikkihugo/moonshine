{
  description = "Moon Shine - Development project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    codingAgents.url = "path:/home/mhugo/code/mhugo/coding-agents";
    codingAgents.inputs.nixpkgs.follows = "nixpkgs";
    codingAgents.inputs.flake-utils.follows = "flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    codingAgents,
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
          nodePackages.pnpm
          git
          typescript
          nodePackages.typescript-language-server
          nodePackages.eslint
          curl
          wget
          jq
          tree
          rustup
          cargo
          rustc
          moon
        ];
        aiPackages = codingAgents.packages.${system};
        aiTools = [
          aiPackages.claude
          aiPackages.gemini
          aiPackages."gemini-cli"
          aiPackages.codex
        ];
      in {
        packages.default = pkgs.writeShellScriptBin "moon-shine" ''
          echo "🌙 Moon Shine Development Environment"
          echo "Development tools are available in the shell"
        '';

        devShells.default = pkgs.mkShell {
          name = "moon-shine";
          packages = baseTools ++ aiTools;
          shellHook = ''
            export PATH="$HOME/.local/bin:$PATH"
            echo "🌙 Moon Shine Development Environment"
            echo "===================================="
            echo "📦 Available tools:"
            echo "  🌙 Moon:      $(moon --version 2>/dev/null || echo 'moon unavailable')"
            echo "  🤖 Claude:    $(claude --version 2>/dev/null || echo 'claude unavailable')"
            echo "  🔮 Gemini:    $(gemini --version 2>/dev/null || echo 'gemini unavailable')"
            echo "  🧠 Codex:     $(codex --version 2>/dev/null || echo 'codex unavailable')"
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
