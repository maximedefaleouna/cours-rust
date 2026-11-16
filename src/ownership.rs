pub fn main() {
    println!("=== CONCEPTS D'OWNERSHIP ET D'EMPRUNT EN RUST ===");

    // 1. Possession unique et deplacement de propriete (Move)
    let nom = String::from("DEFALEOUNA");
    let nom_copie = nom.clone(); // Duplication explicite sur le tas (heap)
    println!("Chaine originale clonee : {}", nom_copie);

    // 2. Emprunt immuable (Borrowing &)
    let longueur = calculer_longueur(&nom);
    println!("Longueur de '{}' via reference immuable : {}", nom, longueur);

    // 3. Emprunt mutable (&mut)
    let mut langage = String::from("Rust");
    ajouter_extension(&mut langage, " 2024 Edition");
    println!("Chaine modifiee en place : {}", langage);

    // 4. Portee lexicale (Scope) et liberation automatique de la memoire (Drop)
    {
        let variable_locale = String::from("Variable temporaire en memoire");
        println!("Dans le scope enfant : {}", variable_locale);
    } // variable_locale sort du scope et sa memoire est liberee sans garbage collector
}

fn calculer_longueur(texte: &String) -> usize {
    texte.len()
}

fn ajouter_extension(chaine: &mut String, extension: &str) {
    chaine.push_str(extension);
}
