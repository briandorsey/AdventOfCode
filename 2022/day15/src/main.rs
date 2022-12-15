use color_eyre::eyre::Result;
use std::collections::HashSet;
use std::ops::RangeInclusive;

fn main() -> Result<()> {
    // test input
    let (sensors, beacons) = parse_input(&TEST_INPUT);
    let test_count = count_of_non_beacon(10, &sensors, &beacons);
    println!("part1 test: {}", test_count);

    let (sensors, beacons) = parse_input(&_INPUT);
    let count = count_of_non_beacon(2_000_000, &sensors, &beacons);
    println!("part1     : {}", count);

    // part 2
    let (sensors, _) = parse_input(&TEST_INPUT);
    let distress = find_beacon((0, 20), &sensors);
    println!(
        "part2 test: ({} * 4,000,000) + {} = {}",
        distress.0,
        distress.1,
        (distress.0 * 4_000_000) + distress.1
    );

    let (sensors, _) = parse_input(&_INPUT);
    let distress = find_beacon((0, 4_000_000), &sensors);
    println!(
        "part2     : ({} * 4,000,000) + {} = {}",
        distress.0,
        distress.1,
        (distress.0 as i128 * 4_000_000_i128) + distress.1 as i128
    ); // 12051287042458

    Ok(())
}

type Pos = (i32, i32);
type Sensors = Vec<(Pos, i32)>;
type Beacons = HashSet<Pos>;

fn parse_input(input: &[(Pos, Pos)]) -> (Sensors, Beacons) {
    let sensors: Vec<_> = input
        .iter()
        .map(|(sensor, beacon)| (*sensor, manhattan_distance(sensor, beacon)))
        .collect();
    let beacons: HashSet<_> = input.iter().map(|(_, beacon)| *beacon).collect();
    (sensors, beacons)
}

fn find_beacon(limits: (i32, i32), sensors: &Sensors) -> Pos {
    for (y, row) in (limits.0..=limits.1).enumerate() {
        let mut ranges: Vec<_> = sensors.iter().map(|s| overlap_ends(row, s)).collect();
        ranges.sort();
        //println!("before reduce: len: {}", ranges.len(),);
        let mut ranges = reduce_overlaps(&ranges);
        //println!("after reduce: len: {}", ranges.len(),);

        // trim to limits
        for r in &mut ranges {
            if r.0 < limits.0 {
                r.0 = limits.0
            }
            if r.1 > limits.1 {
                r.1 = limits.1
            }
        }

        for _offset in ranges.iter().take(25) {
            //print!(", {:?}", _offset,);
        }
        //println!();

        let exclude_count = ranges
            .iter()
            .map(positions_in_range)
            //.inspect(|sum| print!(", {sum}"))
            .sum::<i32>();
        //println!("{exclude_count}");
        // actually just finding the first case of exactly one uncovered position. :/
        if exclude_count == limits.1 {
            return (ranges[0].1 + 1, y.try_into().unwrap());
        }
    }
    (-123, -123)
}

fn count_of_non_beacon(row: i32, sensors: &Sensors, beacons: &Beacons) -> i32 {
    let mut ranges: Vec<_> = sensors.iter().map(|s| overlap_ends(row, s)).collect();
    ranges.sort();
    //println!("before reduce: len: {}", ranges.len(),);
    let ranges = reduce_overlaps(&ranges);
    //println!("after reduce: len: {}", ranges.len(),);

    for _offset in ranges.iter().take(20) {
        //print!(", {:?}", _offset,);
    }
    //println!();

    // remove beacons
    let mut beacons_in_range = 0;
    for b in beacons {
        if b.1 == row {
            for range in &ranges {
                if range.0 <= b.0 && range.1 >= b.0 {
                    beacons_in_range += 1;
                }
            }
        }
    }
    //println!("beacons_in_range: {beacons_in_range}");

    ranges
        .iter()
        .map(positions_in_range)
        //.inspect(|sum| print!(", {sum}"))
        .sum::<i32>()
        - beacons_in_range
}

fn positions_in_range(range: &Pos) -> i32 {
    // + 1 for inclusive range counting
    (range.0 - range.1).abs() + 1
}

fn manhattan_distance(a: &Pos, b: &Pos) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn _overlap(row: i32, sensor: &(Pos, i32)) -> Vec<i32> {
    let remainder = sensor.1 - (row - sensor.0 .1).abs();
    RangeInclusive::new(sensor.0 .0 - remainder, sensor.0 .0 + remainder).collect()
}

fn overlap_ends(row: i32, sensor: &(Pos, i32)) -> Pos {
    let remainder = sensor.1 - (row - sensor.0 .1).abs();
    (sensor.0 .0 - remainder, sensor.0 .0 + remainder)
}

fn reduce_overlaps(ranges: &Vec<Pos>) -> Vec<Pos> {
    let mut output = Vec::new();
    if ranges.is_empty() {
        return output;
    }
    let mut ranges = ranges.clone();
    ranges.sort();
    let mut tmp = ranges[0];
    for r in ranges {
        if tmp.0 <= r.0 && tmp.1 >= r.1 {
            // tmp contains r
            continue;
        } else if tmp.1 < r.0 {
            // non contiguous
            output.push(tmp);
            tmp = r;
        } else if tmp.0 >= r.0 && tmp.0 <= r.1 && tmp.1 < r.1 {
            // tmp.0 in r, tmp.1 to the right --> extend tmp
            tmp.1 = r.1;
        } else if tmp.1 >= r.0 && tmp.1 <= r.1 && tmp.0 > r.0 {
            // tmp.1 in r, tmp.0 to the left --> extend tmp
            tmp.0 = r.0;
        } else if tmp.0 < r.0 && tmp.0 <= r.1 && tmp.1 < r.1 {
            // tmp.0 left of r, tmp.1 in r --> extend tmp
            tmp.1 = r.1;
        } else {
            unreachable!("{tmp:?}, {r:?}");
        }
    }
    output.push(tmp);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(12, manhattan_distance(&(0, 0), &(6, 6)));
        assert_eq!(12, manhattan_distance(&(0, 0), &(-6, -6)));
        assert_eq!(12, manhattan_distance(&(3, 3), &(9, 9)));
        assert_eq!(12, manhattan_distance(&(3, 3), &(-3, -3)));
    }

    #[test]
    fn test_overlap() {
        assert_eq!(vec![-2, -1, 0, 1, 2], overlap(10, &((0, 11), 3)));
        assert_eq!(vec![2], overlap(10, &((2, 0), 10)));
        assert_eq!(vec![12], overlap(10, &((12, 14), 4)));
        assert_eq!(13, overlap(10, &((8, 7), 9)).len());
    }

    #[test]
    fn test_puzzle_answers_valid() {
        let (sensors, beacons) = parse_input(&TEST_INPUT);
        assert_eq!(26, count_of_non_beacon(10, &sensors, &beacons));

        let (sensors, beacons) = parse_input(&_INPUT);
        assert_eq!(4725496, count_of_non_beacon(2_000_000, &sensors, &beacons));
    }

    #[test]
    fn test_reduce_overlaps() {
        assert_eq!(Vec::<Pos>::new(), reduce_overlaps(&Vec::<Pos>::new()));
        assert_eq!(vec![(1, 10)], reduce_overlaps(&vec![(1, 6), (4, 10)]));
        assert_eq!(vec![(1, 10)], reduce_overlaps(&vec![(4, 10), (1, 6)]));
        // contains with overlapping endpoint
        assert_eq!(vec![(1, 10)], reduce_overlaps(&vec![(1, 10), (4, 10)]));
        // two separate ranges
        assert_eq!(
            vec![(1, 4), (7, 10)],
            reduce_overlaps(&vec![(1, 4), (7, 10)])
        );
    }

    #[test]
    fn test_positions_in_range() {
        assert_eq!(6, positions_in_range(&(1, 6)), "(1, 6)");
        assert_eq!(6, positions_in_range(&(0, 5)), "(0, 5)");
        assert_eq!(5, positions_in_range(&(-2, 2)), "(-2, 2)");
        assert_eq!(26, positions_in_range(&(-2, 23)), "(-2, 23)");
    }
}

const TEST_INPUT: [(Pos, Pos); 14] = [
    ((2, 18), (-2, 15)),
    ((9, 16), (10, 16)),
    ((13, 2), (15, 3)),
    ((12, 14), (10, 16)),
    ((10, 20), (10, 16)),
    ((14, 17), (10, 16)),
    ((8, 7), (2, 10)),
    ((2, 0), (2, 10)),
    ((0, 11), (2, 10)),
    ((20, 14), (25, 17)),
    ((17, 20), (21, 22)),
    ((16, 7), (15, 3)),
    ((14, 3), (15, 3)),
    ((20, 1), (15, 3)),
];

const _INPUT: [(Pos, Pos); 33] = [
    ((1943362, 12808), (1861152, -42022)),
    ((906633, 3319637), (2096195, 3402757)),
    ((2358896, 2158796), (2331052, 2934800)),
    ((1787606, 3963631), (2096195, 3402757)),
    ((2282542, 3116014), (2331052, 2934800)),
    ((173912, 1873897), (429790, 2000000)),
    ((3391153, 3437167), (3720655, 3880705)),
    ((3834843, 2463103), (2971569, 2563051)),
    ((3917316, 3981011), (3720655, 3880705)),
    ((1466100, 1389028), (429790, 2000000)),
    ((226600, 3967233), (85598, 4102832)),
    ((1757926, 2834180), (2331052, 2934800)),
    ((2176953, 3240563), (2096195, 3402757)),
    ((2883909, 2533883), (2971569, 2563051)),
    ((376161, 2533578), (429790, 2000000)),
    ((3015271, 3913673), (3720655, 3880705)),
    ((490678, 388548), (429790, 2000000)),
    ((2725765, 2852933), (2331052, 2934800)),
    ((86373, 2839828), (429790, 2000000)),
    ((1802070, 14830), (1861152, -42022)),
    ((19628, 1589839), (429790, 2000000)),
    ((2713787, 3381887), (2096195, 3402757)),
    ((2148471, 3729393), (2096195, 3402757)),
    ((3999318, 3263346), (3720655, 3880705)),
    ((575700, 1390576), (429790, 2000000)),
    ((273266, 2050976), (429790, 2000000)),
    ((3008012, 993590), (2971569, 2563051)),
    ((3306379, 2782128), (2971569, 2563051)),
    ((44975, 3820788), (85598, 4102832)),
    ((2941700, 2536797), (2971569, 2563051)),
    ((2040164, 102115), (1861152, -42022)),
    ((3928008, 3692684), (3720655, 3880705)),
    ((3905950, 222812), (4759853, -796703)),
];
