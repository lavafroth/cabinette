use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    pub version: u32,
    pub ingredients: Vec<String>,
    pub recipes: Vec<Recipe>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Recipe {
    name: String,
    ingredients: Vec<u32>,
}

impl Store {
    pub fn new(version: u32) -> Self {
        Self {
            version,
            ingredients: vec![],
            recipes: vec![],
        }
    }

    pub fn add_ingredient(&mut self, ingredient: &str) {
        self.ingredients.push(ingredient.to_string());
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        assert!(recipe
            .ingredients
            .iter()
            .all(|&x| x < self.ingredients.len() as u32));
        self.recipes.push(recipe);
    }

    pub fn remove_ingredient(&mut self, ingredient: &str) {
        let Some(pos) = self.ingredients.iter().position(|s| s == ingredient) else {
            return;
        };
        let Some(last) = self.ingredients.pop() else {
            return;
        };

        if pos == self.ingredients.len() - 1 {
            return;
        }

        self.ingredients[pos] = last;

        let pos = pos as u32;

        while self
            .recipes
            .last()
            .is_some_and(|r| r.ingredients.contains(&pos))
        {
            self.recipes.pop();
        }

        while let Some(rpos) = self
            .recipes
            .iter()
            .rposition(|r| r.ingredients.contains(&pos))
        {
            self.recipes[rpos] = self.recipes.pop().unwrap();
        }
    }

    pub fn remove_recipe(&mut self, recipe: &Recipe) {
        let Some(pos) = self.recipes.iter().position(|s| s == recipe) else {
            return;
        };
        let Some(last) = self.recipes.pop() else {
            return;
        };

        if pos == self.recipes.len() - 1 {
            return;
        }

        self.recipes[pos] = last;
    }
}

impl Recipe {
    pub fn new(name: &str, ingredients: &[u32]) -> Self {
        Self {
            name: name.to_string(),
            ingredients: ingredients.to_vec(),
        }
    }
}
