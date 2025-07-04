# rofi-snippets

A Rofi plugin to select snippets and simulate keyboard input of snippet content.
It supports both Wayland and X11.


## Installation

### Nix

This package is available as an [NUR](https://nur.nix-community.org/documentation/) package `nur.repos.dcsunset.rofi-snippets`.
It is also available from the flake output of the [NUR repo](https://github.com/DCsunset/nur-packages), where a NixOS overlay is provided as well.

To use it with NixOS:
```nix
environment.systemPackages = [
  (pkgs.rofi.override (old: {
    plugins = old.plugins ++ [nur.repos.dcsunset.rofi-snippets];
  }))
];
```

Or you can also use it with home-manager:
```nix
programs.rofi = {
  enable = true;
  plugins = [nur.repos.dcsunset.rofi-snippets];
}
```

The NUR repo also provides a home-manager module via flake:
``` nix
{
  imports = [ inputs.nur-dcsunset.homeManagerModules.rofi-snippets ];

  # rofi should be enabled as well
  programs.rofi.enable = true;

  programs.rofi-snippets = {
    enable = true;
    settings = {
      entries = [
        {
          key = "date";
          snippet.command = ["date" "--iso-8601"];
        }
      ];
    };
  };
}
```


### Pre-built Binaries

Pre-built binaries can be downloaded from GitHub release.
Put the downloaded `.so` file in Rofi lib dir depending on your Rofi installation (e.g. `/lib/rofi`) or add the plugin's dir to `ROFI_PLUGIN_PATH` environment variable.

Note that runtime dependencies (including glib, cairo, pango) must be installed before using the pre-built binaries.
If the pre-built binaries don't work for your system, try other installation methods.

### Manually

1. Clone this repo
2. Install dependencies based on your environment
   - Common: rust, pkg-config, glib, cairo, pango, libxkbcommon
   - X11: xdotool
3. Build the plugin
   - Wayland only: `cargo build --release --features=wayland --no-default-features`
   - X11 only: `cargo build --release --features=x11 --no-default-features`
   - Both: `cargo build --release`
4. Move the built plugin (`target/release/librofi_snippets.so`) to Rofi lib dir or add the plugin's dir to `ROFI_PLUGIN_PATH` environment variable


## Usage

To start Rofi with rofi-snippets mode:
`rofi -show rofi-snippets -modes rofi-snippets`


## Configuration

Rofi-snippets will look for the configuration file in the following order (with environment variables):
- `$ROFI_SNIPPETS_CONFIG`
- `$XDG_CONFIG_HOME/rofi-snippets/config.json`
- `$HOME/.config/rofi-snippets/config.json`

The config file is in JSON format and follows the following structure (shown in TypeScript):
```ts
// Type for the whole config file
type Config = {
  // (optional) shell command to use for shell snippet (default: sh)
  shell?: string,
  // snippet entries
  entries: Entry[],
}

type Entry = {
  // string to match in Rofi
  key: string,
  // Snippet will be used for simulating keyboard input
  snippet: Snippet,
  // (optionaL) extra description for this entry
  description?: string
}

type Snippet = Text | Command | Shell | Sequence

// Use the text as is
type Text = {
  text: string
}

// Run a command and use the output
// (the first elem is the command and the rest are arguments)
type Command = {
  command: string[]
}

// Run a shell command and use the output
type Shell = {
  shell: string
}

// Evaluate a sequence of snippets and concatenate the outputs
type Sequence = {
  sequence: Snippet[]
}
```

Example:

```json
{
  "shell": "bash",
  "entries": [
    {
      "key": "hello",
      "description": "Hello Test",
      "snippet": {
        "text": "Hello world"
      }
    },
    {
      "key": "date",
      "snippet": {
        "command": ["date", "--iso-8601"]
      }
    },
    {
      "key": "datetime",
      "snippet": {
        "sequence": [
          { "shell": "date +%F" },
          { "text": " " },
          { "shell": "date +%T" }
        ]
      }
    }
  ]
}
```


## Licence

AGPL-3.0

