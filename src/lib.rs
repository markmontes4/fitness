use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::*;
use std::thread;
use std::time::Duration;

const CAL_PER_PUSHUP: f32 = 0.45;
const TIME_PER_PUSHUP: u64 = 2;
const CAL_PER_SITUP: f32 = 0.15;
const TIME_PER_SITUP: u64 = 3;
const CAL_PER_SQUAT: f32 = 0.32;
const TIME_PER_SQUAT: u64 = 3;
const REST_INTERVALS: u64 = 60;

#[derive(Eq, PartialEq)]
pub enum Intensity {
    LIGHT,
    MEDIUM,
    HEAVY,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Plan {
    pub situp_plan: (u64, u64, u64),
    pub pushup_plan: (u64, u64, u64),
    pub squat_plan: (u64, u64, u64),
    pub duration: Duration,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Workout {
    pub calories_burned: u64,
    pub pushup_count: u64,
    pub situp_count: u64,
    pub squat_count: u64,
    pub duration: Duration,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub current_workout: Workout,
    pub previous_workout: Workout,
}

impl Workout {
    pub fn new(pu_count: u64, su_count: u64, sq_count: u64) -> Workout {
        Workout {
            calories_burned: ((pu_count as f32 * CAL_PER_PUSHUP)
                + (su_count as f32 * CAL_PER_SITUP)
                + (sq_count as f32 * CAL_PER_SQUAT)) as u64,
            pushup_count: pu_count,
            situp_count: su_count,
            squat_count: sq_count,
            duration: Duration::new(
                (pu_count * TIME_PER_PUSHUP)
                    + (su_count * TIME_PER_SITUP)
                    + (sq_count * TIME_PER_SQUAT),
                0,
            ),
        }
    }

    pub fn light_workout(&mut self, prev_wo: &Workout) {
        self.calories_burned = prev_wo.calories_burned;
        self.pushup_count = prev_wo.pushup_count;
        self.situp_count = prev_wo.pushup_count;
        self.squat_count = prev_wo.squat_count;
        self.duration = prev_wo.duration;
    }
    pub fn med_workout(&mut self, prev_wo: &Workout) {
        let add_cal = (5.0 * (CAL_PER_PUSHUP + CAL_PER_SITUP + CAL_PER_SQUAT)) as u64;
        self.calories_burned = prev_wo.calories_burned + add_cal;
        self.pushup_count = prev_wo.pushup_count + 5;
        self.situp_count = prev_wo.pushup_count + 5;
        self.squat_count = prev_wo.squat_count + 5;
        self.duration = prev_wo.duration + Duration::new(600, 0);
    }
    pub fn heavy_workout(&mut self, prev_wo: &Workout) {
        let add_cal = (10.0 * (CAL_PER_PUSHUP + CAL_PER_SITUP + CAL_PER_SQUAT)) as u64;
        self.calories_burned = prev_wo.calories_burned + add_cal;
        self.pushup_count = prev_wo.pushup_count + 10;
        self.situp_count = prev_wo.pushup_count + 10;
        self.squat_count = prev_wo.squat_count + 10;
        self.duration = prev_wo.duration + Duration::new(1200, 0);
    }
    pub fn no_workout(&self) -> bool {
        self.calories_burned == 0
    }
}

impl User {
    pub fn new(username: &str, email: &str) -> User {
        User {
            username: username.to_string(),
            email: email.to_string(),
            sign_in_count: Default::default(),
            current_workout: Default::default(),
            previous_workout: Default::default(),
        }
    }

    pub fn build_user(
        uname: &str,
        em: &str,
        sic: u64,
        curr_workout: Workout,
        prev_workout: Workout,
    ) -> User {
        User {
            username: uname.to_string(),
            email: em.to_string(),
            sign_in_count: sic,
            current_workout: curr_workout,
            previous_workout: prev_workout,
        }
    }

    pub fn recommend_workout(&mut self, intensity: Intensity) {
        if !self.current_workout.no_workout() {
            return;
        }
        match intensity {
            Intensity::LIGHT => self.current_workout.light_workout(&self.previous_workout),
            Intensity::MEDIUM => self.current_workout.med_workout(&self.previous_workout),
            Intensity::HEAVY => self.current_workout.heavy_workout(&self.previous_workout),
        };
    }

    pub fn readfile(&mut self, filename: &str) {
        let filec: String = std::fs::read_to_string(filename).expect("Coult not read file");
        let file_cont: Vec<String> = filec.split_whitespace().map(str::to_string).collect();
        self.parse_file_cont(file_cont);
    }

    pub fn parse_file_cont(&mut self, cont: Vec<String>) {
        if cont.is_empty() {
            self.current_workout = Workout::new(10, 10, 10);
            self.sign_in_count = 1;
            return;
        }
        self.username = cont[0].clone();
        self.email = cont[1].clone();
        self.sign_in_count = cont[2]
            .parse::<u64>()
            .expect("Could not parse sign in count")
            + 1;
        self.current_workout = Default::default();
        self.previous_workout.calories_burned = cont[3]
            .parse::<u64>()
            .expect("Could not parse calories burned");
        self.previous_workout.pushup_count = cont[4]
            .parse::<u64>()
            .expect("Could not parse push up count");
        self.previous_workout.situp_count = cont[5]
            .parse::<u64>()
            .expect("Could not parse sit up count");
        self.previous_workout.squat_count =
            cont[6].parse::<u64>().expect("Could not parse squat count");
        self.previous_workout.duration =
            Duration::new(cont[7].parse::<u64>().expect("Could not parse duration"), 0);
    }

    pub fn writefile(&self, filename: &str) -> Result<()> {
        if self.current_workout.no_workout() {
            panic!("No workout to record.");
        }

        let mut file = OpenOptions::new().write(true).open(filename)?;
        file.write_all(self.username.to_string().as_bytes())
            .expect("Username read failed");
        file.write_all(format!("\n{}", self.email.to_string()).as_bytes())
            .expect("Email read failed");
        file.write_all(format!("\n{}", self.sign_in_count.to_string()).as_bytes())
            .expect("Sign in count read failed");
        file.write_all(
            format!("\n{}", self.current_workout.calories_burned.to_string()).as_bytes(),
        )
        .expect("Calories burned count read failed");
        file.write_all(format!("\n{}", self.current_workout.pushup_count.to_string()).as_bytes())
            .expect("Pushup count read failed");
        file.write_all(format!("\n{}", self.current_workout.situp_count.to_string()).as_bytes())
            .expect("Situp count read failed");
        file.write_all(format!("\n{}", self.current_workout.squat_count.to_string()).as_bytes())
            .expect("Squat count read failed");
        file.write_all(
            format!(
                "\n{}",
                self.current_workout.duration.as_secs_f32().to_string()
            )
            .as_bytes(),
        )
        .expect("Duration read failed");
        Ok(())
    }
}

impl Plan {
    pub fn new() -> Plan {
        Plan {
            situp_plan: Default::default(),
            pushup_plan: Default::default(),
            squat_plan: Default::default(),
            duration: Default::default(),
        }
    }

    pub fn create_plan(&mut self, cwo: &mut Workout) {
        if cwo.no_workout() {
            panic!("No workout provided")
        }
        match cwo.situp_count {
            1..=10 => {
                self.situp_plan = (1, cwo.situp_count, 1);
                self.duration += Duration::new(
                    cwo.situp_count * TIME_PER_SITUP + (self.situp_plan.2 * 60),
                    0,
                );
            }
            11..=20 => {
                self.situp_plan = (2, cwo.situp_count / 2, 2);
                self.duration += Duration::new(
                    cwo.situp_count * TIME_PER_SITUP + (self.situp_plan.2 * 60),
                    0,
                );
            }
            21..=50 => {
                self.situp_plan = (3, cwo.situp_count / 3, 3);
                self.duration += Duration::new(
                    cwo.situp_count * TIME_PER_SITUP + (self.situp_plan.2 * 60),
                    0,
                );
            }
            51..=100 => {
                self.situp_plan = (4, cwo.situp_count / 4, 4);
                self.duration += Duration::new(
                    cwo.situp_count * TIME_PER_SITUP + (self.situp_plan.2 * 60),
                    0,
                );
            }
            101..=150 => {
                self.situp_plan = (5, cwo.situp_count / 5, 5);
                self.duration += Duration::new(
                    cwo.situp_count * TIME_PER_SITUP + (self.situp_plan.2 * 60),
                    0,
                );
            }
            151..=200 => {
                self.situp_plan = (6, cwo.situp_count / 6, 6);
                self.duration += Duration::new(
                    cwo.situp_count * TIME_PER_SITUP + (self.situp_plan.2 * 60),
                    0,
                );
            }
            _ => println!("Too many situps"),
        }
        match cwo.pushup_count {
            1..=10 => {
                self.pushup_plan = (1, cwo.pushup_count, 1);
                self.duration += Duration::new(
                    cwo.pushup_count * TIME_PER_PUSHUP + (self.pushup_plan.2 * 60),
                    0,
                );
            }
            11..=20 => {
                self.pushup_plan = (2, cwo.pushup_count / 2, 2);
                self.duration += Duration::new(
                    cwo.pushup_count * TIME_PER_PUSHUP + (self.pushup_plan.2 * 60),
                    0,
                );
            }
            21..=50 => {
                self.pushup_plan = (3, cwo.pushup_count / 3, 3);
                self.duration += Duration::new(
                    cwo.pushup_count * TIME_PER_PUSHUP + (self.pushup_plan.2 * 60),
                    0,
                );
            }
            51..=100 => {
                self.pushup_plan = (4, cwo.pushup_count / 4, 4);
                self.duration += Duration::new(
                    cwo.pushup_count * TIME_PER_PUSHUP + (self.pushup_plan.2 * 60),
                    0,
                );
            }
            101..=150 => {
                self.pushup_plan = (5, cwo.pushup_count / 5, 5);
                self.duration += Duration::new(
                    cwo.pushup_count * TIME_PER_PUSHUP + (self.pushup_plan.2 * 60),
                    0,
                );
            }
            151..=200 => {
                self.pushup_plan = (6, cwo.pushup_count / 6, 6);
                self.duration += Duration::new(
                    cwo.pushup_count * TIME_PER_PUSHUP + (self.pushup_plan.2 * 60),
                    0,
                );
            }
            _ => println!("Too many pushups"),
        }
        match cwo.squat_count {
            1..=10 => {
                self.squat_plan = (1, cwo.squat_count, 1);
                self.duration += Duration::new(
                    cwo.squat_count * TIME_PER_SQUAT + (self.squat_plan.2 * 60),
                    0,
                );
            }
            11..=20 => {
                self.squat_plan = (2, cwo.squat_count / 2, 2);
                self.duration += Duration::new(
                    cwo.squat_count * TIME_PER_SQUAT + (self.squat_plan.2 * 60),
                    0,
                );
            }
            21..=50 => {
                self.squat_plan = (3, cwo.squat_count / 3, 3);
                self.duration += Duration::new(
                    cwo.squat_count * TIME_PER_SQUAT + (self.squat_plan.2 * 60),
                    0,
                );
            }
            51..=100 => {
                self.squat_plan = (4, cwo.squat_count / 4, 4);
                self.duration += Duration::new(
                    cwo.squat_count * TIME_PER_SQUAT + (self.squat_plan.2 * 60),
                    0,
                );
            }
            101..=150 => {
                self.squat_plan = (5, cwo.squat_count / 5, 5);
                self.duration += Duration::new(
                    cwo.squat_count * TIME_PER_SQUAT + (self.squat_plan.2 * 60),
                    0,
                );
            }
            151..=200 => {
                self.squat_plan = (6, cwo.squat_count / 6, 6);
                self.duration += Duration::new(
                    cwo.squat_count * TIME_PER_SQUAT + (self.squat_plan.2 * 60),
                    0,
                );
            }
            _ => println!("Too many squats"),
        }
        cwo.duration = self.duration;
    }
    pub fn start(&self) -> Result<()> {
        if self.situp_plan.0 == 0 || self.pushup_plan.0 == 0 || self.squat_plan.0 == 0 {
            panic!("No plan to start")
        }

        println!("\nGet ready for pushups...");
        for i in 0..self.pushup_plan.0 {
            println!("\nSet: {}\n----------READY!----------",i+1);
            thread::sleep(Duration::new(2, 0));
            println!("----------SET!----------");
            thread::sleep(Duration::new(2, 0));
            println!("----------GO!----------");
            thread::sleep(Duration::new(2, 0));
            Plan::start_helper(self.pushup_plan);
        }

        println!("\nGet ready for situps...");
        for i in 0..self.situp_plan.0 {
            println!("\nSet: {}\n----------READY!----------",i+1);
            thread::sleep(Duration::new(2, 0));
            println!("----------SET!----------");
            thread::sleep(Duration::new(2, 0));
            println!("----------GO!----------");
            thread::sleep(Duration::new(2, 0));
            Plan::start_helper(self.situp_plan);
        }

        println!("\nGet ready for squats...");
        for i in 0..self.squat_plan.0 {
            println!("\nSet: {}\n----------READY!----------",i+1);
            thread::sleep(Duration::new(2, 0));
            println!("----------SET!----------");
            thread::sleep(Duration::new(2, 0));
            println!("----------GO!----------");
            thread::sleep(Duration::new(2, 0));
            Plan::start_helper(self.squat_plan);
        }

        Ok(())
    }
    fn start_helper(plan: (u64, u64, u64)) {
        for j in 0..plan.1 {
            println!("Count: {}", j + 1);
            //thread::sleep(Duration::new(TIME_PER_PUSHUP,0));
        }
        println!("==========SET DONE==========");
        println!("Now rest for {} seconds", REST_INTERVALS);
        for r in 0..REST_INTERVALS {
            if let 55..=REST_INTERVALS = r {println!("Rest ends in {}...", REST_INTERVALS-r)};
            thread::sleep(Duration::new(1, 0));
        }
    }
}
