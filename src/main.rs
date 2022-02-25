extern crate rand;
use rand::Rng;
fn main() {
    let mut pass = String::from("");
    for _ in 0..18{
    let i = rand::thread_rng().gen_range(0..2);
    if i ==0{
      
       let stff = vec!["A","B","C","D","E","F","G","H","I","J",
       "K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","a",
       "b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r",
       "s","t","u","v","w","x","y","z","#","$","%","&"];
       let j = rand::thread_rng().gen_range(0..stff.len()-1);
       pass+=&stff[j].to_string();
    }
    else{
        let _k = rand::thread_rng().gen_range(0..10);
        pass +=&(_k.to_string())
    }
}
    println!("{}",pass);

}
