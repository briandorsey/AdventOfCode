use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;
use itertools::chain;
use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::DfsPostOrder;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use tracing::{debug, info, trace, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    let input = env::args_os().nth(1).expect("need input file name");
    let level = match env::args().nth(2) {
        Some(e) => match e.as_str() {
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO,
        },
        None => Level::INFO,
    };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .without_time()
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing default subscriber failed");
    info!("");
    debug!("{input:?}");
    let input = fs::read_to_string(input)?;

    let mut graph: Graph<Valve, i32, petgraph::Directed> = Graph::new();
    let mut nodes: HashMap<ValveID, NodeIndex> = HashMap::new();
    let mut edges: Vec<(ValveID, ValveID)> = Vec::new();
    let mut valve_locs: HashSet<ValveID> = HashSet::new();
    for line in input.lines() {
        let line = line.replace("Valve ", "");
        let line = line.replace("tunnels", "tunnel");
        let line = line.replace("valves", "valve");
        let line = line.replace("leads", "lead");
        //println!("{line:?}");
        let (valveid, line) = line.split_once(" has flow rate=").expect("parsing id");
        let (flow_rate, line) = line
            .split_once("; tunnel lead to valve ")
            .expect("parsing flow_rate");

        let tunnels: Vec<_> = line
            .split(", ")
            .map(|e| (valveid.to_string(), e.to_string()))
            .collect();
        edges.extend(tunnels);

        let valve = Valve {
            id: valveid.to_string(),
            flow_rate: flow_rate.parse::<i32>().expect("flow_rate to int"),
        };
        if valve.flow_rate > 0 {
            valve_locs.insert(valveid.to_string());
        }
        let node = graph.add_node(valve);
        nodes.insert(valveid.to_string(), node);

        //println!("  --> {valve}");
    }
    for (a, b) in edges {
        //println!("{a:?} --> {b:?}");
        graph.add_edge(nodes[&a], nodes[&b], 1);
    }

    // graph info
    //graph.remove_node(nodes["II"]);
    debug!("graph.is_directed(): {}", graph.is_directed());
    debug!("graph.node_count(): {}", graph.node_count());
    debug!("graph.edge_count(): {}", graph.edge_count());
    debug!("valve_locs: {:?}", valve_locs);
    debug!("");
    //println!("{}", petgraph::dot::Dot::new(&graph));
    //println!("{:#?}", &graph);

    // adding both directions... both graphs appear to have paired to/from edges
    let combinations: HashSet<_> = chain(&valve_locs, vec![&"AA".to_string()])
        .combinations(2)
        .flat_map(|e| {
            let mut tmp = e.clone();
            tmp.sort();
            [
                (tmp[0].clone(), tmp[1].clone()),
                (tmp[1].clone(), tmp[0].clone()),
            ]
        })
        .collect();
    //debug!("combinations: {:?}", combinations);

    // find all the shortest routes between combinations of valves
    // only considering nodes with valves for routing - may not be the absolute shortest route
    // for all inputs? But I think it works for these graphs.
    let mut min_paths: HashMap<ValveID, Vec<(ValveID, Vec<NodeIndex>)>> = HashMap::new();

    for (begin, end) in combinations {
        // get minimum cost path from begin to end
        let (_, path) = astar(
            &graph,
            nodes[&begin],
            |finish| finish == nodes[&end],
            |e| *e.weight(),
            |_| 0,
        )
        .ok_or(eyre!("astar failed"))?;
        trace!(
            "{}-->{}: {:?}",
            begin,
            end,
            &path
                .iter()
                .map(|n| graph.node_weight(*n).unwrap().id.clone())
                .collect::<Vec<_>>()
        );
        trace!(
            "{}-->{}: {:?}",
            begin,
            end,
            &path
                .iter()
                .map(|n| graph.node_weight(*n).unwrap().flow_rate)
                .collect::<Vec<_>>()
        );

        let cost = (end.clone(), path);
        min_paths.entry(begin.clone()).or_default().push(cost);
    }
    trace!("min_paths: {:?}", min_paths);

    // walk all the variations on paths possible, creating actions DAG
    // variations: all combinations of AA to somewhere, with total action length <=30

    debug!("walking...");
    let mut paths: Graph<Action, i32, petgraph::Directed> = Graph::new();
    let root = paths.add_node(Action::Start("AA".to_string()));
    let mut ends: VecDeque<NodeIndex> = VecDeque::new();
    ends.push_front(root);
    let mut completed: VecDeque<NodeIndex> = VecDeque::new();

    let mut temp_counter = 0; // TODO: end the loop by detecting path lengths
    loop {
        temp_counter += 1;
        trace!("walking... loop {temp_counter}");
        // avoid runaways
        if temp_counter > 50_000_000 {
            debug!("break, current ends: {ends:?}");
            break;
        };
        let Some(step) = ends.pop_back() else {break};
        trace!("at step: {:?}", &paths[step]);
        let step_count = paths_step_count(&paths, step);
        // part 1 limit
        if step_count >= 30 {
            // for add paths that hit this many steps to completed
            completed.push_back(step);
            continue;
        }

        let opened_valves = paths_open_valves(&paths, step);
        trace!("opened_valves: {:?}", opened_valves);

        // TODO: if all valves open, then wait
        if let Action::Move(id) = &paths[step] {
            // open this valve if it hasn't already been opened
            if !opened_valves.contains(id) {
                trace!("Open({id})");
                assert!(graph[nodes[id]].flow_rate != 0);
                let n = paths.add_node(Action::Open(id.to_string(), graph[nodes[id]].flow_rate));
                paths.add_edge(n, step, 1);
                ends.push_front(n);
                continue;
            } else {
                panic!("Somehow ended up at an already opened valve: {id}")
            }
        }

        // awkward convert to let/else?
        match &paths[step] {
            Action::Start(id) | Action::Open(id, _) => {
                // find all of the possible next steps and add them to the paths graph
                let candidates: HashSet<_> = valve_locs.difference(&opened_valves).collect();
                trace!("candidates: {:?}", candidates);
                if candidates.is_empty() {
                    // duplicated in Wait arm. :/
                    let n = paths.add_node(Action::Wait(id.to_string()));
                    paths.add_edge(n, step, 1);
                    ends.push_front(n);
                    continue;
                }

                for (dest, route) in min_paths.get(id).unwrap() {
                    if !candidates.contains(dest) {
                        continue;
                    }
                    trace!("Move({dest}), {route:?}");
                    let mut prev = step;
                    for node in &route[1..] {
                        let n = paths.add_node(Action::Move(graph[*node].id.to_string()));
                        paths.add_edge(n, prev, 1);
                        prev = n;
                    }
                    ends.push_front(prev);
                }
            }
            Action::Wait(id) => {
                let n = paths.add_node(Action::Wait(id.to_string()));
                paths.add_edge(n, step, 1);
                ends.push_front(n);
                continue;
            }
            Action::Move(_) => unreachable!(),
        };
    }

    // nodes remaining in ends should be the list of starts of all paths

    debug!("ends: {:?}", ends.len());
    debug!("completed: {:?}", completed.len());

    let mut max_pressure: (i32, NodeIndex) = (0, root);
    for node in completed.drain(..) {
        let pressure = paths_total_pressure(&paths, node).0;
        if pressure > max_pressure.0 {
            max_pressure = (pressure, node);
        }
    }
    debug!("max_pressure: {:?}", max_pressure);
    //_paths_step_print(&paths, max_pressure.1);

    //println!("{}", petgraph::dot::Dot::new(&paths));

    // 3203 - too high  5:58 to run

    Ok(())
}

#[derive(Debug, Clone)]
enum Action {
    Start(ValveID),
    Move(ValveID),
    Open(ValveID, FlowRate),
    Wait(ValveID),
}
impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Action::Start(id) => write!(f, "Start({id})"),
            Action::Move(id) => write!(f, "Move({id})"),
            Action::Open(id, fr) => write!(f, "Open({id},{fr})"),
            Action::Wait(id) => write!(f, "Wait({id})"),
        }
    }
}

type ValveID = String;
type FlowRate = i32;

#[derive(Clone, Debug)]
struct Valve {
    id: ValveID,
    flow_rate: FlowRate,
}

impl Display for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{}", self.id, self.flow_rate,)
    }
}

fn _paths_step_print(graph: &Graph<Action, i32>, start: NodeIndex) {
    let mut count = -1;
    let mut dfs = DfsPostOrder::new(graph, start);
    while let Some(nx) = dfs.next(graph) {
        count += 1;
        println!("{:>3}: {}", count, graph[nx]);
    }
}

fn paths_step_count(graph: &Graph<Action, i32>, start: NodeIndex) -> i32 {
    let mut count = -1;
    let mut dfs = DfsPostOrder::new(graph, start);
    while dfs.next(graph).is_some() {
        count += 1;
    }
    count
}

fn paths_open_valves(graph: &Graph<Action, i32>, start: NodeIndex) -> HashSet<ValveID> {
    let mut opened = HashSet::new();
    let mut dfs = DfsPostOrder::new(graph, start);
    while let Some(nx) = dfs.next(graph) {
        if let Action::Open(id, _) = &graph[nx] {
            opened.insert(id.to_string());
        }
    }
    opened.insert("AA".to_string()); // treat AA as always opened
    opened
}

fn paths_total_pressure(graph: &Graph<Action, i32>, start: NodeIndex) -> (i32, i32, i32) {
    trace!("total_pressure");
    let mut minute = -1;
    let mut total = 0;
    let mut flow_rate = 0;
    let mut just_opened = 0;
    let mut dfs = DfsPostOrder::new(graph, start);
    while let Some(nx) = dfs.next(graph) {
        match graph[nx] {
            Action::Open(_, fr) => just_opened += fr,
            _ => {
                flow_rate += just_opened;
                just_opened = 0
            }
        }
        minute += 1;
        total += flow_rate;
        // we can access `graph` mutably here still
        trace!(
            "{minute:>3}: {:<12} --> fr: {flow_rate:>4}, total: {total:>4}",
            format!("{}", graph[nx])
        );
    }
    (total, flow_rate, minute)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _init() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .without_time()
            .with_target(false)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting tracing default subscriber failed");
    }

    #[test]
    fn test_total_pressure() {
        //_init();
        let actions = vec![
            (1, Action::Move("DD".to_string())),
            (2, Action::Open("DD".to_string(), 20)),
            (3, Action::Move("CC".to_string())),
            (4, Action::Move("BB".to_string())),
            (5, Action::Open("BB".to_string(), 13)),
            (6, Action::Move("AA".to_string())),
            (7, Action::Move("II".to_string())),
            (8, Action::Move("JJ".to_string())),
            (9, Action::Open("JJ".to_string(), 21)),
            (10, Action::Move("II".to_string())),
            (11, Action::Move("AA".to_string())),
            (12, Action::Move("DD".to_string())),
            (13, Action::Move("EE".to_string())),
            (14, Action::Move("FF".to_string())),
            (15, Action::Move("GG".to_string())),
            (16, Action::Move("HH".to_string())),
            (17, Action::Open("HH".to_string(), 22)),
            (18, Action::Move("GG".to_string())),
            (19, Action::Move("FF".to_string())),
            (20, Action::Move("EE".to_string())),
            (21, Action::Open("EE".to_string(), 3)),
            (22, Action::Move("DD".to_string())),
            (23, Action::Move("CC".to_string())),
            (24, Action::Open("CC".to_string(), 2)),
            (25, Action::Wait("CC".to_string())),
            (26, Action::Wait("CC".to_string())),
            (27, Action::Wait("CC".to_string())),
            (28, Action::Wait("CC".to_string())),
            (29, Action::Wait("CC".to_string())),
            (30, Action::Wait("CC".to_string())),
        ];

        let mut graph: Graph<Action, i32, petgraph::Directed> = Graph::new();
        let root = graph.add_node(Action::Start("AA".to_string()));

        let mut prev = root;
        for action in actions {
            let n = graph.add_node(action.1);
            graph.add_edge(n, prev, 1);
            prev = n;
        }

        // manually add another path to root, to make sure it doesn't confuse things
        {
            let n = graph.add_node(Action::Move("TEST".to_string()));
            graph.add_edge(n, root, 1);
        }
        //println!("{}", petgraph::dot::Dot::new(&graph));
        //println!("{:#?}", &graph);

        assert_eq!(1651, paths_total_pressure(&graph, prev).0);
    }
}
