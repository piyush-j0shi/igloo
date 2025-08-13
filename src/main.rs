use std::collections::HashMap;

#[derive(Debug)]
enum Unit {
    Gram,
    Kilogram,
    Milimiter,
    Liter,
    Cup,
    Spoon,
}

#[derive(Debug, PartialEq)]
enum DietaryTag {
    Vegetarian,
    NonVegetarian,
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    quantity: f64,
    unit: Unit,
    dietary_tag: DietaryTag,
}

#[derive(Debug)]
struct Recipe {
    name: String,
    ingredient: Vec<Ingredient>,
    instructions: Vec<String>,
    servings: u32,
    tags: Vec<DietaryTag>,
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

impl RecipeBook {
    fn new() -> Self {
        let recipes: Vec<Recipe> = Vec::new();
        let substitutions: Vec<Substitution> = Vec::new();

        Self {
            recipes: recipes,
            substitutions: substitutions,
        }
    }

    fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    fn remove_recipe(&mut self, name: &str) {
        if let Some(index) = self.recipes.iter().position(|x| x.name == name) {
            self.recipes.remove(index);
        } else {
            println!("recipe did not found for this name");
        }
    }

    fn findrecipe_byname(&self, name: &str) {
        if let Some(recipe) = self.recipes.iter().find(|x| x.name.contains(name)) {
            println!("recipe found : {:#?}", recipe);
        } else {
            println!("recipe did not found");
        }
    }

    fn findby_ingredient(&self, ingredientsearch: &str) {
        let mut ingredientsvec: Vec<&Recipe> = Vec::new();

        for ingredients in &self.recipes {
            for moreingredients in &ingredients.ingredient {
                if moreingredients.name.contains(ingredientsearch) {
                    ingredientsvec.push(ingredients);
                }
            }
        }

        if ingredientsvec.is_empty() {
            println!("no recipes found");
        } else {
            println!("all recipes are : {:#?}", ingredientsvec);
        }
    }

    fn findrecipe_bydietary(&self, tag: DietaryTag) {
        let mut ingredientsvec: Vec<&Recipe> = Vec::new();

        for ingredients in &self.recipes {
            if ingredients.tags.contains(&tag) {
                ingredientsvec.push(ingredients);
            }
        }

        if ingredientsvec.is_empty() {
            println!("nothing found");
        } else {
            println!("all recipes are : {:#?}", ingredientsvec);
        }
    }
}

#[derive(Debug)]
struct MealPlan {
    days: HashMap<String, Vec<Recipe>>,
}

fn scale_recipe(recipe: &mut Recipe, target_serving: u32) {
    let scale_factor = target_serving / recipe.servings;

    for ingredients in &mut recipe.ingredient {
        println!("ingredients are : {:#?}", ingredients);
        let updated_quantity = ingredients.quantity * scale_factor as f64;
        ingredients.quantity = updated_quantity;
    }
}

fn main() {
    let mut spaghetti_recipe = Recipe {
        name: String::from("Spaghetti with Tomato Sauce"),
        ingredient: vec![
            Ingredient {
                name: String::from("Spaghetti"),
                quantity: 400.0,
                unit: Unit::Gram,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Olive oil"),
                quantity: 2.0,
                unit: Unit::Spoon,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Garlic (minced)"),
                quantity: 3.0,
                unit: Unit::Spoon,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Canned crushed tomatoes"),
                quantity: 800.0,
                unit: Unit::Gram,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Salt"),
                quantity: 1.0,
                unit: Unit::Spoon,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Sugar"),
                quantity: 1.0,
                unit: Unit::Spoon,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Fresh basil (chopped)"),
                quantity: 10.0,
                unit: Unit::Spoon,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Parmesan cheese"),
                quantity: 50.0,
                unit: Unit::Gram,
                dietary_tag: DietaryTag::Vegetarian,
            },
        ],
        instructions: vec![
            String::from("Boil spaghetti in salted water until cooked."),
            String::from("Heat olive oil, sauté garlic until fragrant."),
            String::from("Add crushed tomatoes, salt, and sugar; simmer."),
            String::from("Stir in fresh basil."),
            String::from("Mix spaghetti with sauce."),
            String::from("Top with Parmesan cheese before serving."),
        ],
        servings: 4,
        tags: vec![DietaryTag::Vegetarian],
    };

    scale_recipe(&mut spaghetti_recipe, 8);
    println!("update recipe is : {:#?}", spaghetti_recipe);

    let mut recipe_book = RecipeBook::new();

    recipe_book.add_recipe(spaghetti_recipe);
    // recipe_book.add_recipe(spaghetti_recipe1);

    // println!("new updates recipe_book : {:#?}", recipe_book);

    recipe_book.findrecipe_byname("Spaghetti with Tomato Sauce");
    recipe_book.findby_ingredient("Sugar");
    recipe_book.findrecipe_bydietary(DietaryTag::Vegetarian);
    recipe_book.remove_recipe("Spaghetti with Tomato Sauce");
    // println!("new updated recipe_book : {:#?}", recipe_book);
}
