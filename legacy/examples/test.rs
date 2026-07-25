use ruthenian::*;

fn main() {
    println!("=== Russian to Ruthenian ===");
    let russian1 = "Привет мир!";
    println!("{} -> {}", russian1, russian_to_ruthenian(russian1));

    let russian2 = "Здравствуйте!";
    println!("{} -> {}", russian2, russian_to_ruthenian(russian2));

    let russian3 = "Щука жарко";
    println!("{} -> {}", russian3, russian_to_ruthenian(russian3));

    println!("\n=== Ruthenian to Russian ===");
    let ruthenian1 = "Pryvjet mir!";
    println!("{} -> {}", ruthenian1, ruthenian_to_russian(ruthenian1));

    let ruthenian2 = "Zdravstvujtje!";
    println!("{} -> {}", ruthenian2, ruthenian_to_russian(ruthenian2));

    let ruthenian3 = "Szczuka zzarko";
    println!("{} -> {}", ruthenian3, ruthenian_to_russian(ruthenian3));

    println!("\n=== Round-trip test ===");
    let original = "Борщ";
    let to_ruthenian = russian_to_ruthenian(original);
    let back_to_russian = ruthenian_to_russian(&to_ruthenian);
    println!("Original: {}", original);
    println!("To Ruthenian: {}", to_ruthenian);
    println!("Back to Russian: {}", back_to_russian);

    match test_roundtrip_from_file_ru("biblija_ru.txt") {
        Ok(()) => {}
        Err(e) => {
            println!("Error reading file: {}", e);
            println!("\nTo test, create a file named 'russian_text.txt' with Russian text.");
            println!("Example content:");
            println!("Привет мир!");
            println!("Здравствуйте!");
            println!("Щука жарко");
        }
    }

    match test_roundtrip_from_file_ukr("biblija_ukr.txt") {
        Ok(()) => {}
        Err(e) => {
            println!("Error reading file: {}", e);
            println!("\nTo test, create a file named 'russian_text.txt' with Russian text.");
            println!("Example content:");
            println!("Привет мир!");
            println!("Здравствуйте!");
            println!("Щука жарко");
        }
    }
}
