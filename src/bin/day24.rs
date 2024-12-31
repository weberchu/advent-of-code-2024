use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use regex::Regex;

const TEST_CASES: [(u64, u64); 8] = [
    (0, 0),
    (0b100000000000, 0),
    (0b11111111111111111111111111111111111111111111, 0b0),
    (0b0, 0b11111111111111111111111111111111111111111111),
    (0b1, 0b11111111111111111111111111111111111111111111),
    (0b11111111111111111111111111111111111111111111, 0b11111111111111111111111111111111111111111111),
    (0b10101010101010101010101010101010101010101010, 0b01010101010101010101010101010101010101010101),
    (0b00110011001100110011001100110011001100110011, 0b10011001100110011001100110011001100110011001),
];
const MAX_BIT: u32 = 44;

fn main() {
    let input = include_str!("../../input/day24.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    part2_test(input);
    println!("part 2 tests, took: {:?}", start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {} , took: {:?}", p2_ans, start.elapsed());
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Gate {
    input_1: String,
    input_2: String,
    operation: String,
    output: String,
}

fn process_numbers(number_x: u64, number_y: u64, gates: Vec<Gate>) -> u64 {
    let mut wires = HashMap::new();
    for bit in 0..=MAX_BIT {
        let x_bit = number_x >> bit & 1;
        let y_bit = number_y >> bit & 1;
        let x_wire = format!("x{:02}", bit);
        let y_wire = format!("y{:02}", bit);
        wires.insert(x_wire, x_bit);
        wires.insert(y_wire, y_bit);
    }

    process(wires, gates)
}

fn process(wires: HashMap<String, u64>, gates: Vec<Gate>) -> u64 {
    let mut wires = wires;
    let mut gates = gates;
    while !gates.is_empty() {
        let mut has_processed = false;
        for i in (0..gates.len()).rev() {
            let gate = &gates[i];
            if wires.contains_key(&gate.input_1) && wires.contains_key(&gate.input_2) {
                let output = evaluate(wires[&gate.input_1], wires[&gate.input_2], &gate.operation);
                wires.insert(gate.output.clone(), output);
                gates.remove(i);
                has_processed = true;
            }
        }

        if !has_processed {
            return u64::MAX;
        }
    }

    let mut z_wires: Vec<(&String, &u64)> = wires.iter().filter(|(wire_key, _)| wire_key.starts_with("z")).collect();
    z_wires.sort_by(|a, b| a.0.cmp(b.0));

    z_wires.iter().enumerate().map(|(i, &(_, value))| {
        value * 2_u64.pow(i.try_into().unwrap())
    }).sum()
}

fn evaluate(input_1:  u64, input_2: u64, operation: &str) -> u64 {
    match operation {
        "AND" => input_1 & input_2,
        "OR" => input_1 | input_2,
        "XOR" => input_1 ^ input_2,
        _ => panic!("Unknown operation: {}", operation),
    }
}

fn is_gate_input_xy(gate: &Gate) -> bool {
    let is_xy = |g: &String| g.starts_with("x") || g.starts_with("y");
    is_xy(&gate.input_1) && is_xy(&gate.input_2)
}

fn sort_gates_with_xy_at_end(g1: &Gate, g2: &Gate) -> Ordering {
    let is_g1_xy = is_gate_input_xy(g1);
    let is_g2_xy = is_gate_input_xy(g2);
    if is_g1_xy && !is_g2_xy {
        Ordering::Greater
    } else if !is_g1_xy && is_g2_xy {
        Ordering::Less
    } else {
        let g1_str = format!("{}{}", g1.input_1, g1.input_2);
        let g2_str = format!("{}{}", g2.input_1, g2.input_2);
        g1_str.cmp(&g2_str)
    }
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut wires = HashMap::new();
    let gate_pattern = Regex::new(r"(\w+) (XOR|OR|AND) (\w+) -> (\w+)").unwrap();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let mut split = line.split(": ");
        let wire = split.next().unwrap();
        let value: u64 = split.next().unwrap().parse().unwrap();

        wires.insert(wire.to_string(), value);
    }

    let mut gates = Vec::new();

    for line in lines {
        let captures = gate_pattern.captures(line).unwrap();
        let input_1 = captures.get(1).unwrap().as_str();
        let operation = captures.get(2).unwrap().as_str();
        let input_2 = captures.get(3).unwrap().as_str();
        let output = captures.get(4).unwrap().as_str();

        gates.push(Gate {
            input_1: input_1.to_string(),
            input_2: input_2.to_string(),
            operation: operation.to_string(),
            output: output.to_string(),
        });
    }

    gates.sort_by(sort_gates_with_xy_at_end);

    process(wires, gates)
}

fn part2_test(input: &str) {
    let mut lines = input.lines();
    let mut wires = HashMap::new();
    let gate_pattern = Regex::new(r"(\w+) (XOR|OR|AND) (\w+) -> (\w+)").unwrap();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let mut split = line.split(": ");
        let wire = split.next().unwrap();
        let value: u64 = split.next().unwrap().parse().unwrap();

        wires.insert(wire.to_string(), value);
    }

    let mut gates = Vec::new();
    let mut output_to_gates = HashMap::new();

    for line in lines {
        let captures = gate_pattern.captures(line).unwrap();
        let input_1 = captures.get(1).unwrap().as_str();
        let operation = captures.get(2).unwrap().as_str();
        let input_2 = captures.get(3).unwrap().as_str();
        let output = captures.get(4).unwrap().as_str();

        gates.push(Gate {
            input_1: input_1.to_string(),
            input_2: input_2.to_string(),
            operation: operation.to_string(),
            output: output.to_string(),
        });
        output_to_gates.insert(output.to_string(), Gate {
            input_1: input_1.to_string(),
            input_2: input_2.to_string(),
            operation: operation.to_string(),
            output: output.to_string(),
        });
    }

    gates.sort_by(sort_gates_with_xy_at_end);

    let mut invalid_internal_gate = HashSet::new();
    {
        let mut gates = gates.clone();
        let mut input_xor_gates = HashSet::new();
        let mut input_and_gates = HashSet::new();

        for i in (0..gates.len()).rev() {
            let gate = &gates[i];
            if is_gate_input_xy(gate) {
                if gate.operation == "XOR" {
                    input_xor_gates.insert(gate.output.clone());
                    gates.remove(i);
                } else if gate.operation == "AND" {
                    input_and_gates.insert(gate.output.clone());
                    gates.remove(i);
                }
            }
        }


        for gate in gates {
            match gate.operation.as_str() {
                "AND" => {
                    if !((input_xor_gates.contains(&gate.input_1) && !input_xor_gates.contains(&gate.input_2) && output_to_gates.get(&gate.input_2).unwrap().operation == "OR") ||
                    (input_xor_gates.contains(&gate.input_2) && !input_xor_gates.contains(&gate.input_1) && output_to_gates.get(&gate.input_1).unwrap().operation == "OR")) {
                        println!("Invalid internal AND gate: {:?}", gate);
                        invalid_internal_gate.insert(gate.clone());
                    }
                },
                "XOR" => {
                    if !((input_xor_gates.contains(&gate.input_1) && !input_xor_gates.contains(&gate.input_2) && output_to_gates.get(&gate.input_2).unwrap().operation == "OR") ||
                        (input_xor_gates.contains(&gate.input_2) && !input_xor_gates.contains(&gate.input_1) && output_to_gates.get(&gate.input_1).unwrap().operation == "OR")) {
                        println!("Invalid internal XOR gate: {:?}", gate);
                        invalid_internal_gate.insert(gate.clone());
                    }},
                "OR" => {
                    if !((input_and_gates.contains(&gate.input_1) && !input_and_gates.contains(&gate.input_2) && output_to_gates.get(&gate.input_2).unwrap().operation == "AND") ||
                        (input_and_gates.contains(&gate.input_2) && !input_and_gates.contains(&gate.input_1) && output_to_gates.get(&gate.input_1).unwrap().operation == "AND")) {
                        println!("Invalid internal OR gate: {:?}", gate);
                        invalid_internal_gate.insert(gate.clone());
                    }
                },
                _ => panic!("unknown operation"),
            }
        }
    }


    let mut wrong_bits = HashSet::new();

    for i in 0..=MAX_BIT {
        let (number_x, number_y) = (2_u64.pow(i), 0);
        let result = process_numbers(number_x, number_y, gates.clone());
        let expected = number_x + number_y;
        if result != expected {
            wrong_bits.insert(i);
        }
    }

    let mut processed_gates = HashSet::new();
    for bit in 0..MAX_BIT {
        let z_gate = format!("z{:02}", bit);
        let mut pending_to_processed = vec![z_gate];
        let mut dependent_gates = Vec::new();
        while !pending_to_processed.is_empty() {
            let g = pending_to_processed.remove(0);
            if processed_gates.contains(&g) || g.starts_with("x") || g.starts_with("y") {
                continue;
            }
            let option = output_to_gates.get(&g);
            let gate = option.unwrap();
            pending_to_processed.push(gate.input_1.clone());
            pending_to_processed.push(gate.input_2.clone());
            processed_gates.insert(g.clone());
            dependent_gates.push(g);
        }

        let expected_dependency = match bit {
            0 => 1,
            1 => 3,
            _ => 5,
        };

        if dependent_gates.len() != expected_dependency {
            println!("bit {} has wrong dependency length = {:?}", bit, dependent_gates);
        } else if wrong_bits.contains(&bit) {
            println!("bit {} has incorrect result with these dependencies = {:?}", bit, dependent_gates);
        } else if dependent_gates.iter().any(|g| invalid_internal_gate.iter().any(|invalid| invalid.output == **g)) {
            println!("bit {} has dependencies with invalid gate = {:?}", bit, dependent_gates);
        } else if bit > 0 && wrong_bits.contains(&(bit - 1)) {
            println!("  bit {} has these dependencies = {:?}", bit, dependent_gates);
        }
    }
}

fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let gate_pattern = Regex::new(r"(\w+) (XOR|OR|AND) (\w+) -> (\w+)").unwrap();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
    }

    let mut gates = HashMap::new();
    let mut maybe_wrong_outputs = HashSet::new();

    for line in lines {
        let captures = gate_pattern.captures(line).unwrap();
        let input_1 = captures.get(1).unwrap().as_str();
        let operation = captures.get(2).unwrap().as_str();
        let input_2 = captures.get(3).unwrap().as_str();
        let output = captures.get(4).unwrap().as_str();

        gates.insert(output.to_string(), Gate {
            input_1: input_1.to_string(),
            input_2: input_2.to_string(),
            operation: operation.to_string(),
            output: output.to_string(),
        });
        maybe_wrong_outputs.insert(output.to_string());
    }

    // informed by the potentially wrong gates reported by part2_tests
    let bit_9_fixes = ["pcd", "gws", "hcb", "nnt", "tqw"];
    let bit_13_fixes = ["z13", "npf", "tbd"];
    let bit_19_fixes = ["z19", "crr", "dgm", "cph"];
    let bit_33_fixes = ["z33", "wvn", "fvk", "hgj"];

    for a1 in 0..bit_9_fixes.len() {
        for a2 in a1 + 1..bit_9_fixes.len() {
            if a1 == a2 {
                continue;
            }
            let gates = swap_output(&gates, &bit_9_fixes[a1].to_string(), &bit_9_fixes[a2].to_string());

            for b1 in 0..bit_13_fixes.len() {
                for b2 in b1 + 1..bit_13_fixes.len() {
                    if b1 == b2 {
                        continue;
                    }
                    let gates = swap_output(&gates, &bit_13_fixes[b1].to_string(), &bit_13_fixes[b2].to_string());

                    for c1 in 0..bit_19_fixes.len() {
                        for c2 in c1 + 1..bit_19_fixes.len() {
                            if c1 == c2 {
                                continue;
                            }
                            let gates = swap_output(&gates, &bit_19_fixes[c1].to_string(), &bit_19_fixes[c2].to_string());

                            for d1 in 0..bit_33_fixes.len() {
                                for d2 in c1 + 1..bit_33_fixes.len() {
                                    if d1 == d2 {
                                        continue;
                                    }
                                    let gates = swap_output(&gates, &bit_33_fixes[d1].to_string(), &bit_33_fixes[d2].to_string());

                                    let mut h = HashSet::new();
                                    let bit_to_fix = correct_bit_count(&gates, &mut h);

                                    if bit_to_fix > MAX_BIT + 1 {
                                        let mut swaps = [
                                            bit_9_fixes[a1],
                                            bit_9_fixes[a2],
                                            bit_13_fixes[b1],
                                            bit_13_fixes[b2],
                                            bit_19_fixes[c1],
                                            bit_19_fixes[c2],
                                            bit_33_fixes[d1],
                                            bit_33_fixes[d2],
                                        ];
                                        swaps.sort();

                                        return swaps.join(",")
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    panic!("no swaps are working");
}

fn swap_output(gates: &HashMap<String, Gate>, output_1: &String, output_2: &String) -> HashMap<String, Gate> {
    let mut gate_1 = gates.get(output_1).unwrap().clone();
    let mut gate_2 = gates.get(output_2).unwrap().clone();
    gate_1.output = output_2.clone();
    gate_2.output = output_1.clone();

    let mut gates = gates.clone();
    gates.insert(output_2.clone(), gate_1);
    gates.insert(output_1.clone(), gate_2);

    gates
}

fn correct_bit_count(
    gates: &HashMap<String, Gate>,
    maybe_wrong_outputs: &mut HashSet<String>
) -> u32 {
    let output_to_inputs: HashMap<String, (String, String)> = gates.values().map(|gate| {
        (gate.output.clone(), (gate.input_1.clone(), gate.input_2.clone()))
    }).collect();
    let mut all_gates: Vec<Gate> = gates.values().cloned().collect();
    all_gates.sort_by(sort_gates_with_xy_at_end);
    let outputs = TEST_CASES.iter().map(|test_case| {
        let output = process_numbers(test_case.0, test_case.1, all_gates.clone());
        (output, test_case.0 + test_case.1)
    }).collect::<Vec<(u64, u64)>>();

    let mut bit = 0;

    loop {
        let mut correct = true;
        for (output, expected) in outputs.iter() {
            let output_bit = output >> bit & 1_u64;
            let expected_bit = expected >> bit & 1_u64;
            if output_bit != expected_bit {
                correct = false;
            }
        }

        if !correct {
            return bit;
        }

        let mut outputs = vec![format!("z{:02}", bit)];
        while let Some(output) = outputs.pop() {
            maybe_wrong_outputs.remove(&output);

            if output.starts_with("x") || output.starts_with("y") {
                continue;
            }
            let intput = output_to_inputs.get(&output).unwrap();
            outputs.push(intput.0.clone());
            outputs.push(intput.1.clone());
        }

        bit += 1;
        if bit > MAX_BIT + 1 {
            break;
        }
    }

    bit
}
