use rand::{prelude::SliceRandom, Rng};
use std::env;

mod utils;
use utils::Client;

struct OnePizza {
    ingredients: Vec<String>,
    clients: Vec<Client>,
}

impl OnePizza {
    fn new(filename: &String) -> OnePizza {
        let (ingredients, clients) = utils::parse_input(filename);
        OnePizza {
            ingredients,
            clients,
        }
    }

    fn score_solution(&self, solution: &Vec<String>) -> i32 {
        let mut score = 0;
        for client in &self.clients {
            if client.likes.iter().all(|x| solution.contains(x))
                && client.dislikes.iter().all(|x| !solution.contains(x))
            {
                score += 1;
            }
        }
        score
    }

    fn calculate_random_solution(&self) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let mut ingredients_copy = self.ingredients.clone();
        let mut best_solution = vec![];
        let mut highest_score = 0;
        for _ in 0..10 {
            let n_ingredients = rand::thread_rng().gen_range(1..ingredients_copy.len());
            ingredients_copy.shuffle(&mut rng);
            let solution = ingredients_copy[0..n_ingredients].to_vec();
            let score = self.score_solution(&solution);
            if score > highest_score {
                best_solution = solution;
                highest_score = score;
            }
        }
        best_solution
    }

    fn calculate_greedy_solution(&self) -> Vec<String> {
        let ingredients_copy = self.ingredients.clone();
        let scores: Vec<(String, i32)> = ingredients_copy
            .iter()
            .map(|ingredient| {
                let mut score = 0;
                for client in &self.clients {
                    if client.likes.contains(&ingredient.to_string()) {
                        score += 1;
                    }
                    if client.dislikes.contains(&ingredient.to_string()) {
                        score -= 1;
                    }
                }
                (ingredient.to_string(), score)
            })
            .collect();
        let mut sorted_scores = scores.clone();
        sorted_scores.sort_by(|a, b| b.1.cmp(&a.1));
        let n = ingredients_copy.len() * 3 / 4;
        sorted_scores[0..n].iter().map(|x| x.0.clone()).collect()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid number of arguments")
    }
    let filename = &args[1];
    let one_pizza = OnePizza::new(filename);
    let greedy_solution = one_pizza.calculate_greedy_solution();
    let output_filename = &filename
        .replace("/input_data/", "/output_data/")
        .replace(".in.", ".out.");
    utils::write_output(output_filename, greedy_solution);
}
