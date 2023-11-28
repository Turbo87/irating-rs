fn main() {
    let race_results = vec![
        ("Driver 1", 1, 7526, true).into(),
        ("Driver 2", 2, 5982, true).into(),
        ("Driver 3", 3, 5463, true).into(),
        ("Driver 4", 4, 4279, true).into(),
        ("Driver 5", 5, 4137, true).into(),
        ("Driver 6", 6, 4044, true).into(),
        ("Driver 7", 7, 3891, true).into(),
        ("Driver 8", 8, 3612, true).into(),
        ("Driver 9", 9, 3147, true).into(),
        ("Driver 10", 10, 2823, true).into(),
        ("Driver 11", 11, 2715, true).into(),
        ("Driver 12", 12, 2603, true).into(),
        ("Driver 13", 13, 2512, true).into(),
        ("Driver 14", 14, 2352, false).into(),
        ("Driver 15", 15, 2227, true).into(),
        ("Driver 16", 16, 2195, true).into(),
        ("Driver 17", 17, 2166, true).into(),
        ("Driver 18", 18, 2089, true).into(),
        ("Driver 19", 19, 1773, true).into(),
        ("Driver 20", 20, 1772, true).into(),
        ("Driver 21", 21, 1752, true).into(),
        ("Driver 22", 22, 1748, true).into(),
        ("Driver 23", 23, 1705, true).into(),
        ("Driver 24", 24, 1662, true).into(),
        ("Driver 25", 25, 1622, true).into(),
        ("Driver 26", 26, 1537, true).into(),
        ("Driver 27", 27, 1464, true).into(),
        ("Driver 28", 28, 1203, true).into(),
    ];

    let result = irating::calculate(race_results);
    dbg!(&result);
}
