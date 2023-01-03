use std::fs;

fn main() {
    let input = fs::read_to_string("20-twenty/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one : {}", part_one);
}

#[derive(Debug)]
struct Data {
    // index: i64,
    value: i64,
    inital_index: usize,
}

impl Data {
    fn new(index: usize, value: i64) -> Self {
        Data {
            // index: index as i64,
            value,
            inital_index: index,
        }
    }

    // fn change_index(&mut self, travel: i64, vec_len: i64) {
    //     self.index += travel % vec_len;
    // }
}


fn parse_input_as_data(input: &str) -> Vec<Data> {
    input
        .lines()
        .enumerate()
        .map(|(idx, value)| Data::new(idx, value.parse().unwrap()))
        .collect()
}

// fn parse_input(input: &str) -> impl Iterator<Item = i64> + '_ {
//     // let nb_lines = input.lines().count();
//     input.lines().into_iter().cycle().map(|x| x.parse::<i64>().unwrap())
// }
// fn find_affected_index(start: i64, travel: i64) -> Range<i64> {
//     match travel {
//         0 => 0..0,
//         1.. => start..(start + travel),
//         _ => (start - travel)..start,
//     }
// }
//
// fn move_neighbour(datas: &mut Vec<Data>, moving_neighbours: Range<i64>, is_positive: bool) {
//     for subject_index in moving_neighbours {
//         let Some(mut subject) = datas.get_mut(subject_index as usize) else {
//             panic!("negative index");
//         };
//         if is_positive {
//             subject.index -= 1;
//         } else {
//             subject.index += 1;
//         }
//     }
// }

fn part_one(input: &str) -> i64 {
    const SUMMED_ELEMENT_INDEX: [i64; 3] = [1000, 2000, 3000];
    let mut datas = parse_input_as_data(input);
    let message_size = datas.len() as i64 - 1;
    let vec_len = datas.len();
    for current in 0..vec_len {
        let index = datas.iter().position(|x| x.inital_index == current).unwrap();
        let mut new_index = index as i64 + datas[index].value;
        new_index = ((new_index % message_size) + message_size) % message_size;
        let number = datas.remove(index);
        datas.insert( new_index as usize, number);
    }
    // datas.sort_by(|a, b| a.inital_index.partial_cmp(&b.inital_index).unwrap());
    // for mut data in datas {
    //     let moving_neighbours = find_affected_index(data.index, data.value % vec_len);
    //     data.change_index(data.value, vec_len);
    //     move_neighbour(&mut datas, moving_neighbours, data.value.is_positive());
    // }
    let zero_index = datas.iter().position(|x| x.value == 0).unwrap() as i64;
    SUMMED_ELEMENT_INDEX
        .iter()
        .map(|x| datas.get((zero_index + *x) as usize % vec_len).unwrap())
        .map(|x| x.value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part_one() {
        const EXPECTED: i64 = 3; 
        let res = part_one(INPUT);
        assert_eq!(EXPECTED, res);
    }

}
