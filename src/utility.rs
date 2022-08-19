pub fn print_board(mut board: u64) {
  let mut horizontals: Vec<String> = Vec::new();
  let mut horizontal: String = String::from("");
  
  for rank in 0..8 {
    horizontal += "\n";
    horizontal = format!("{}{}", horizontal, rank.to_string());
    horizontal += "   ";
    for file in 0..8 {
      horizontal = format!("{}{}", horizontal, (board & 1).to_string());
      horizontal += " ";
      board >>= 1;
    }
    horizontals.push(horizontal);
    horizontal = String::from("");
  }

  for rank in 0..8 {
    println!("{}", horizontals[7 - rank]);
  }
  println!("\n    0 1 2 3 4 5 6 7\n\n\n")
}