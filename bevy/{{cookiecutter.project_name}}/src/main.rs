#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

fn main() -> std::process::ExitCode {
  {{ cookiecutter.project_name | replace("-", "_") }}::plugin::main()
}
