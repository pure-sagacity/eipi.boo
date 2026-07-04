{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  env = {
    EIPI_LISTEN = "0.0.0.0:2222";
    EIPI_NO_LIMIT = "1";
    #   # Linux
    #   LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    #     pkgs.stdenv.cc.cc.lib
    #   ];
    #   # MacOS
    # DYLD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    #  pkgs.stdenv.cc.cc.lib
    # ];
  };
  # https://devenv.sh/packages/
  packages = with pkgs; [
    # flyctl
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    lsp.enable = true;
    components = [
      "rustc"
      "cargo"
      "rustfmt"
      "clippy"
      "rust-analyzer"
    ];
  };

  enterShell = ''
    echo "------eipi.boo--------"
  '';

  enterTest = ''
    echo "Running tests"
    cargo test
  '';

  git-hooks.hooks = {
    shellcheck.enable = true;
    prettier.enable = true;
    clippy.enable = true;
    rustfmt.enable = true;
    alejandra.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
