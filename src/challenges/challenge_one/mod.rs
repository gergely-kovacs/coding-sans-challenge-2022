/*
-----------------------------------------------------------------------------------------
    Challenge 1
    What was last week's sales in terms of money?

    With the Tax Authority already breathing down Stevie's neck,
    he quickly needs to report his last week of sales. Based on the data you received,
    calculate how much money he generated, if he only needs to report his total sales.

    Your function should take the whole data-set, and return a number,
    calculated based on his last week's sales, and the price he sells the sweets for.
-----------------------------------------------------------------------------------------
*/

use serde_json::json;

use crate::answer_writer::write_answer_to_json;
use crate::types::{BakeryData, Recipe, SoldEntry};

fn calculate_last_weeks_sales(recipes: &[Recipe], sold_entries: &[SoldEntry]) -> usize {
    sold_entries.iter().fold(0, |acc, sold_entry| {
        let matching_recipe = recipes.iter().find(|recipe| recipe.name == sold_entry.name);
        match matching_recipe {
            Some(recipe) => {
                acc + recipe
                    .price
                    .strip_suffix(" Ft")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    * sold_entry.amount
            }
            None => acc,
        }
    })
}

pub fn solve_challenge_one(input_data: &BakeryData) -> usize {
    let sales_value =
        calculate_last_weeks_sales(&input_data.recipes, &input_data.sales_of_last_week);
    write_answer_to_json(
        "src/answers/answer-one.json",
        json!({"last_weeks_sales": sales_value}),
    );
    sales_value
}
