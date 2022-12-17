use color_eyre::eyre::Result;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::{Display, Formatter};
use std::fs;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    //println!("{input:?}");
    let input = fs::read_to_string(input)?;

    let mut valves = HashMap::<ValveID, Valve>::new();
    for line in input.lines() {
        let line = line.replace("Valve ", "");
        let line = line.replace("tunnels", "tunnel");
        let line = line.replace("valves", "valve");
        let line = line.replace("leads", "lead");
        //println!("{line:?}");
        let (id, line) = line.split_once(" has flow rate=").expect("parsing id");
        let (flow_rate, line) = line
            .split_once("; tunnel lead to valve ")
            .expect("parsing flow_rate");
        let tunnels: Vec<_> = line.split(", ").map(|e| (1, e.to_string())).collect();
        let valve = Valve {
            id: id.to_string(),
            flow_rate: flow_rate.parse::<i32>().expect("flow_rate to int"),
            tunnels: tunnels,
        };
        //println!("  --> {valve}");
        // diy HashMap graph
        valves.insert(id.to_string(), valve.clone());
    }

    //let valves = _collapse_zeros(valves);

    // lets make a dot file by hand
    if false {
        println!("digraph G {{\nAA [style=bold] \n");
        for valve in valves.values() {
            println!(
                "{} [label=\"{} fr{}\"];",
                valve.id, valve.id, valve.flow_rate
            );
            for t in &valve.tunnels {
                println!("{} -> {} [label=\"s{}\"];", valve.id, t.1, t.0);
            }
        }
        println!("}}");
    }

    //candidate paths - for now, let's just try to fill out all possible options.
    if false {
        let max_iterations = 3;
        // let mut paths: Vec<Action> = Vec::new();
        let mut positions = vec![String::from("AA")];
        for i in 1..=max_iterations {
            let tmp: Vec<ValveID> = Vec::new();
            for pos in positions.drain(..) {
                println!("{i}: {} {:?}", valves[&pos], valves[&pos].tunnels);
                //tmp.extend(valves[&pos].tunnels.clone());
            }
            //println!("positions.len(): {}", positions.len());
            positions = tmp;
        }
    }

    Ok(())
}

// ok, giving up now. I somehow forgot that most nodes have bidirectional links
fn _collapse_zeros(mut valves: HashMap<String, Valve>) -> HashMap<String, Valve> {
    println!("collapse_zeros()");
    let mut found = 0;
    let reference = valves.clone();
    let mut output = HashMap::new();
    let mut orphans: Vec<ValveID> = Vec::new();
    for (k, mut v) in valves.drain() {
        println!("{}", v);
        if v.tunnels.len() == 1 {
            println!("tunnels.len() = 1 {}", v);
            for t in v.tunnels.iter_mut() {
                println!("{:?} fr: {}", t, reference[&t.1].flow_rate,);
                if reference[&t.1].flow_rate == 0 {
                    println!("{:?}", t);
                    orphans.push(t.1.clone());
                    found += 1;
                    t.0 += reference[&t.1].tunnels[0].0;
                    t.1 = reference[&t.1].tunnels[0].1.clone();
                }
            }
        }
        output.insert(k, v);
    }
    //for o in orphans {
    //output.remove(&o);
    //}

    println!("-- end collapse_zeros()");
    if found > 0 {
        _collapse_zeros(output)
    } else {
        output
    }
}

//fn parse_input(input: &str) ->

enum _Action {
    MoveTo(ValveID),
    Open(ValveID, FlowRate),
}

struct _Path {
    actions: Vec<_Action>,
    open: HashSet<ValveID>,
    flow_rate: FlowRate,
}

type ValveID = String;
type FlowRate = i32;

#[derive(Clone)]
enum State {
    _Open,
    _Closed,
}

#[derive(Clone)]
struct Valve {
    id: ValveID,
    flow_rate: FlowRate,
    tunnels: Vec<(u32, ValveID)>,
}

impl Display for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}:{}->{}",
            self.id,
            self.flow_rate,
            self.tunnels
                .iter()
                .map(|t| { format!("{}s{}", t.0, t.1) })
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
}
