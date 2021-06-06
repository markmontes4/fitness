use fitapp::Intensity::*;

fn main() {
    let filename = "data.txt";
    let mut data = "Hello from main.\n";
    

    let mut pwo = fitapp::Workout {
        calories_burned: 360,
        pushup_count: 30,
        situp_count: 30,
        squat_count: 39,
    };

    let mut cwo = fitapp::Workout {
        calories_burned: 500,
        pushup_count: 35,
        situp_count: 35,
        squat_count: 40,
    };

    let mut u1 = fitapp::User::build_user("Markaveliii", "mam25@pdx.edu", 1, cwo, pwo);
    fitapp::writefile(&u1,filename)
        .expect("Could not write to file.");
    let file_cont = fitapp::readfile(&u1, filename)
        .expect("Did not readfile");

    let file_cont: Vec<String> = file_cont.split_whitespace().map(str::to_string).collect();
    
    u1 = fitapp::parse_file_cont(file_cont);

    u1.recommend_workout(HEAVY);
    println!("{:?}", u1);

        
}
