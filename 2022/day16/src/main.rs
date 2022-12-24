use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;
use petgraph::algo::astar;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::DfsPostOrder;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use tracing::{info, trace, Level};
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

    // get minimum cost path from AA to HH
    let (_, path) = astar(
        &graph,
        nodes["AA"],
        |finish| finish == nodes["HH"],
        |e| *e.weight(),
        |_| 0,
    )
    .ok_or(eyre!("astar failed"))?;
    trace!(
        "AA-->HH: {:?}",
        &path
            .iter()
            .map(|n| graph.node_weight(*n).unwrap().id.clone())
            .collect::<Vec<_>>()
    );
    trace!(
        "AA-->HH: {:?}",
        &path
            .iter()
            .map(|n| graph.node_weight(*n).unwrap().flow_rate)
            .collect::<Vec<_>>()
    );

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

#[derive(Debug, Clone)]
struct Path {
    actions: Vec<Action>,
    open: HashSet<ValveID>,
    _visited: HashSet<ValveID>,
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

fn total_pressure(graph: &Graph<Action, i32>, start: NodeIndex) -> (i32, i32, i32) {
    trace!("total_pressure");
    let mut minute = -1;
    let mut total = 0;
    let mut flow_rate = 0;
    let mut just_opened = 0;
    let mut dfs = DfsPostOrder::new(&graph, start);
    while let Some(nx) = dfs.next(&graph) {
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

        assert_eq!(1651, total_pressure(&graph, prev).0);
    }
}
