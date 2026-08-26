use clap::Parser;
use zeno::cmd;

fn main() {
  let args = cmd::Cmd::parse();
  cmd::process_cmd(args);
}
