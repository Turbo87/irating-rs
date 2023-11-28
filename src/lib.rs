use std::f32::consts::LN_2;

#[derive(Debug, Clone)]
pub struct RaceResult<D = ()> {
    pub driver: D,
    pub finish_rank: u32,
    pub start_irating: u32,
    pub started: bool,
}

impl<D> From<(D, u32, u32, bool)> for RaceResult<D> {
    fn from(value: (D, u32, u32, bool)) -> Self {
        RaceResult {
            driver: value.0,
            finish_rank: value.1,
            start_irating: value.2,
            started: value.3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CalculationResult<D = ()> {
    pub race_result: RaceResult<D>,
    pub irating_change: f32,
    pub new_irating: u32,
}

pub fn calculate<D>(race_results: Vec<RaceResult<D>>) -> Vec<CalculationResult<D>> {
    let br1 = 1600. / LN_2;

    let num_registrations = race_results.len();
    let num_starters = race_results.iter().filter(|result| result.started).count();
    let num_non_starters = num_registrations - num_starters;

    let chances: Vec<Vec<f32>> = race_results
        .iter()
        .map(|result| result.start_irating as f32)
        .map(|a| {
            race_results
                .iter()
                .map(|result| result.start_irating as f32)
                .map(|b| chance(a, b, br1))
                .collect()
        })
        .collect();

    // this appears to be unused?
    //
    // let sof_exponential: Vec<f32> = race_results
    //     .iter()
    //     .map(|result| result.start_irating as f32)
    //     .map(|irating| (-irating / br1).exp())
    //     .collect();

    let expected_scores: Vec<f32> = chances
        .iter()
        .map(|chances| chances.iter().sum::<f32>() - 0.5)
        .collect();

    let fudge_factors: Vec<f32> = race_results
        .iter()
        .map(|result| match result.started {
            false => 0.,
            true => {
                let x = num_registrations as f32 - num_non_starters as f32 / 2.;
                (x / 2. - result.finish_rank as f32) / 100.
            }
        })
        .collect();

    let changes_starters: Vec<Option<f32>> = race_results
        .iter()
        .zip(expected_scores.iter())
        .zip(fudge_factors.iter())
        .map(
            |((result, expected_score), fudge_factor)| match result.started {
                false => None,
                true => Some(
                    (num_registrations as f32
                        - result.finish_rank as f32
                        - expected_score
                        - fudge_factor)
                        * 200.
                        / num_starters as f32,
                ),
            },
        )
        .collect();

    let sum_changes_starters: f32 = changes_starters.iter().filter_map(Option::as_ref).sum();

    let expected_score_non_starters: Vec<Option<f32>> = race_results
        .iter()
        .zip(expected_scores.iter())
        .map(|(result, expected_score)| (!result.started).then_some(*expected_score))
        .collect();

    let sum_expected_score_non_starters: f32 = expected_score_non_starters
        .iter()
        .filter_map(Option::as_ref)
        .sum();

    let changes_non_starters: Vec<Option<f32>> = expected_score_non_starters
        .iter()
        .map(|expected_score| {
            expected_score.map(|expected_score| {
                -sum_changes_starters / num_non_starters as f32 * expected_score
                    / (sum_expected_score_non_starters / num_non_starters as f32)
            })
            //
        })
        .collect();

    let changes: Vec<f32> = changes_starters
        .iter()
        .zip(changes_non_starters.iter())
        .map(|change| match change {
            (Some(change), None) => *change,
            (None, Some(change)) => *change,
            (_, _) => panic!(),
        })
        .collect();

    race_results
        .into_iter()
        .zip(changes.iter())
        .map(|(result, change)| {
            let new_irating = (result.start_irating as f32 + change).round() as u32;
            CalculationResult {
                race_result: result,
                irating_change: *change,
                new_irating,
            }
        })
        .collect()
}

fn chance(a: f32, b: f32, factor: f32) -> f32 {
    (1. - (-a / factor).exp()) * (-b / factor).exp()
        / ((1. - (-b / factor).exp()) * (-a / factor).exp()
            + (1. - (-a / factor).exp()) * (-b / factor).exp())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
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

        let result = super::calculate(race_results);
        insta::assert_debug_snapshot!(result);
    }
}
