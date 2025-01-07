{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    naersk.url = "github:nix-community/naersk/master";
    utils.url = "github:numtide/flake-utils";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, systems, ... } @ inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (system: {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
        devenv-test = self.devShells.${system}.default.config.test;
      });

      devShells = forEachSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            default = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                {
                  # https://devenv.sh/reference/options/
                  packages = [
                    pkgs.wasm-pack
                    pkgs.lld
                    pkgs.mold

                  ];
                  devcontainer.enable = true;
                 languages.rust.enable = true;
                 languages.rust.mold.enable = false;
                 languages.typescript.enable = true;
                 languages.nix.enable = true;
                 languages.javascript.yarn.enable = true;
                 languages.javascript.enable = true;
                  enterShell = ''
                    echo hi
                  '';

                  processes.hello.exec = "echo hi";
                }
              ];
            };
          });
    };
}
