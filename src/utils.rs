use std::ops::Div;

use ndarray::{Array2, Axis, Zip};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::log;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_all_neighborhood(network: Vec<String>, genes: Vec<String>, n: usize) -> Vec<String> {
    log("Building graph");
    log(&network
        .iter()
        .take(10)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n"));
    let unique_nodes = network
        .iter()
        .flat_map(|line| line.split_whitespace().take(2))
        .collect::<std::collections::HashSet<_>>();
    log(&format!("Unique nodes: {:?}", unique_nodes.len()));
    let mut adj_matrix = Array2::zeros((unique_nodes.len(), unique_nodes.len()));
    log("HERE NOW");
    let mut node_map = std::collections::HashMap::new();
    let mut reverse_node_map = std::collections::HashMap::new();
    for (i, node) in unique_nodes.iter().enumerate() {
        node_map.insert(node.to_string(), i);
        reverse_node_map.insert(i, node.to_string());
    }
    log("What now?");
    for line in network.iter() {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        let weight = 1.0;
        let a_index = node_map.get(a).unwrap();
        let b_index = node_map.get(b).unwrap();
        adj_matrix[[*a_index, *b_index]] = weight;
        adj_matrix[[*b_index, *a_index]] = weight;
    }
    log("Running random walk");
    let node_indices: Vec<usize> = genes
        .iter()
        .map(|gene| node_map.get(gene).unwrap())
        .cloned()
        .collect();
    let walk_res = random_walk_probability(&adj_matrix, &node_indices, 0.5, 0.000001);
    let walk = walk_res.to_vec();
    let mut top_n = walk.iter().enumerate().collect::<Vec<_>>();
    top_n.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    top_n.truncate(n + 1);
    let top_n: Vec<String> = top_n
        .iter()
        .map(|(i, _p)| reverse_node_map[i].to_string())
        .collect();
    top_n
}

fn random_walk_probability(
    adj_matrix: &ndarray::Array2<f64>,
    node_indices: &Vec<usize>,
    r: f64,
    tolerance: f64,
) -> ndarray::Array1<f64> {
    let num_nodes = node_indices.len() as f64;
    let de = adj_matrix.sum_axis(Axis(0));
    // de to 2d array
    let de = de.insert_axis(Axis(1));
    let temp = adj_matrix.t().div(de);
    let w = temp.t();
    let mut p0 = ndarray::Array1::from_elem(w.shape()[0], 0.0);
    for i in node_indices {
        p0[*i] = 1.0 / num_nodes;
    }
    let mut pt = p0.clone();
    let mut pt1 = w.dot(&pt) * (1.0 - r) + (r * &p0);
    while Zip::from(&pt1)
        .and(&pt)
        .map_collect(|a, b| (a - b).abs())
        .sum()
        > tolerance
    {
        pt = pt1;
        pt1 = w.dot(&pt) * (1.0 - r) + (r * &p0);
    }
    pt1
}
