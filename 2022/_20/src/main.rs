use std::io::stdin;


fn main() {
    let mut numbers = Vec::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        numbers.push(line.trim().parse::<i64>().unwrap() * 811589153);
    }

    let mut orig_index: Vec<_> = (0..numbers.len()).collect();
    for _ in 0..10 {
    for index_to_visit in 0..numbers.len() {
        let mut index = 0;
        while orig_index[index] != index_to_visit {
            index += 1;
        }
        let value = numbers[index];
        let new_spot = if value < 0 {
            let value = value % (numbers.len() - 1) as i64;
            let mut new_index = index as i64 + value;
            if new_index < 0 {
                new_index += numbers.len() as i64 - 1;
            }
            new_index as usize
        } else {
            let value = value % (numbers.len() - 1) as i64;
            let mut new_index = index as i64 + value;
            if new_index >= numbers.len() as i64 {
                new_index -= numbers.len() as i64 - 1;
            }
            new_index as usize
        };
        if index < new_spot {
            for i in (index + 1)..=new_spot {
                numbers[i - 1] = numbers[i];
                orig_index[i - 1] = orig_index[i];
            }
        } else {
            for i in (new_spot..index).rev() {
                numbers[i + 1] = numbers[i];
                orig_index[i + 1] = orig_index[i];
            }
        }
        numbers[new_spot] = value;
        orig_index[new_spot] = index_to_visit;
        // dbg!(&numbers);
    }
}
    let mut zero_index = 0;
    for (i, &num) in numbers.iter().enumerate() {
        if num == 0 {
            zero_index = i;
            break;
        }
    }
    dbg!(zero_index);
    dbg!(numbers[(zero_index + 1000) % numbers.len()]);
    dbg!(numbers[(zero_index + 2000) % numbers.len()]);
    dbg!(numbers[(zero_index + 3000) % numbers.len()]);
    dbg!(
        numbers[(zero_index + 1000) % numbers.len()] +
        numbers[(zero_index + 2000) % numbers.len()] +
        numbers[(zero_index + 3000) % numbers.len()]
    );
}
