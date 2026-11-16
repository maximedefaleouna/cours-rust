pub fn main() {
    println!("=== FONDAMENTAUX DU LANGAGE RUST ===");

    // 1. Variables et Mutabilite
    let x: i32 = 5; // Immuable par defaut
    let mut y: i32 = 4; // Mutable
    let z: i32 = y * x;
    y = 10;
    println!("Valeur immuable x : {}", x);
    println!("Valeur mutable y modifiee : {}", y);
    println!("Produit initial z : {}", z);

    // Destructuration de tuple
    let (j, k) = (15, 30);
    println!("Tuple decompose : j = {}, k = {}", j, k);

    // 2. Chaines de caracteres : &str (tranche de chaine) vs String (tas alloue dynamiquement)
    let texte_statique: &'static str = "Chaine litterale constante en lecture seule";
    println!("{}", texte_statique);

    let mut phrase_dynamique: String = String::from("Une chaine dynamique");
    phrase_dynamique.push_str(" enrichie en cours d'execution");
    println!("{}", phrase_dynamique);

    // 3. Types numeriques reels, booleens et caracteres
    let pi: f64 = 3.141592;
    let base_e: f64 = 2.71828;
    println!("Flottants : pi = {}, e = {}", pi, base_e);

    let actif: bool = true;
    println!("Booleen : {}", actif);

    let caractere: char = 'z';
    println!("Caractere Unicode (4 octets) : {}", caractere);

    // 4. Structures conditionnelles if / else if / else
    let seuil = 5;
    if x > seuil {
        println!("x ({}) est superieur au seuil ({})", x, seuil);
    } else if x == seuil {
        println!("x ({}) est exactement egal au seuil ({})", x, seuil);
    } else {
        println!("x ({}) est inferieur au seuil ({})", x, seuil);
    }

    // 5. Filtrage par motif (Pattern Matching avec match)
    let code_statut = 2;
    match code_statut {
        1 => println!("Statut : Initialisation"),
        2 => println!("Statut : En cours de traitement"),
        3 => println!("Statut : Termine avec succes"),
        _ => println!("Statut inconnu ou non pris en charge"),
    }

    // 6. Boucles : while, loop et for
    println!("Boucle while :");
    let mut compteur = -3;
    while compteur < 3 {
        compteur += 1;
        if compteur == 0 {
            print!("[point d'inflexion zero] ");
            continue;
        }
        print!("{} ", compteur);
    }
    println!();

    println!("Boucle loop avec break conditionnel :");
    let mut pas: i32 = 0;
    loop {
        print!("{} ", pas);
        if pas >= 5 {
            break;
        }
        pas += 1;
    }
    println!();

    println!("Boucle for sur un tableau et intervalle range :");
    let tableau: [i32; 5] = [10, 20, 30, 40, 50];
    for element in tableau.iter() {
        print!("{} ", element);
    }
    println!();

    for index in 0..=3 {
        print!("#{} ", index);
    }
    println!();

    // 7. Blocs d'expressions et fonctions
    let total_bloc: i32 = {
        let sous_total_a = 100;
        let sous_total_b = 400;
        sous_total_a + sous_total_b // Expression retournee sans point-virgule
    };
    println!("Resultat de l'evaluation de bloc : {}", total_bloc);

    println!("Somme (10 + 20) : {}", somme(10, 20));
    println!("Produit (10 * 20) : {}", produit(10, 20));

    // 8. Structures de donnees (Structs) et methodes associees (impl)
    let mut utilisateur = User {
        nom: String::from("DEFALEOUNA"),
        prenom: String::from("Maxime"),
        age: 21,
    };
    utilisateur.age = 22;
    println!("Utilisateur : {:?}", utilisateur);

    let rectangle = Rectangle::square(10);
    println!("Rectangle cree : {:?}", rectangle);
    println!("Surface calculee : {}", rectangle.area());

    // 9. Enumerations avec donnees et pattern matching
    let pikachu = Pokemon {
        nom: String::from("Pikachu"),
        type_element: TypePokemon::Electrique,
    };
    println!("Pokemon declare : {:?}", pikachu);
    pikachu.declencher_attaque();

    // 10. Gestion robuste des valeurs nulles avec Option<T>
    let division_valide = division_securisee(20, 2);
    let division_par_zero = division_securisee(20, 0);

    match division_valide {
        Some(valeur) => println!("Division reussie : {}", valeur),
        None => println!("Erreur : division impossible"),
    }

    if division_par_zero.is_none() {
        println!("Gestion prevenue : division par zero neutralisee avec succes");
    }

    println!("Addition avec option Some : {}", additionner_option(5, 10, Some(3)));
    println!("Addition avec option None : {}", additionner_option(5, 10, None));
}

// Fonctions
fn somme(a: i32, b: i32) -> i32 {
    a + b
}

fn produit(a: i32, b: i32) -> i32 {
    a * b
}

fn division_securisee(numerateur: isize, denominateur: isize) -> Option<isize> {
    if denominateur == 0 {
        None
    } else {
        Some(numerateur / denominateur)
    }
}

fn additionner_option(a: isize, b: isize, bonus: Option<isize>) -> isize {
    a + b + bonus.unwrap_or(0)
}

// Structures
#[derive(Debug)]
struct User {
    nom: String,
    prenom: String,
    age: u8,
}

#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn square(taille: usize) -> Rectangle {
        Rectangle {
            width: taille,
            height: taille,
        }
    }

    fn area(&self) -> usize {
        self.width * self.height
    }
}

// Enumerations
#[derive(Debug, PartialEq)]
enum TypePokemon {
    Feu,
    Eau,
    Electrique,
}

#[derive(Debug)]
struct Pokemon {
    nom: String,
    type_element: TypePokemon,
}

impl Pokemon {
    fn declencher_attaque(&self) {
        match self.type_element {
            TypePokemon::Electrique => println!("{} lance une attaque electrique foudroyante !", self.nom),
            TypePokemon::Eau => println!("{} declenche un puissant hydrocanon !", self.nom),
            TypePokemon::Feu => println!("{} souffle une tempete de flammes !", self.nom),
        }
    }
}
