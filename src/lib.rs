// rofi-snippets
// Copyright (C) 2025  DCsunset
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use enigo::Keyboard;
use serde::{Serialize, Deserialize};
use std::{default::Default, env, ffi::OsStr, fs, process::Command};
use nix::unistd::{fork, ForkResult};

#[derive(Serialize, Deserialize)]
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

/// Run command and get output as string
fn run_command(cmd: &str, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> String {
  let output = Command::new(OsStr::new(cmd))
    .args(args)
    .output()
    .expect("Error running command");
  String::from_utf8(output.stdout).expect("Fail to decode output")
}

fn trim_str(s: String, trim: bool) -> String {
  if trim {
    s.trim().to_string()
  } else {
    s
  }
}

#[derive(Serialize, Deserialize)]
struct Entry {
  // string to match in Rofi
  key: String,
  // Snippet will be used for simulating keyboard input
  snippet: Snippet,
  // (optionaL) extra description for this entry
  description: Option<String>,
}

impl From<&Entry> for rofi_mode::String {
  fn from(entry: &Entry) -> Self {
    let desc = entry.description.as_ref()
      .map(|v| format!("<span weight='light' size='small' style='italic'>({})</span>", v)).unwrap_or_default();
    rofi_mode::format!("{} {}", entry.key, desc)
  }
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
  // shell command to use for shell snippet (default: sh)
  shell: Option<String>,
  // snippet entries
  entries: Vec<Entry>,
  // Delay time before sending input event to ensure rofi window closes (in ms)
  delay: Option<u64>,
}

struct Mode {
  cfg: Config,
}

impl Mode {
  fn compute_snippet(&self, snippet: &Snippet) -> String {
    match snippet {
      Snippet::Text { value } => value.clone(),
      Snippet::Command { value, trim } => {
        trim_str(
          run_command(value.get(0).expect("Empty command"), &value[1..]),
          trim.unwrap_or(false)
        )
      },
      Snippet::Shell { value, trim } => {
        trim_str(
          run_command(
            self.cfg.shell.as_ref().map(|v| v.as_str()).unwrap_or("sh"),
            ["-c", value]
          ),
          trim.unwrap_or(false)
        )
      },
      Snippet::Sequence { value} => {
        value.iter()
          .map(|v| self.compute_snippet(v))
          .fold(String::new(), |l, r| l + r.as_str())
      }
    }
  }
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode {
  const NAME: &'static str = "rofi-snippets\0";

  fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
    let config_file = env::var("ROFI_SNIPPETS_CONFIG")
      .or_else(|_| env::var("XDG_CONFIG_HOME").map(|v| v + "/rofi-snippets/config.json"))
      .or_else(|_| env::var("HOME").map(|v| v + "/.config/rofi-snippets/config.json"))
      .expect("Unable to locate config dir");
    let cfg: Config = fs::File::open(&config_file)
      .map(|v| serde_json::from_reader(v).expect("Invalid config file"))
      .unwrap_or_default();

    api.set_display_name("snippets");
    Ok(Self { cfg })
  }

  fn entries(&mut self) -> usize {
    self.cfg.entries.len()
  }

  fn entry_content(&self, line: usize) -> rofi_mode::String {
    (&self.cfg.entries[line]).into()
  }

  fn entry_style(&self, _line: usize) -> rofi_mode::Style {
    rofi_mode::Style::MARKUP
  }

  fn react(
    &mut self,
    event: rofi_mode::Event,
    input: &mut rofi_mode::String,
  ) -> rofi_mode::Action {
    match event {
      rofi_mode::Event::Cancel { selected: _ } => return rofi_mode::Action::Exit,
      rofi_mode::Event::Ok {
        alt: _,
        selected,
      } => {
        // Must send input events in the backround to let rofi exit first
        match unsafe { fork().unwrap() } {
          ForkResult::Parent { .. } => {},
          ForkResult::Child => {
            if let Some(delay) = self.cfg.delay {
              std::thread::sleep(std::time::Duration::from_millis(delay));
            }
            let mut enigo = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
            enigo.text(
              self.compute_snippet(&self.cfg.entries[selected].snippet).as_str()
            ).unwrap();
            unsafe { nix::libc::exit(0) };
          }
        }
        return rofi_mode::Action::Exit;
      }
      rofi_mode::Event::Complete {
        selected: Some(selected),
      } => {
        input.clear();
        input.push_str(&self.cfg.entries[selected].key);
      }
      _ => {}
    }
    rofi_mode::Action::Reload
  }

  fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
    matcher.matches(&self.cfg.entries[line].key)
  }
}

rofi_mode::export_mode!(Mode);

