# Fitapp Library
The Fitapp library can be used for an application that needs a way to structure a persons workout routine. The library focuses on 3 major exercises: pushups, situps, and squats. It allows the creation of a User that holds information like username, email, current workout routine, and previous workout routine. The library can also recommend a workout based on intensity level: Light, Medium, Hard. It can also create a workout Plan by setting up the amount of sets and reps for an exercise, number of breaks, and a total duration. This library is made for progressive workouts, and allows to save a previous workout, and load it into a current workout. The library also includes an in-terminal program that allows you to start a workout within a terminal session.

## Usage
To use the library, you can download the project from Github via: https://github.com/markmontes4/fitness. In the CARGO.toml, ensure to include the correct dependencies. From there, you can use the fitapp namespace to access User, Workout, and Plan structures. 

<br/> Sample Run
```Rust
fn main(){
    let mut u1 = fitapp::User::new("Mark Montes","mam25@pdx.edu");
    let  wo = fitapp::Workout::new(10,10,10);
    u1.current_workout = wo;
    assert_eq!(u1.current_workout.pushup_count,10);
}
```

Run a sample run of a workout by typing 'cargo run --example workout_202020'

## Results
I'm pretty satisfied with how in turned out. My initial proposal seemed align with what I have. I wasn't able to implement certain stats like heart rate, age, and weight because of not being an expert in the fitness field, I didn't know how to factor them into the workout results/recommendations.
<br/>The Things I Struggled With:
* Rust
* Redundant code... Could be written more smoothly
* Pleasing clippy
* Calculating statistical things like calories and time.

Things I'd Like to Add
* Other stats like age, weight, and heartrate to give a more accurate calorie count.
* Make a sample fitness app with a GUI.



## LICENSE
fitapp is distributed under the terms of MIT License

See [MIT-LICENSE](MIT-LICENSE)