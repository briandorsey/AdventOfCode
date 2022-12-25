use color_eyre::eyre::Result;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::DfsPostOrder;
use std::fmt::{Display, Formatter};

type ValveID = String;
type FlowRate = i32;

#[allow(dead_code)] //todo: remove
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

fn total_pressure(graph: &Graph<Action, i32>, start: NodeIndex) -> (i32, i32, i32) {
    println!("total_pressure");
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
        println!(
            "{minute:>3}: {:<12} --> fr: {flow_rate:>4}, total: {total:>4}",
            format!("{}", graph[nx])
        );
    }
    (total, flow_rate, minute)
}

fn main() -> Result<()> {
    // proof of concept for calculating the total pressure relieved.
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

    Ok(())
}
