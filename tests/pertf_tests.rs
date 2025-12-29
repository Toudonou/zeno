#[cfg(test)]
mod pertf_tests {
  use std::fs::File;
  use std::io;
  use std::path::Path;

  use std::io::BufRead;
  use zeno::perft;
  use zeno::position::Position;

  #[test]
  fn pertf_standard_epd() -> io::Result<()> {
    let path = Path::new("tests/standard.epd");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut test_count = 0;
    for line in reader.lines() {
      test_count += 1;
      let line = line?;

      let mut parts = line.split(';');
      let fen = parts.next().unwrap().trim();

      let mut position = Position::from_fen(fen);
      let mut depth = 0;

      loop {
        let depth_info = parts.next();
        match depth_info {
          None => break,
          Some(depth_info) => {
            let mut info = depth_info.split_whitespace();
            depth = info.next().unwrap()[1..].parse().unwrap();
            let number_of_moves: u64 = info.next().unwrap().parse().unwrap();

            assert_eq!(number_of_moves, perft::perft(depth, &mut position));
            depth += 1;
          }
        }
      }
      depth -= 1;
      println!("Test {test_count}: FEN: {fen}; {depth}/{depth} tests OK")
    }

    Ok(())
  }
}
