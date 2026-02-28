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
          snippet = {
            type = "command";
            value = [ "date" "--iso-8601" ];
            trim = true;
          };
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

The config file is in JSON format and follows the following structure (shown in Rust):
```rust
// Type for the whole config file
struct Config {
  // shell command to use for shell snippet (default: sh)
  shell: Option<String>,
  // snippet entries
  entries: Vec<Entry>,
  // Delay time before sending input event to ensure rofi window closes (in ms)
  delay: Option<u64>,
}

struct Entry {
  // string to match in Rofi
  key: String,
  // Snippet will be used for simulating keyboard input
  snippet: Snippet,
  // (optionaL) extra description for this entry
  description: Option<String>,
}

# Snippet is a tagged union
#[serde(tag = "type", rename_all = "camelCase")]
enum Snippet {
  // Use the text as is
  Text { value: String },
  // Run a command and use the output
  // (the first elem is the command and the rest are arguments)
  Command { value: Vec<String>, trim: Option<bool> },
  // Run a shell command and use the output
  Shell { value: String, trim: Option<bool> },
  // Evaluate a sequence of snippets and concatenate the outputs
  Sequence { value: Vec<Snippet> },
}

// Use the text as is
type Text = {
  type: "text",
  value: string,
}

// Run a command and use the output
// (the first elem is the command and the rest are arguments)
type Command = {
  type: "command",
  value: string[],
  trim?: boolean,  // Trim whitespaces at the beginning and the end
}

// Run a shell command and use the output
type Shell = {
  type: "shell",
  value: string,
  trim?: boolean,  // Trim whitespaces at the beginning and the end
}

// Evaluate a sequence of snippets and concatenate the outputs
type Sequence = {
  type: "sequence",
  value: Snippet[],
}
```

Example:

```json
{
  "shell": "bash",
  "delay": 100,
  "entries": [
    {
      "key": "hello",
      "description": "Hello Test",
      "snippet": {
        "type": "text",
        "value": "Hello world ❤️"
      }
    },
    {
      "key": "date",
      "snippet": {
        "type": "command",
        "value": ["date", "--iso-8601"],
        "trim": true
      }
    },
    {
      "key": "datetime",
      "snippet": {
        "type": "sequence",
        "value": [
          { "type": "shell", "value": "date +%F", "trim": true },
          { "type": "text", "value": " " },
          { "type": "shell", "value": "date +%T", "trim": true }
        ]
      }
    }
  ]
}
```


## Licence

AGPL-3.0

