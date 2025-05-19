mod c_bindings;

fn main() {
    let result = c_bindings::add(5, 7);
    println!("Le résultat de l'addition est : {}", result);
}
