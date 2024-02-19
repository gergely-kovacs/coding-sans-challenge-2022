/*
-----------------------------------------------------------------------------------------
    Challenge 2
    Create a menu for people with food allergies.

    Currently, Stevie keeps all information about gluten
    or lactose free options in his head,and his memory isnt getting better.
    To avoid him having a local hospital on speed-dial,
    create a list of gluten and lactose-free items!
    It should have three sections, gluten-free only ,
    lactose-free only , and both gluten and lactose-free.

    Your function should take the whole data-set,
    and return an object, with the three options containing an array
    of objects with the name,and price of each item.
-----------------------------------------------------------------------------------------
*/

use crate::answer_writer::write_answer_to_json;
use crate::types::{BakeryData, MenuEntry, MenuWithAllergens, Recipe};

fn create_menu_with_allergens(recipes: &[Recipe]) -> MenuWithAllergens {
    MenuWithAllergens {
        gluten_free: recipes
            .iter()
            .filter_map(|recipe| {
                if recipe.gluten_free && !recipe.lactose_free {
                    Some(MenuEntry {
                        name: recipe.name.clone(),
                        price: recipe.price.clone(),
                    })
                } else {
                    None
                }
            })
            .collect(),
        lactose_free: recipes
            .iter()
            .filter_map(|recipe| {
                if recipe.lactose_free && !recipe.gluten_free {
                    Some(MenuEntry {
                        name: recipe.name.clone(),
                        price: recipe.price.clone(),
                    })
                } else {
                    None
                }
            })
            .collect(),
        both: recipes
            .iter()
            .filter_map(|recipe| {
                if recipe.gluten_free && recipe.lactose_free {
                    Some(MenuEntry {
                        name: recipe.name.clone(),
                        price: recipe.price.clone(),
                    })
                } else {
                    None
                }
            })
            .collect(),
    }
}

pub fn solve_challenge_two(data: &BakeryData) -> MenuWithAllergens {
    let allergenic_menu = create_menu_with_allergens(&data.recipes);
    write_answer_to_json(
        "src/answers/answer-two.json",
        serde_json::to_value(&allergenic_menu).unwrap(),
    );
    allergenic_menu
}
