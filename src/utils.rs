use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

#[derive(Debug)]
pub struct Client {
    pub likes: Vec<String>,
    pub dislikes: Vec<String>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_input(filename: &String) -> (Vec<String>, Vec<Client>) {
    let mut all_ingredients: Vec<String> = vec![];
    let mut all_clients: Vec<Client> = vec![];
    if let Ok(mut lines) = read_lines(filename) {
        let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        for _ in 0..n {
            let line = lines.next().unwrap().unwrap();
            let ingredients_like = &line.split(" ").collect::<Vec<&str>>()[1..];
            let line = lines.next().unwrap().unwrap();
            let ingredients_dislike = &line.split(" ").collect::<Vec<&str>>()[1..];
            for ingredient in [ingredients_like, ingredients_dislike].concat() {
                if !all_ingredients.contains(&ingredient.to_string()) {
                    all_ingredients.push(ingredient.to_string());
                }
            }
            all_clients.push(Client {
                likes: ingredients_like
                    .to_vec()
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                dislikes: ingredients_dislike
                    .to_vec()
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
            });
        }
    }
    (all_ingredients, all_clients)
}

pub fn write_output(filename: &String, solution: Vec<String>) {
    let mut output_file = File::create(filename).unwrap();
    writeln!(output_file, "{} {}", solution.len(), solution.join(" ")).unwrap();
}
