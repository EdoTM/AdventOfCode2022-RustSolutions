use crate::utils;

pub fn day13_solution(example: bool) {
    let lines = utils::get_lines_from_input_file(13, example);

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut lines_iter = lines.iter();
    let mut current_index = 0u32;
    let mut correct_indexes_sum = 0u32;

    loop {
        current_index += 1;
        let left = lines_iter.next().unwrap();
        let right = lines_iter.next().unwrap();

        let left_packet = create_packet_from_str(left);
        let right_packet = create_packet_from_str(right);

        if packets_are_in_the_right_order(&left_packet, &right_packet) {
            correct_indexes_sum += current_index;
        }

        if lines_iter.next().is_none() {
            break;
        }
    }

    utils::print_part_solution(1, correct_indexes_sum);
}

fn part2(lines: &Vec<String>) {
    let mut packets: Vec<Packet> = Vec::new();
    
    for line in lines {
        if line.is_empty() {
            continue;
        }
        packets.push(create_packet_from_str(line));
    }

    let first_marker = create_packet_from_str("[[2]]");
    let second_marker = create_packet_from_str("[[6]]");

    packets.push(first_marker.clone());
    packets.push(second_marker.clone());
    
    packets.sort_by(|l, r| get_packets_order(l, r).to_std_ordering());

    let mut product_of_marker_indexes = 1usize;
    for (i, packet) in packets.iter().enumerate() {
        if get_packets_order(packet, &first_marker) == PacketOrder::SameOrder {
            product_of_marker_indexes *= i + 1;
        }
        else if get_packets_order(packet, &second_marker) == PacketOrder::SameOrder {
            product_of_marker_indexes *= i + 1;
            break;
        }
    }

    utils::print_part_solution(2, product_of_marker_indexes);
}


fn create_packet_from_str(line: &str) -> Packet {
    let mut tokens = line.chars();
    assert_eq!(tokens.next(), Some('['));

    Packet::List(create_packet_recursively(&mut tokens))
}

fn create_packet_recursively(tokens: &mut core::str::Chars) -> Vec<Packet> {
    let mut list: Vec<Packet> = Vec::new();
    let mut cur_number = String::new();
    loop {
        match tokens.next() {
            Some('[') => {
                let subpacket = create_packet_recursively(tokens);
                let sublist = Packet::List(subpacket);
                list.push(sublist);
            }
            Some(']') => {
                push_value_to_list(&mut cur_number, &mut list);
                return list;
            }
            Some(',') => push_value_to_list(&mut cur_number, &mut list),
            Some(n) => cur_number.push(n),
            None => return list,
        }
    }
}

fn push_value_to_list(value: &mut String, list: &mut Vec<Packet>) {
    if !value.is_empty() {
        let n: u32 = value.parse().unwrap();
        list.push(Packet::Value(n));
        value.clear();
    }
}


fn packets_are_in_the_right_order(left: &Packet, right: &Packet) -> bool {
    get_packets_order(left, right) == PacketOrder::RightOrder
}

fn get_packets_order(left: &Packet, right: &Packet) -> PacketOrder {
    use Packet::*;

    match (left, right) {
        (List(l), List(r)) => compare_vectors(l, r),
        (Value(l), Value(r)) => compare_values(l, r),
        (List(_), Value(_)) => get_packets_order(left, &right.convert()),
        (Value(_), List(_)) => get_packets_order(&left.convert(), right),
    }
}

fn compare_vectors(left_vector: &Vec<Packet>, right_vector: &Vec<Packet>) -> PacketOrder {
    let mut left_iter = left_vector.iter();
    let mut right_iter = right_vector.iter();
    loop {
        match (left_iter.next(), right_iter.next()) {
            (Some(l), Some(r)) => {
                let order = get_packets_order(l, r);
                if order != PacketOrder::SameOrder {
                    return order;
                }
            }
            (None, Some(_)) => return PacketOrder::RightOrder,
            (Some(_), None) => return PacketOrder::WrongOrder,
            (None, None) => return PacketOrder::SameOrder,
        }
    }
}

fn compare_values(left_value: &u32, right_value: &u32) -> PacketOrder {
    if left_value < right_value {
        PacketOrder::RightOrder
    } else if left_value > right_value {
        PacketOrder::WrongOrder
    } else {
        PacketOrder::SameOrder
    }
}


#[derive(PartialEq)]
enum PacketOrder {
    RightOrder,
    WrongOrder,
    SameOrder,
}
impl PacketOrder {
    pub fn to_std_ordering(&self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match self {
            Self::RightOrder => Ordering::Less,
            Self::WrongOrder => Ordering::Greater,
            Self::SameOrder => Ordering::Equal,
        }
    }
}


#[derive(Clone)]
enum Packet {
    List(Vec<Packet>),
    Value(u32),
}
impl Packet {
    pub fn convert(&self) -> Packet {
        match self {
            Self::Value(v) => {
                let vec = vec![Self::Value(*v)];
                return Self::List(vec);
            }
            Self::List(_) => panic!("Cannot convert a list."),
        }
    }
}
