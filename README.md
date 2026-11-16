# Guide Complet du Langage Rust - Fondamentaux, Typage et Systeme d'Ownership

Projet pedagogique exhaustif dedie a l'apprentissage pragmatique du langage systeme Rust, de la gestion de la memoire sans ramasse-miettes (Garbage Collector) jusqu'aux constructions avancees du typage statique moderne.

Ce depot regroupe des implementations commentees et structurees illustrant les concepts fondamentaux qui garantissent la robustesse et la securite memoire de Rust a la compilation.

## Principes Fondamentaux de Rust

Rust est un langage de programmation systeme moderne concu pour offrir des performances equivalentes au C/C++ tout en eliminant les erreurs d'acces memoire (segmentation faults, dangling pointers, data races) grace a son systeme unique de propriete (Ownership) et de verificateur d'emprunts (Borrow Checker).

## Modules et Concepts Abordes

Le projet est organise en modules thematiques dans le dossier source :

### 1. Fondamentaux et Syntaxe (src/general.rs)

Ce module couvre l'ensemble des bases du langage necessaires a l'ecriture d'un code fiable :

- Immuabilite par defaut : les variables sont immuables (`let`) a moins d'etre explicitement declarees mutables (`let mut`).
- Typage statique et inference : entiers signes (`i32`, `isize`), flottants (`f64`), booleens (`bool`) et caracteres Unicode (`char` occupant 4 octets).
- Gestion des chaines de caracteres :
  - `&str` : references immuables a des tranches de chaines litterales stockees en memoire en lecture seule.
  - `String` : structures de chaines dynamiques allouees sur le tas (heap), redimensionnables et mutables.
- Structures de controle : conditions `if`, `else if`, `else` et filtrage par motif exhaustif avec `match`.
- Iterations et boucles :
  - Boucle conditionnelle `while`.
  - Boucle infinie `loop` avec point de sortie controle `break` et saut `continue`.
  - Boucle `for` avec intervalle range (`0..=3`) et parcours securise de tableaux via `.iter()`.
- Expressions et fonctions : distinction fondamentale entre les instructions terminant par un point-virgule et les expressions de retour implicites sans point-virgule dans les blocs et fonctions.
- Structures de donnees (`struct`) et blocs d'implementation (`impl`) :
  - Definition de structures nommees (`User`, `Rectangle`).
  - Methodes d'instance avec passage de la reference `&self` (calcul de surface `area`).
  - Fonctions associees ou constructeurs statiques (creation de carres avec `Rectangle::square`).
- Enumerations et Pattern Matching (`enum`) :
  - Definition d'enumerations parametrables (`TypePokemon`).
  - Deconstruction complete et exhaustive via `match` sans cas non couvert.
- Gestion robuste de l'absence de valeur (`Option<T>`) :
  - Elimination des pointeurs nuls (`null` / `nil`) au profit du type standard `Option<T>` (`Some(valeur)` ou `None`).
  - Fonctions de securisation (division protegee contre le zero) et utilisation de `.unwrap_or()` pour definir des valeurs de repli.

### 2. Modele d'Ownership et d'Emprunt (src/ownership.rs)

Ce module explicite le coeur architectural de Rust qui remplace le ramasse-miettes :

- Regle d'or de l'Ownership : chaque valeur en memoire possede une et une seule variable proprietaire (owner). Lorsque le proprietaire sort de son bloc de portee (scope), la memoire est instantanement liberee via le destructeur automatique (`drop`).
- Semantique de deplacement (Move) : l'affectation d'une donnee allouee sur le tas transfere la propriete, invalidant la variable source pour empecher les doubles desallocations (double-free).
- Clonage explicite (`.clone()`) : duplication profonde de la memoire sur le tas quand une copie reelle est souhaitee.
- Emprunt immuable (Borrowing `&T`) : passage d'une reference en lecture seule a une fonction sans perte de propriete (`calculer_longueur`). Plusieurs emprunts immuables peuvent coexister simultanement.
- Emprunt mutable (`&mut T`) : passage d'une reference autorisant la modification en place (`ajouter_extension`). Rust impose qu'il n'y ait qu'une seule reference mutable active a la fois pour une ressource donnee, interdisant formellement les acces concurrents non synchronises (data races).

## Structure des Fichiers

- `Cargo.toml` : Manifeste du projet Rust precisant le nom, la version et l'edition (Edition 2024).
- `src/main.rs` : Point d'entree executant successivement les modules d'apprentissage.
- `src/general.rs` : Syntaxe generale, structures, enums, pattern matching et gestion des options.
- `src/ownership.rs` : Demonstrations detaillees de l'ownership, du move, de l'emprunt et de la portee.

## Prerequis et Execution

### Prerequis

- Rust et son gestionnaire d'outils Cargo (installable via rustup)

### Commandes Utiles

1. Cloner le depot :

```bash
git clone https://github.com/maximedefaleouna/Rust.git
cd Rust
```

2. Verifier la conformite du code sans compiler :

```bash
cargo check
```

3. Compiler le projet en mode developpement :

```bash
cargo build
```

4. Executer le programme complet :

```bash
cargo run
```

5. Compiler pour la production avec optimisations maximales :

```bash
cargo build --release
```

## Auteur

- Concepteur : Maxime DEFALEOUNA ([@maximedefaleouna](https://github.com/maximedefaleouna))

## Licence

Ce projet est distribue sous licence MIT. Libre a vous de l'utiliser a des fins pedagogiques et de perfectionnement en programmation systeme.
