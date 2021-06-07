use fitapp::Intensity::*;

fn main() {
    let mut u1 = fitapp::User::new("Markaveliii","mam25@pdx.edu");
    let filename = "user1.txt";

    u1.readfile(filename);
    println!("{:?}",u1);
    u1.recommend_workout(MEDIUM);

    u1.writefile(filename)
        .expect("Could not write to file.");

    let mut p1: fitapp::Plan = fitapp::Plan::new();
    p1.create_plan(&u1.current_workout);
    println!("{:?}", p1);
        
}
