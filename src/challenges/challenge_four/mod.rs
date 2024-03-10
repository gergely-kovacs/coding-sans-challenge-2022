/*
-----------------------------------------------------------------------------------------
    Challenge 4
    The freedom of choice

    The best-selling items are already available,
    but Stevie wants to know what sweets he could make without
    purchasing additional ingredients, only working from his existing inventory.

    At this point, Stevie confesses with tears in his eyes that he somehow managed to go
    through primary school without actually
    learning metric unit conversions. He only knows milliliters and grams, so the recipes
    in his cookbook are in these units.
    However, his inventory software, and the wholesaler
    both report in kilograms and liters.

    This hasn't been much of a problem as long as he was conducting heart surgeries,
    apart from getting weird looks from his butcher
    when asking for 1523 grams of chicken breast. But he can't run a bakery like this,
    so he needs your help.

    Give Stevie the amount of sweets he could produce,if he used his
    entire inventory for one recipe.
    Do this for each recipe he can make based on his current inventory and list it in
    alphabetic order based on the recipe's name.
    Your function should take the whole data-set, and return a list matching the
    structure of the example below.
-----------------------------------------------------------------------------------------
*/

use crate::answer_writer::write_answer_to_json;
use crate::types::{BakeryData, InventoryEntry, MaxAmountFromInventory, Recipe};

fn determine_max_amount_of_each_recipe_from_inventory(
    recipes: &[Recipe],
    inventory: &[InventoryEntry],
) -> Vec<MaxAmountFromInventory> {
    recipes
        .iter()
        .map(|recipe| {
            let max_amount_per_ingredient: Vec<usize> = recipe
                .ingredients
                .iter()
                .map(|ingredient| {
                    let ingredient_in_inventory = inventory
                        .iter()
                        .find(|item| item.name.to_lowercase() == ingredient.name.to_lowercase());
                    match ingredient_in_inventory {
                        Some(ingredient_in_inv) => {
                            let normalized_amount_in_inventory = if ingredient_in_inv
                                .amount
                                .contains(" kg")
                            {
                                ingredient_in_inv
                                    .amount
                                    .strip_suffix(" kg")
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap()
                                    * 1000
                            } else if ingredient_in_inv.amount.contains(" l") {
                                ingredient_in_inv
                                    .amount
                                    .strip_suffix(" l")
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap()
                                    * 1000
                            } else if ingredient_in_inv.amount.contains(" pc") {
                                ingredient_in_inv
                                    .amount
                                    .strip_suffix(" pc")
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap()
                            } else {
                                panic!("Invalid amount in inventory: {}", ingredient_in_inv.amount);
                            };
                            let ingredient_in_recipe = if ingredient.amount.contains(" g") {
                                ingredient
                                    .amount
                                    .strip_suffix(" g")
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap()
                            } else if ingredient.amount.contains(" ml") {
                                ingredient
                                    .amount
                                    .strip_suffix(" ml")
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap()
                            } else if ingredient.amount.contains(" pc") {
                                ingredient
                                    .amount
                                    .strip_suffix(" pc")
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap()
                            } else {
                                panic!("Invalid amount in recipe: {}", ingredient.amount);
                            };
                            normalized_amount_in_inventory / ingredient_in_recipe
                        }
                        None => 0,
                    }
                })
                .collect();
            MaxAmountFromInventory {
                name: recipe.name.clone(),
                amount: *max_amount_per_ingredient.iter().min().unwrap_or(&0usize),
            }
        })
        .collect()
}

pub fn solve_challenge_four(bakery_data: &BakeryData) -> Vec<MaxAmountFromInventory> {
    let inventory = &bakery_data.inventory;
    let recipes = &bakery_data.recipes;
    let mut max_amounts = determine_max_amount_of_each_recipe_from_inventory(recipes, inventory);
    max_amounts.sort_by(|a, b| a.name.cmp(&b.name));
    write_answer_to_json(
        "src/answers/answer-four.json",
        serde_json::to_value(&max_amounts).unwrap(),
    );
    max_amounts
}
