use color_eyre::eyre::Result;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::{HashMap, HashSet};
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
    info!("{input:?}");
    let input = fs::read_to_string(input)?;

    let mut graph: Graph<Valve, i32, petgraph::Directed> = Graph::new();
    let mut nodes: HashMap<ValveID, NodeIndex> = HashMap::new();
    let mut edges: Vec<(ValveID, ValveID)> = Vec::new();
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
    trace!("graph.is_directed(): {}", graph.is_directed());
    trace!("graph.node_count(): {}", graph.node_count());
    trace!("graph.edge_count(): {}", graph.edge_count());

    trace!("");
    //println!("{}", petgraph::dot::Dot::new(&graph));
    //println!("{:#?}", &graph);

    // summarize nodes - remove 0 rate nodes with only two peers
    if false {
        graph.retain_nodes(|g, n| {
            trace!("{:?}", n);
            let valve = &g[n];
            if valve.flow_rate == 0 && g.neighbors(n).collect::<Vec<_>>().len() == 2 {
                trace!("  {:?}", valve);
                for edge in g.edges_directed(n, petgraph::Outgoing) {
                    trace!("    O: {:?}", edge);
                }
                for edge in g.edges_directed(n, petgraph::Incoming) {
                    trace!("    I: {:?}", edge);
                }
                for neighbor in g.neighbors(n) {
                    trace!("    {:?}", g[neighbor]);
                }
            }
            true
        });
    }

    //let mut paths: Graph<Action, (), petgraph::Directed> = Graph::new();
    //paths.add_node(Action::Start(String::from("AA")));
    let mut paths: Vec<Path> = Vec::new();
    paths.push(Path {
        actions: vec![Action::Start(String::from("AA"))],
        open: HashSet::new(),
        visited: HashSet::new(),
    });
    //let mut positions: Vec<NodeIndex> = Vec::new();
    //positions.push(nodes["AA"]);
    let node_count = graph.node_count();
    let mut completed_paths: Vec<Path> = Vec::new();

    for iteration in 1..=25 {
        debug!("== Minute {iteration} ==");
        let mut next_paths: Vec<Path> = Vec::new();

        for path in &mut paths {
            if path.visited.len() == node_count {
                completed_paths.push(path.clone());
                continue;
            }
            trace!("-- {path}");
            let i = path.actions.len() - 1;
            match &mut path.actions[i] {
                Action::MoveTo(id)
                    if &graph[nodes[id]].flow_rate > &0 && !path.open.contains(id) =>
                {
                    let valve = &graph[nodes[id]];
                    trace!("---- open valve! {valve}");
                    let new_path = &mut path.clone();
                    new_path.actions.push(Action::Open(
                        valve.id.to_string(),
                        graph[nodes[&valve.id]].flow_rate,
                    ));
                    next_paths.push(new_path.clone());
                    continue;
                }
                Action::Start(id) | Action::Open(id, _) | Action::MoveTo(id) => {
                    let valve = &graph[nodes[id]];
                    trace!("---- {valve}");
                    for neighbor in graph.neighbors(nodes[id]) {
                        trace!("  \\-->{:?}", &graph[neighbor].id);
                        let new_path = &mut path.clone();
                        new_path.visited.insert(valve.id.to_string());
                        new_path
                            .actions
                            .push(Action::MoveTo(graph[neighbor].id.to_string()));
                        next_paths.push(new_path.clone());
                    }
                }
            }
        }
        paths = next_paths;
        debug!(
            "   paths: {}    complete: {}",
            paths.len(),
            completed_paths.len()
        );
    }

    Ok(())
}

//fn parse_input(input: &str) ->

#[derive(Debug, Clone)]
enum Action {
    Start(ValveID),
    MoveTo(ValveID),
    Open(ValveID, FlowRate),
}
impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Action::Start(id) => write!(f, "S({id})"),
            Action::MoveTo(id) => write!(f, "M({id})"),
            Action::Open(id, fr) => write!(f, "O({id}, {fr})"),
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    actions: Vec<Action>,
    open: HashSet<ValveID>,
    visited: HashSet<ValveID>,
    //flow_rate: FlowRate,
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "P:({}):{:?}",
            self.actions
                .iter()
                .map(|a| format!("{}", a))
                .collect::<Vec<_>>()
                .join(","),
            self.open,
        )
    }
}

type ValveID = String;
type FlowRate = i32;

#[derive(Clone)]
enum State {
    _Open,
    _Closed,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(true);
    }
}
