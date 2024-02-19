/*
-----------------------------------------------------------------------------------------
    Challenge 3
    What was last week's profit?

    Now that the Tax Guys are off his back, and people with food allergies are safe,
    Stevie wants to know how much money he actually made last week.
    Based on the quantity of the items sold,with the ingredients taken into account,
    calculate the difference between the sum of sale prices
    and the wholesale price of the ingredients used.

    Your function should take the whole data-set, and return a number,
    the difference between the amount of money selling sweets generated,
    and the cost of producing them,
    calculated based on the wholesale price of the ingredients used.
-----------------------------------------------------------------------------------------
*/

use serde_json::json;

use crate::answer_writer::write_answer_to_json;
use crate::types::{BakeryData, Recipe, SoldEntry, WholesalePrice};

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

fn calculate_ingredient_costs(
    sold_entries: &[SoldEntry],
    recipes: &[Recipe],
    wholesale_prices: &[WholesalePrice],
) -> f32 {
    sold_entries.iter().fold(0.0, |total_cost, sold_entry| {
        let matching_recipe = recipes.iter().find(|recipe| recipe.name == sold_entry.name);
        match matching_recipe {
            Some(recipe) => {
                total_cost
                    + recipe
                        .ingredients
                        .iter()
                        .fold(0.0, |cost_per_recipe, ingredient| {
                            let matching_wholesale_price = wholesale_prices
                                .iter()
                                .find(|wholesale_price| wholesale_price.name == ingredient.name)
                                .expect(
                                    "Wholesale price not found for ingredient: {ingredient.name}",
                                );

                            let normalized_ingredient_price =
                                if matching_wholesale_price.amount.ends_with(" kg") {
                                    matching_wholesale_price.price as f32
                                        / (matching_wholesale_price
                                            .amount
                                            .strip_suffix(" kg")
                                            .unwrap()
                                            .parse::<f32>()
                                            .unwrap()
                                            * 1000.0)
                                } else if matching_wholesale_price.amount.ends_with(" l") {
                                    matching_wholesale_price.price as f32
                                        / (matching_wholesale_price
                                            .amount
                                            .strip_suffix(" l")
                                            .unwrap()
                                            .parse::<f32>()
                                            .unwrap()
                                            * 1000.0)
                                } else {
                                    matching_wholesale_price.price as f32
                                        / matching_wholesale_price
                                            .amount
                                            .strip_suffix(" pc")
                                            .unwrap()
                                            .parse::<f32>()
                                            .unwrap()
                                };

                            let ingredient_amount = if ingredient.amount.ends_with(" g") {
                                ingredient
                                    .amount
                                    .strip_suffix(" g")
                                    .unwrap()
                                    .parse::<f32>()
                                    .unwrap()
                            } else if ingredient.amount.ends_with(" ml") {
                                ingredient
                                    .amount
                                    .strip_suffix(" ml")
                                    .unwrap()
                                    .parse::<f32>()
                                    .unwrap()
                            } else {
                                ingredient
                                    .amount
                                    .strip_suffix(" pc")
                                    .unwrap()
                                    .parse::<f32>()
                                    .unwrap()
                            };

                            cost_per_recipe
                                + normalized_ingredient_price
                                    * ingredient_amount
                                    * sold_entry.amount as f32
                        })
            }
            None => total_cost,
        }
    })
}

fn calculate_last_weeks_profit(sales_value: usize, ingredients_value: f32) -> f32 {
    sales_value as f32 - ingredients_value
}

pub fn solve_challenge_three(bakery_data: &BakeryData) -> f32 {
    let sold_entries = &bakery_data.sales_of_last_week;
    let recipes = &bakery_data.recipes;
    let wholesale_prices = &bakery_data.wholesale_prices;
    let sales_value = calculate_last_weeks_sales(recipes, sold_entries);
    let ingredients_value = calculate_ingredient_costs(sold_entries, recipes, wholesale_prices);
    let last_weeks_profit = calculate_last_weeks_profit(sales_value, ingredients_value);
    write_answer_to_json(
        "src/answers/answer-three.json",
        json!({"last_weeks_profit": last_weeks_profit}),
    );
    last_weeks_profit
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Recipe, SoldEntry, WholesalePrice};

    use serde_json::from_str;

    #[test]
    fn test_ingredient_value_calculation() {
        let recipes: Vec<Recipe> = vec![from_str(
            r#"
        {
            "name": "Francia krémes",
            "price": "420 Ft",
            "lactoseFree": false,
            "glutenFree": false,
            "ingredients": [
                { "name": "egg", "amount": "2 pc" },
                { "name": "sugar", "amount": "1000 g" },
                { "name": "fruit", "amount": "60 g" },
                { "name": "milk", "amount": "1000 ml" },
                { "name": "butter", "amount": "70 g" },
                { "name": "flour", "amount": "300 g" }
            ]
        }"#,
        )
        .unwrap()];
        let wholesale_prices: Vec<WholesalePrice> = from_str(
            r#"[
            { "name": "flour", "amount": "10 kg", "price": 1500 },
            { "name": "egg", "amount": "12 pc", "price": 240 },
            { "name": "sugar", "amount": "10 kg", "price": 1200 },
            { "name": "milk", "amount": "10 l", "price": 2000 },
            { "name": "butter", "amount": "1 kg", "price": 2000 },
            { "name": "fruit", "amount": "10 kg", "price": 2000 }
        ]"#,
        )
        .unwrap();
        let sales_of_last_week: Vec<SoldEntry> = vec![SoldEntry {
            name: "Francia krémes".to_string(),
            amount: 2,
        }];
        let ingredients_value =
            calculate_ingredient_costs(&sales_of_last_week, &recipes, &wholesale_prices);
        // 40 on eggs
        // 120 on sugar
        // 12 on fruit
        // 200 on milk
        // 140 on butter
        // 45 on flour
        // 557 per piece
        assert_eq!(ingredients_value, 1114.0);
    }
}
