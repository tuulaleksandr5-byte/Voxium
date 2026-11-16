//! Пересоздаёт `docs/BLOCKS.md` и `docs/BLOCKS.csv` из реестров блоков.
//!
//! Запуск из корня репозитория: `cargo run -p game_core --example block_list`.

fn main() {
    std::fs::write("docs/BLOCKS.md", game_core::catalog::markdown()).expect("docs/BLOCKS.md");
    std::fs::write("docs/BLOCKS.csv", game_core::catalog::csv()).expect("docs/BLOCKS.csv");
    println!("docs/BLOCKS.md и docs/BLOCKS.csv обновлены");
}
