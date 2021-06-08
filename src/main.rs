use fitapp::Intensity::*;

fn main() {
    let mut u1 = fitapp::User::new("Mark Montes", "mam25@pdx.edu"); //Creates a user account
    u1.current_workout = fitapp::Workout::new(20, 20, 20); //Creates a workout for user with base count 20 20 20 pushup situp squats respectively.
    u1.recommend_workout(HEAVY);
    let mut p1 = fitapp::Plan::new(); //Initiates a plan
    println!("{:?}", u1.current_workout);
    p1.create_plan(&mut u1.current_workout); //and creates a plan for users workout as tuple (set,reps,breaks) for each activity.
    println!("{:?}", u1.current_workout);
    println!("{:?}", p1);
    p1.start().expect("Did not start");
}
