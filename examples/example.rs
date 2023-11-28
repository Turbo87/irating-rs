fn main() {
    let race_results = vec![
        ("Driver 1", 1, 3203, true).into(),
        ("Driver 2", 2, 3922, true).into(),
        ("Driver 3", 3, 2974, true).into(),
        ("Driver 4", 4, 1739, true).into(),
        ("Driver 5", 5, 1250, true).into(),
        ("Driver 6", 6, 2588, false).into(),
    ];

    let results = irating::calculate(race_results);
    for result in results {
        println!(
            "#{}  {}: {} -> {} ({}{})",
            result.race_result.finish_rank,
            result.race_result.driver,
            result.race_result.start_irating,
            result.new_irating,
            if result.irating_change > 0. { "+" } else { "" },
            result.irating_change
        );
    }
}
