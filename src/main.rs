use fitapp::Intensity::*;

fn main() {
    let filename = "data.txt";

    let mut u1 = fitapp::User::new("Markaveliii","mam25@pdx.edu");
    u1.readfile(filename);
    u1.recommend_workout(HEAVY);
    println!("{:?}", u1);
    fitapp::writefile(&u1,filename)
        .expect("Could not write to file.");
        
}
