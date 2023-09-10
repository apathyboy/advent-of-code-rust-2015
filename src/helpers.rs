/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

#[must_use]
pub fn parse_dimensions(input: &str) -> Option<(i32, i32, i32)> {
    let mut dimensions = input.split('x');
    let length = dimensions.next()?.parse::<i32>().ok()?;
    let width = dimensions.next()?.parse::<i32>().ok()?;
    let height = dimensions.next()?.parse::<i32>().ok()?;
    Some((length, width, height))
}
