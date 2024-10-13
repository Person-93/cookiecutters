#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::app::App;

fn main() {
  App::new().add_plugins({{ cookiecutter.project_name | replace("-", "_") }}::plugin).run();
}
