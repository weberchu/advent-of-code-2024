use std::ops::Range;
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day09.txt").trim();

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> usize {
    let disk_map: Vec<usize> = input.chars().map(|c| c as usize - '0' as usize).collect();

    let mut current_block_id = 0;
    let mut current_cluster_pointer = 0;
    let mut end_cluster_pointer = input.len() - 1;
    let mut end_cluster_used = 0;
    let mut checksum = 0;

    while current_cluster_pointer < end_cluster_pointer {
        // forward scanning
        let num_of_block = disk_map[current_cluster_pointer];
        let current_cluster_id = current_cluster_pointer / 2;
        for _ in 0..num_of_block {
            checksum += current_block_id * current_cluster_id;
            current_block_id += 1;
        }
        current_cluster_pointer += 1;

        // backward scanning
        let mut num_of_free_space = disk_map[current_cluster_pointer];
        if current_cluster_pointer == end_cluster_pointer - 1 {
            let end_cluster = disk_map[end_cluster_pointer];
            if end_cluster - end_cluster_used < num_of_free_space {
                // end cluster is not enough to fill the space
                let end_cluster_id = end_cluster_pointer / 2;
                for _ in 0..(end_cluster - end_cluster_used) {
                    checksum += current_block_id * end_cluster_id;
                    current_block_id += 1;
                    end_cluster_used += 1;
                }

                break;
            }
        }

        while num_of_free_space > 0 && current_cluster_pointer < end_cluster_pointer {
            let end_cluster = disk_map[end_cluster_pointer];
            let end_cluster_id = end_cluster_pointer / 2;
            if end_cluster_used < end_cluster {
                // take a block from end cluster
                checksum += current_block_id * end_cluster_id;
                current_block_id += 1;

                end_cluster_used += 1;
                num_of_free_space -= 1;
            } else {
                // end cluster used up, move to second last cluster
                end_cluster_used = 0;
                end_cluster_pointer -= 2;
            }
        }
        current_cluster_pointer += 1;
    }

    // handle leftover end cluster
    let end_cluster = disk_map[end_cluster_pointer];
    while end_cluster > end_cluster_used {
        let end_cluster_id = end_cluster_pointer / 2;
        checksum += current_block_id * end_cluster_id;
        current_block_id += 1;
        end_cluster_used += 1;
    }

    checksum
}

#[derive(Debug, Clone)]
struct FileBlock {
    id: usize,
    block_len: usize,
    position_range: Range<usize>,
}

#[derive(Debug)]
struct FreeBlock {
    file_blocks: Vec<FileBlock>,
    free_len: usize,
    position_range: Range<usize>,
}

fn part2(input: &str) -> usize {
    let mut files: Vec<FileBlock> = Vec::new();
    let mut free_blocks: Vec<FreeBlock> = Vec::new();
    let mut filled_blocks: Vec<FreeBlock> = Vec::new();
    let mut position_pointer = 0;

    input.chars().enumerate().for_each(|(i, c)| {
        let block_len = c as usize - '0' as usize;
        if i % 2 == 0 {
            // file
            files.push(FileBlock {
                id: i / 2,
                block_len,
                position_range: position_pointer .. position_pointer + block_len,
            });
        } else {
            // free block
            free_blocks.push(FreeBlock {
                file_blocks: Vec::new(),
                free_len: block_len,
                position_range: position_pointer .. position_pointer + block_len,
            })
        }

        position_pointer += block_len;
    });

    for i in (0..files.len()).rev() {
        let file = &files[i];
        for j in 0..free_blocks.len() {
            let free_block = &free_blocks[j];
            if file.position_range.end <= free_block.position_range.start {
                // file cannot move right
                break;
            }

            if file.block_len <= free_block.free_len {
                // move left
                let mut file_blocks = free_block.file_blocks.clone();
                let position_start = free_block.position_range.end - free_block.free_len;
                file_blocks.push(FileBlock {
                    id: file.id,
                    block_len: file.block_len,
                    position_range: position_start .. position_start + file.block_len,
                });
                let free_block = FreeBlock {
                    file_blocks,
                    free_len: free_block.free_len - file.block_len,
                    position_range: free_block.position_range.clone(),
                };

                if free_block.free_len > 0 {
                    free_blocks[j] = free_block;
                } else {
                    filled_blocks.push(free_block);
                    free_blocks.remove(j);
                }
                files.remove(i);

                break;
            }
        }
    }

    let mut checksum = 0;
    for file in files {
        for position in file.position_range {
            checksum += file.id * position;
        }
    }

    for block in free_blocks {
        for file in block.file_blocks {
            for position in file.position_range {
                checksum += file.id * position;
            }
        }
    }

    for block in filled_blocks {
        for file in block.file_blocks {
            for position in file.position_range {
                checksum += file.id * position;
            }
        }
    }

    checksum
}
