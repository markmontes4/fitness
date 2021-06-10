#[cfg(test)]
mod fitapp_tests {
    #[test]
    fn test_structs() {
        let mut u1 = fitapp::User::new("markmontes", "mam25@pdx.edu");
        let wo = fitapp::Workout::new(10, 10, 10);
        u1.current_workout = wo;
        assert_eq!(u1.current_workout.pushup_count, 10);
        assert_eq!(u1.current_workout.situp_count, 10);
        assert_eq!(u1.current_workout.squat_count, 10);
    }

    //Test to check validity of recommend workout function.
    #[test]
    fn test_recommend() {
        let mut u1 = fitapp::User::new("markmontes", "mam25@pdx.edu");
        u1.previous_workout = fitapp::Workout::new(20, 20, 20);

        u1.recommend_workout(fitapp::Intensity::HEAVY); //Adds 10 reps for each exercise if a current workout doesn't exist already.
        assert_eq!(u1.current_workout.pushup_count, 30);
        assert_eq!(u1.current_workout.situp_count, 30);
        assert_eq!(u1.current_workout.squat_count, 30);
    }

    //Test to check validity of read and writes to file into a User.
    #[test]
    fn test_readwrite() {
        let test_file = "test.txt";
        let mut u1 = fitapp::User::new("markmontes", "mam25@pdx.edu");
        let mut u2 = fitapp::User::new(Default::default(), Default::default());
        u1.current_workout = fitapp::Workout::new(10, 20, 30);
        u1.writefile(test_file)
            .expect("Could not read file for some reason...");
        u2.readfile(test_file);

        assert_eq!(u1.username, u2.username);
        assert_eq!(u1.email, u2.email);
        assert_ne!(u1.sign_in_count, u2.sign_in_count);
        assert_eq!(u1.current_workout, u2.previous_workout);
    }
}
