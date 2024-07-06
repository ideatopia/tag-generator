# To learn more about how to use Nix to configure your environment
# see: https://developers.google.com/idx/guides/customize-idx-env
{ pkgs, ... }: {
  # Which nixpkgs channel to use.
  channel = "stable-23.11"; # or "unstable"

  # Use https://search.nixos.org/packages to find packages
  packages = [
    pkgs.cargo
    pkgs.rustup
    pkgs.gh
  ];

  # Sets environment variables in the workspace
  env = {};
  idx = {
    # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
    extensions = [
      "rust-lang.rust-lang"
      "franneck94.vscode-rust-config"
      "JScearcy.rust-doc-viewer"
      "LyonSyonII.rust-syntax"
    ];

    # Enable previews
    previews = {
      enable = false;
    };

    # Workspace lifecycle hooks
    workspace = {
      # Runs when a workspace is first created
      onCreate = {
        # Example: install JS dependencies from NPM
        cargo-install = "cargo install";
        rustup-setup = "rustup default stable";
      };
      # Runs when the workspace is (re)started
      onStart = {
        # Example: start a background task to watch and re-build backend code
        cargo-run = "cargo run";
        cargo-install = "cargo install";
      };
    };
  };
}
