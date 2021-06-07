use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::*;

const CAL_PER_PUSHUP:f32 = 0.45;
const CAL_PER_SITUP:f32 = 0.15;
const CAL_PER_SQUAT:f32 = 0.32;


#[derive(Eq,PartialEq)]
pub enum Intensity {
    LIGHT,
    MEDIUM,
    HEAVY,
}

#[derive(Debug, Clone, Default)]
pub struct Workout {
    pub calories_burned: u64,
    pub pushup_count: u64,
    pub situp_count: u64,
    pub squat_count: u64,
}

#[derive(Debug, Clone, Default)]
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub current_workout: Workout,
    pub previous_workout: Workout,
}

impl Workout {
    pub fn new() -> Workout{
        Workout{
            calories_burned: Default::default(),
            pushup_count: Default::default(),
            situp_count: Default::default(),
            squat_count: Default::default(),
        }
    }
    pub fn light_workout(&mut self, prev_wo: &Workout){
        self.calories_burned = prev_wo.calories_burned;
        self.pushup_count = prev_wo.pushup_count;
        self.situp_count = prev_wo.pushup_count;
        self.squat_count = prev_wo.squat_count;
    }
    pub fn med_workout(&mut self, prev_wo: &Workout){
        let add_cal = (5.0*(CAL_PER_PUSHUP+CAL_PER_SITUP+CAL_PER_SQUAT)) as u64;
        self.calories_burned = prev_wo.calories_burned+add_cal;
        self.pushup_count = prev_wo.pushup_count+5;
        self.situp_count = prev_wo.pushup_count+5;
        self.squat_count = prev_wo.squat_count+5;
    }
    pub fn heavy_workout(&mut self, prev_wo: &Workout){
        let add_cal = (10.0*(CAL_PER_PUSHUP+CAL_PER_SITUP+CAL_PER_SQUAT)) as u64;
        self.calories_burned = prev_wo.calories_burned+add_cal;
        self.pushup_count = prev_wo.pushup_count+10;
        self.situp_count = prev_wo.pushup_count+10;
        self.squat_count = prev_wo.squat_count+10;
    }
}

impl User {
    pub fn new(username: &str, email: &str)-> User{
        User{
            username: username.to_string(),
            email: email.to_string(),
            sign_in_count: Default::default(),
            current_workout: Default::default(),
            previous_workout: Default::default(),
        }
    }

    pub fn build_user(uname: &str, em: &str, sic: u64, curr_workout: Workout, prev_workout: Workout) -> User {
        User {
            username: uname.to_string(),
            email: em.to_string(),
            sign_in_count: sic,
            current_workout: curr_workout,
            previous_workout: prev_workout,
        }
    
    }

    pub fn recommend_workout(&mut self, intensity: Intensity){
        match intensity{
            Intensity::LIGHT => self.current_workout.light_workout(&self.previous_workout),
            Intensity::MEDIUM => self.current_workout.med_workout(&self.previous_workout),
            Intensity::HEAVY => self.current_workout.heavy_workout(&self.previous_workout),
            _ => panic!("Workout does not exist"),
        };
    }

    pub fn readfile(&mut self, filename: &str) {
        let filec:String = std::fs::read_to_string(filename)
            .expect("Coult not read file");
        let file_cont:Vec<String> = filec.split_whitespace().map(str::to_string).collect();
        self.parse_file_cont(file_cont); 
    }

    pub fn parse_file_cont(&mut self, cont: Vec<String>){
        self.username = cont[0].clone();
        self.email =  cont[1].clone();
        self.sign_in_count =  cont[2].parse::<u64>().expect("Could not parse sign in count")+1;
        self.current_workout = Default::default();
        self.previous_workout.calories_burned = cont[3].parse::<u64>().expect("Could not parse calories burned");
        self.previous_workout.pushup_count = cont[4].parse::<u64>().expect("Could not parse push up count");
        self.previous_workout.situp_count = cont[5].parse::<u64>().expect("Could not parse sit up count");
        self.previous_workout.squat_count = cont[6].parse::<u64>().expect("Could not parse squat count");
    }

}


pub fn writefile(user: &User, filename: &str) -> Result <()> {
    let mut file = OpenOptions::new()
        .write(true)
        .open(filename)?;
    file.write_all(format!("{}",user.username).as_bytes()).expect("Username read failed");
    file.write_all(format!("\n{}",user.email).as_bytes()).expect("Email read failed");
    file.write_all(format!("\n{}",user.sign_in_count.to_string()).as_bytes()).expect("Sign in count read failed");
    file.write_all(format!("\n{}",user.current_workout.calories_burned.to_string()).as_bytes()).expect("Calories burned count read failed");
    file.write_all(format!("\n{}",user.current_workout.pushup_count.to_string()).as_bytes()).expect("Pushup count read failed");
    file.write_all(format!("\n{}",user.current_workout.situp_count.to_string()).as_bytes()).expect("Situp count read failed");
    file.write_all(format!("\n{}",user.current_workout.squat_count.to_string()).as_bytes()).expect("Squat count read failed");
    Ok(())
}



