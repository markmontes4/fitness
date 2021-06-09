use fitapp::*;
//Creates a user, makes a workout with 20 20 20 pushups situps squats respectively, and then starts the workout.
fn main(){
    let mut u1 = User::new("Jane Smith","jsmith@yahoo.com");
    u1.current_workout = Workout::new(20,20,20);
    let mut plan = Plan::new();
    plan.create_plan(&mut u1.current_workout);
    plan.start().expect("Could not start plan");
    println!("--Results--\nCalories Burned: {}\nPushups: {}\nSitups: {}\nSquats: {}\nWorkout Duration: {:?}\n",
             u1.current_workout.calories_burned, u1.current_workout.pushup_count, u1.current_workout.situp_count,
             u1.current_workout.squat_count, u1.current_workout.duration);
}