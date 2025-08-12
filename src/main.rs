use std::collections::HashMap;

#[derive(Debug)]
enum Unit {
    Gram,
    Kilogram,
    Milimiter,
    Liter,
    Cup,
    spoon,
}

#[derive(Debug)]
enum DietryTag {
    Vegetarian,
    NonVegetarian,
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    quantity: f64,
    unit: Unit,
    dietry_tag: DietryTag,
}

#[derive(Debug)]
struct Recipe {
    name: String,
    ingredient: Vec<Ingredient>,
    instructions: Vec<String>,
    servings: u32,
    tags: Vec<DietryTag>,
}

#[derive(Debug)]
struct Substitution {
    original: String,
    alternatives: Vec<Ingredient>,
}

#[derive(Debug)]
struct RecipeBook {
    recipes: Vec<Recipe>,
    substitutions: Vec<Substitution>,
}

#[derive(Debug)]
struct MealPlan {
    days: HashMap<String, Vec<Recipe>>,
}

fn main() {
    println!(".rs");
}
