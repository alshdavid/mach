mod cli;

fn main() {
  let mut args = std::env::args().collect::<Vec<String>>();
  args.remove(0);

  match cli::parse_command(&args) {
    Ok(result) => {dbg!(&result);},
    Err(err) => {println!("{}", err);},
  };
}