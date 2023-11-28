iRacing iRating Calculator
===============================================================================

A Rust library to estimate [iRacing](https://www.iracing.com/) iRating changes.

The code in this repository is based on the [iRacing SOF iRating Calculator v1_1.xlsx](https://github.com/SIMRacingApps/SIMRacingApps/files/3617438/iRacing.SOF.iRating.Calculator.v1_1.xlsx) shared in https://github.com/SIMRacingApps/SIMRacingApps/issues/209#issuecomment-531877336


Usage
-------------------------------------------------------------------------------

```rust
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
```

Output:

```
#1  Driver 1: 3203 -> 3280 (+76.789154)
#2  Driver 2: 3922 -> 3939 (+16.828392)
#3  Driver 3: 2974 -> 2979 (+4.8352695)
#4  Driver 4: 1739 -> 1750 (+11.098258)
#5  Driver 5: 1250 -> 1243 (-6.864749)
#6  Driver 6: 2588 -> 2485 (-102.686325)
```


License
------------------------------------------------------------------------------

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.
