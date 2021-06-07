use fitapp::Intensity::*;

fn main() {
    let mut u1 = fitapp::User::new("Markaveliii","mam25@pdx.edu");
    let filename = "user1.txt";

    u1.readfile(filename);

    u1.recommend_workout(MEDIUM);

    u1.writefile(filename)
        .expect("Could not write to file.");
        
}
