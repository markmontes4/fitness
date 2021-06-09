#[cfg(test)]
mod fitapp_tests {
    #[test]
    fn test_structs() {
        let mut u1 = fitapp::User::new("Mark Montes", "mam25@pdx.edu");
        let wo = fitapp::Workout::new(10, 10, 10);
        u1.current_workout = wo;
        assert_eq!(u1.current_workout.pushup_count, 10);
        assert_eq!(u1.current_workout.situp_count, 10);
        assert_eq!(u1.current_workout.squat_count, 10);
    }
    #[test]
    fn test_recommend() {
        let mut u1 = fitapp::User::new("Mark Montes", "mam25@pdx.edu");
        u1.previous_workout = fitapp::Workout::new(20, 20, 20);

        &mut u1.recommend_workout(fitapp::Intensity::HEAVY); //Adds 10 reps for each exercise if a current workout doesn't exist already.
        assert_eq!(u1.current_workout.pushup_count, 30);
        assert_eq!(u1.current_workout.situp_count, 30);
        assert_eq!(u1.current_workout.squat_count, 30);

    }
    
}
