pub fn main(){
    println!("ownership");

    // let a = String::from("Hello") ;
    // let b = a ;

    // println!("{}", a); erreur

    // let a = 1 ;
    // {
    //     let b =2 ;
    // } // a et b sont pas du même scope

    let mut a : String = String::from("DEFALEOUNA");
    // let result = longeure(a); // le move de a se fait
    // let b: &String = &a ; //on unprunt mais c'est non mutable
    let b = &mut a ; // un emprunt mutable
    b.push_str("SS");
    let result = longeure(b); 
    println!("{}", result);
    println!("{}", a);
    // println!("{}", b);


  
}

fn longeure( a : &String)->usize{
    a.len()
}