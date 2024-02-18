use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Ingredient {
    pub name: String,
    pub amount: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Recipe {
    pub name: String,
    pub price: String,
    pub lactose_free: bool,
    pub gluten_free: bool,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Deserialize, Serialize)]
pub struct SoldEntry {
    pub name: String,
    pub amount: usize,
}

#[derive(Deserialize, Serialize)]
pub struct InventoryEntry {
    pub name: String,
    pub amount: String,
}

#[derive(Deserialize, Serialize)]
pub struct WholesalePrice {
    pub name: String,
    pub amount: String,
    pub price: usize,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct BakeryData {
    pub recipes: Vec<Recipe>,
    pub sales_of_last_week: Vec<SoldEntry>,
    pub inventory: Vec<InventoryEntry>,
    pub wholesale_prices: Vec<WholesalePrice>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct MenuWithAllergens {
    pub gluten_free: Vec<Recipe>,
    pub lactose_free: Vec<Recipe>,
    pub both: Vec<Recipe>,
}
