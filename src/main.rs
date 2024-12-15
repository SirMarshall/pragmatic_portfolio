use pragmatic_portfolio::{utilities, algorithms};

fn main() {
   intro();
   let mut choice = 0;
   while choice != 4 {
      menu();
      choice = utilities::get_input().parse().unwrap();
      match choice {
         1 => algorithms::analyze::master(),
         2 => algorithms::organize::master(),
         3 => algorithms::sort::master(),
         4 => println!("Exiting..."),
         _ => println!("Invalid choice. Try again."),
      }
   }
   println!("Goodbye!");
}

fn intro() {
   println!("=== Pragmatic Portfolio ===");
   println!("Enter your name: ");
   let name = utilities::get_input();
   println!("What a name. Let us begin, {}.", name);
}

fn menu() {
   println!("1. Analyze files");
   println!("2. Organize files");
   println!("3. Sort files");
   println!("4. Exit");
}