use std::io::{self, Write, stdin};

fn main() {
    print!("Digite 4 números separados por vírgulas: ");
    io::stdout().flush().unwrap();

    let mut nums = String::new();
    stdin().read_line(&mut nums).expect("Não digitou nada");
    let nums: Vec<&str> = nums.trim().split(",").collect();
    let nums: Vec<i8> = nums.into_iter().map(|num| -> i8{
        let new_num: Result<i8, std::num::ParseIntError> = num.parse();

        return match new_num {
            Ok(value) => value,
            Err(err) => {
                println!("Algo deu errado: {}. Substituindo por 0.", err);
                return 0;
            },
        };
    }).collect();
 
    nums.iter().enumerate().for_each(|(index,n)| {
        let mut summed_items: i8 = *n;
        let mut multiplied_items: i8 = *n;

        for i in index + 1..nums.len() {
            summed_items += nums[i];
        };

        for i in index + 1..nums.len() {
            multiplied_items *= nums[i];
        }

        println!("Soma: {}", summed_items);
        println!("Multiplicação: {}", multiplied_items);
    });
}