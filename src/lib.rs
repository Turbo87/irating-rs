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
