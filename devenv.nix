{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [
    pkgs.git

    pkgs.wasm-pack
    pkgs.lld
    pkgs.mold
    pkgs.linuxPackages_latest.perf
    pkgs.cargo-flamegraph
    pkgs.samply
    pkgs.graphviz
  ];

  # https://devenv.sh/languages/
  # languages.rust.enable = true;
  devcontainer.enable = true;
  languages.rust.enable = true;
  languages.rust.mold.enable = false;
  languages.typescript.enable = true;
  languages.nix.enable = true;
  languages.javascript.yarn.enable = true;
  languages.javascript.enable = true;

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  enterShell = ''
    hello
    git --version
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
