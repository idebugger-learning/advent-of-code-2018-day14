const INPUT: usize = 793031;

fn main() {
    let mut recipies = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;

    while recipies.len() < INPUT + 10 {
        let mut sum = recipies[elf1] + recipies[elf2];
        let mut sum_digits = vec![];
        loop {
            sum_digits.push(sum % 10);
            sum /= 10;

            if sum == 0 {
                break;
            }
        }
        for &digit in sum_digits.iter().rev() {
            recipies.push(digit);
        }

        elf1 = (elf1 + recipies[elf1] + 1) % recipies.len();
        elf2 = (elf2 + recipies[elf2] + 1) % recipies.len();
    }

    let result = recipies[INPUT..INPUT + 10]
        .iter()
        .map(|&score| score.to_string().chars().collect::<Vec<_>>())
        .flatten()
        .collect::<String>();
    println!("Scoreboard len {}", recipies.len());
    println!("Result: {}", result);
}
