pub fn main() {
   
    println!("RUST program");

    let x: i32 = 5; // non mutable
    let mut y: i32 = 4 ;
    let z: i32 = y * x ;
    y = 10;
    println!("{}", x);
    println!("{}",y);
    println!("{}", z);

    let (j , k) = (15 , 30);
    println!("{}", j);
    println!("{}", k);


// les chaines de caractères

    let mut texte: &'static str = "les autres types" ; //Juste lire une chaîne constante
    println!("{}", texte);

    texte = "hello";
    println!("{}", texte);

    let  mut phrase: String = String::from("une phrase mutable"); //Construire ou modifier dynamiquement une chaîne
    phrase.push_str("Interressant");
    phrase = "Phrase netoyé".to_string();
    println!("{}", phrase);
    

// les réels

    let pi: f64 = 3.141592 ;
    let demi: f64 = 2.71;
    println!("valeure de pi : {}",pi);
    println!("valeure de demi : {}",demi);


// les booleans
    let vrai: bool = true ;
    println!("{}", vrai);


// les caractères char

let zed: char = 'z' ;
println!("la lettre zed {} : " ,zed);

// les if/else
y=5;
if x > y {
    println!(" y est superieur a x") ;
}else if x == y {
    println!("x et y sont égaux ");
}
else {
    println!("x est superieur a y") ;
}

// match : equivalent de switch

let y = 4 ;
match y {
    1 => println!("y en lettre : Un"),
    2 => println!("y en lettre : Deux"),
    3 => println!("y en lettre : Trois"),
    _ => println!("Acheter le livre pour tous apprendre")
}
// les boucles 

println!("Boucle while");
let  mut n = -6 ;
while n < 5 {
    n +=1;
    if n==0{
        print!(".....");
        continue;
    }
    print!("{}", n) ;
}

println!("");
println!("Boucle loop");
let mut n: i32 = 0 ;
loop {
    
    print!(" {}", n);

    if n == 9 {
        break;
    }
    n= n+1;
    
}

println!("");
println!("Boucle for parcourir un tableau");
let numbers: [i32; 10] = [0,1,2,3,4,5,6,7,8,9];
for number in numbers.iter()  {
    print!(" {}", number);
}


println!("");
println!("Boucle for ");
for n in 0..=9 { // ou 0..10
    print!(" {}", n);
}


// les fonctions en rust
println!("");

let block: i32 = {
    let a: i32 = 100 ;
    let b: i32 = 400;
    a+b
};
println!("block = {}", block);
fn somme (mut a : i32 , b : i32) -> i32{
    a = a + 0; // car nous avons dit a est mutable
    return a+b;
}

fn product (a : i32 , b : i32)-> i32{
    a*b
}

println!("la somme de 10 + 20 est egale à : {} " , somme(10, 20));
println!("la produit de 10 avec 20 est egale à : {} " , product(10, 20));

// les struct en rust car c'est pas orineté objet

let mut admin = User{
    nom : String::from("ALI"),
    prenom : String::from("Tchagnime"),
    age : 21
} ;
admin.prenom = String::from("Grégoire");
          // let admin2 = User{..admin}; // admin dans Admin2
         // let admin = User{nom,prenom,age};  // possible aussi 

println!("{:?}", admin);

let carre = Rectange::square(10);
println!("{:?}", carre);
println!("{}", carre.area());

// les enum en rusts

let pikachu = Pokemon{ name: String::from("Pikachu") , kind : PokemonKind::Electric};
println!("{:?}", pikachu);
pikachu.attack();


// Les options en Rust. permet de gerer l'incertude du type d'une variable
let div = safe_division(20, 2);
let div2 = safe_division(20, 0);

println!("{:?}", div.unwrap());
println!("{:?}", div2);
// println!("{:?}", div2.unwrap_or(0));
if div2.is_some(){
    println!("Division ok !!");
}

if div2.is_none(){
    println!("On divisise pas par 0");
}

println!("{}", add(1,2, Some(2)));
println!("{}", add(1,2, None));


}


#[derive(Debug)]
struct  User{
    nom : String,
    prenom : String ,
    age : u8
}

#[derive(Debug)]
struct Rectange{ // avec les strucs on definit de nouveaux types
    width : usize,
    height: usize,
}

impl Rectange {
    fn square(width:usize)->Rectange{
        Rectange { width: (width), height: (width) }
    }

    // methode besoin d'une instance
    fn area(&self)->usize{
        self.height * self.height
    }
}



#[derive(Debug , PartialEq)]
enum PokemonKind{
    Fire,Water,Electric
}
#[derive(Debug)]
struct Pokemon{
    name:String,
    kind: PokemonKind
}

impl Pokemon{
    fn attack(&self){
        // if self.kind == PokemonKind::Fire {
        //     println!("Fire attaque");
        // }else if self.kind == PokemonKind::Water {
        //     println!("water attaque")
        // }else if self.kind == PokemonKind::Electric {
        //     println!("Electric attack")
        // }

        match  self.kind {
            PokemonKind::Electric => println!("Electric attack"),
            PokemonKind::Water => println!("water attaque"),
            PokemonKind::Fire => println!("Fire attaque")         
        }

    }
}


fn safe_division ( a:isize , b:isize) -> Option<isize>{
    if b == 0 {
        return  None ;
    }
    return  Some(a/b) ;
}

fn add(a:isize , b:isize , c:Option<isize>)->isize{
    // if c.is_none(){
    //    return  a + b
    // }

    a + b + c.unwrap_or(0)
}