fn main() {
    let mut u1 = fitapp::User::new("Mark Montes", "mam25@pdx.edu");
    u1.previous_workout = fitapp::Workout::new(20, 20, 20);
    &mut u1.recommend_workout(fitapp::Intensity::HEAVY);
    println!("{:?}", u1);
}
