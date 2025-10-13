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
    
    let ruthenian2 ="Zdravstvujtje!";
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
}
