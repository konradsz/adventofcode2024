#[derive(Clone, Copy, Debug)]
enum Block {
    File(usize),
    Free,
}

fn part_1(mut memory: Vec<Block>) -> usize {
    let mut left = 0;
    let mut right = memory.len() - 1;
    while left != right {
        match memory[right] {
            Block::File(_) => match memory[left] {
                Block::File(_) => left += 1,
                Block::Free => {
                    memory.swap(left, right);
                    right -= 1;
                }
            },
            Block::Free => right -= 1,
        }
    }

    memory
        .iter()
        .enumerate()
        .map(|(pos, block)| match block {
            Block::File(idx) => pos * idx,
            Block::Free => 0,
        })
        .sum()
}

fn part_2(mut memory: Vec<Block>) -> usize {
    let Block::File(idx) = memory.last().unwrap() else {
        return 0;
    };

    'block: for i in (0..=*idx).rev() {
        let mut block_end = 0;
        for (back_idx, b) in memory.iter().enumerate().rev() {
            match b {
                Block::File(file_idx) => {
                    if *file_idx == i {
                        block_end = back_idx;
                        break;
                    }
                }
                Block::Free => continue,
            }
        }
        let mut block_start = 0;
        for front_idx in (0..block_end).rev() {
            let Block::File(file_idx) = memory[front_idx] else {
                block_start = front_idx + 1;
                break;
            };
            if file_idx != i {
                block_start = front_idx + 1;
                break;
            }
        }

        let block_length = block_end - block_start + 1;

        for jj in 0..memory.len() {
            if jj >= block_start {
                continue 'block;
            }

            if memory[jj..jj + block_length]
                .iter()
                .all(|b| matches!(b, Block::Free))
            {
                for jjj in 0..block_length {
                    memory.swap(jj + jjj, block_start + jjj);
                }
                break;
            }
        }
    }

    memory
        .iter()
        .enumerate()
        .map(|(pos, block)| match block {
            Block::File(idx) => pos * idx,
            Block::Free => 0,
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut current_block = Block::Free;
    let mut idx = 0;
    let mut memory = Vec::new();
    for c in input.chars() {
        if !c.is_digit(10) {
            break;
        }

        let digit = c.to_digit(10).unwrap();
        match current_block {
            Block::File(_) => {
                current_block = Block::Free;
                memory.append(&mut vec![current_block; digit as usize]);
                idx += 1;
            }
            Block::Free => {
                current_block = Block::File(idx);
                memory.append(&mut vec![current_block; digit as usize]);
            }
        }
    }

    assert_eq!(part_1(memory.clone()), 6334655979668);
    assert_eq!(part_2(memory.clone()), 6349492251099);
}
