#![allow(non_snake_case)]
use fastset::Set;
use outils::graph::dynconn::hdt::DynamicGraph;
use outils::prelude::DynamicConnectivity;
use outils::prelude::VertexIndex;
use outils::types::EmptyWeight;
use std::{cmp::max, io, vec::Vec};
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

fn read_int(input: &mut String) -> usize {
    *input = "".to_string();

    let _ = io::stdin().read_line(input);
    return input.trim().parse::<usize>().unwrap();
}

fn read_line_of_ints(input: &mut String) -> Vec<usize> {
    *input = "".to_string();

    io::stdin().read_line(input).unwrap();
    input
        .trim()
        .split(" ")
        .map(|number| number.trim().parse::<usize>().unwrap())
        .collect()
}

fn minium_spanning_tree(
    edges_with_order: &Vec<(usize, usize, usize, usize)>,
    sorted_edges: &Vec<(usize, usize, usize, usize)>,
    n: usize,
) -> Result<(Set, usize, usize), String> {
    let m = sorted_edges.len() - 1;
    let mut edges_in_st = Set::new(m);

    let mut u = QuickUnionUf::<UnionBySize>::new(n + 1);

    for (x, y, _, i) in &sorted_edges[1..m + 1] {
        if u.find(*x) != u.find(*y) {
            u.union(*x, *y);
            edges_in_st.insert(*i);

            if edges_in_st.len() == n - 1 {
                let minimum_st_sum: usize =
                    edges_in_st.iter().map(|x| edges_with_order[*x].2).sum();
                let mirror_sum: usize = edges_in_st
                    .iter()
                    .map(|x| edges_with_order[m + 1 - *x].2)
                    .sum();
                return Ok((edges_in_st, minimum_st_sum, mirror_sum));
            }
        }
    }

    Err("No spanning tree".to_string())
}

fn find_locked_edges(edges: &Vec<(usize, usize, usize, usize)>, n: usize) -> Set {
    let mut locked_edges = Set::new(edges.len() - 1);
    let mut graph: DynamicGraph<EmptyWeight> = DynamicGraph::new(n + 1, 100);
    let vertices: Vec<VertexIndex> = (0..n + 1).map(|x| VertexIndex(x)).collect();
    let mut edges_in_graph = vec![];

    for (v, w, _, _) in &edges[1..edges.len()] {
        edges_in_graph.push(graph.insert_edge(vertices[*v], vertices[*w]).unwrap());
    }

    for i in 0..edges_in_graph.len() {
        graph.delete_edge(edges_in_graph[i]);
        if !graph.is_connected(vertices[edges[i + 1].0], vertices[edges[i + 1].1]) {
            locked_edges.insert(i + 1);
        }
        graph.insert_edge(vertices[edges[i + 1].0], vertices[edges[i + 1].1]);
    }

    return locked_edges;
}

fn brute_force(
    edges_with_order: &Vec<(usize, usize, usize, usize)>,
    spanning_tree: &mut DynamicGraph,
    spanning_tree_edge_set: &mut Set,
    filtered_edges: &Vec<(usize, usize, usize, usize)>,
    index: usize,
    n: usize,
) -> (usize, Set) {
    let m = edges_with_order.len() - 1;

    if spanning_tree_edge_set.len() == n - 1 {
        let (tree_weight, mirror_weight) =
            spanning_tree_edge_set.iter().fold((0, 0), |(tw, mw), i| {
                (
                    tw + edges_with_order[*i].2,
                    mw + edges_with_order[m + 1 - i].2,
                )
            });
        return (
            max(tree_weight, mirror_weight),
            spanning_tree_edge_set.clone(),
        );
    }

    if index > filtered_edges.len() - (n - 1) + spanning_tree_edge_set.len() {
        return (usize::MAX, Set::new(0));
    }

    let current_edge = edges_with_order[filtered_edges[index].3];

    let result_without_index = brute_force(
        edges_with_order,
        spanning_tree,
        spanning_tree_edge_set,
        filtered_edges,
        index + 1,
        n,
    );

    let (v, w) = (VertexIndex(current_edge.0), VertexIndex(current_edge.1));

    if !spanning_tree.is_connected(v, w) {
        let edge = spanning_tree.insert_edge(v, w).unwrap();
        spanning_tree_edge_set.insert(current_edge.3);
        let result_with_index = brute_force(
            edges_with_order,
            spanning_tree,
            spanning_tree_edge_set,
            filtered_edges,
            index + 1,
            n,
        );
        spanning_tree.delete_edge(edge);
        spanning_tree_edge_set.remove(&current_edge.3);
        if result_with_index.0 <= result_without_index.0 {
            return result_with_index;
        } else {
            return result_without_index;
        }
    }

    return result_without_index;
}

fn minimum_mirror_spanning_tree(
    edges_with_order: &Vec<(usize, usize, usize, usize)>,
    n: usize,
) -> Result<(usize, Set), String> {
    let m = edges_with_order.len() - 1;
    let mut edges_sorted_by_weight = edges_with_order.clone();
    let mut edges_sorted_by_mirror_weight = edges_with_order.clone();

    edges_sorted_by_weight.sort_by(|edge_i, edge_j| edge_i.2.cmp(&edge_j.2));

    edges_sorted_by_mirror_weight.remove(0); // remove index fixer in order to sort
    edges_sorted_by_mirror_weight.sort_by(|edge_i, edge_j| {
        edges_with_order[m + 1 - edge_i.3]
            .2
            .cmp(&edges_with_order[m + 1 - edge_j.3].2)
    });
    edges_sorted_by_mirror_weight.insert(0, (0, 0, 0, 0)); // insert index fixer agian after sorting

    // If no spanning tree exists, there is no solution. If the mst function returns an error, the ? operator here also returns that error.
    let (spanning_tree, tree_weight, mirror_weight) =
        minium_spanning_tree(edges_with_order, &edges_sorted_by_weight, n)?;

    if mirror_weight <= tree_weight {
        return Ok((tree_weight, spanning_tree));
    };

    let (spanning_tree, tree_weight, mirror_weight) =
        minium_spanning_tree(edges_with_order, &edges_sorted_by_mirror_weight, n)?;

    if tree_weight <= mirror_weight {
        return Ok((mirror_weight, spanning_tree));
    }

    let mut locked_edges = find_locked_edges(edges_with_order, n); // every bridge must be in every spanning tree
    let mut filtered_edges = vec![];
    // The DynamicGraph structure supports very fast (log²n) dynamic connectivity
    let mut graph = DynamicGraph::new(n + 1, 100);

    for (v, w, weight, i) in &edges_with_order[1..m+1] {
        if locked_edges.contains(i) {
            graph.insert_edge(VertexIndex(*v), VertexIndex(*w));
            continue;
        }
        filtered_edges.push((*v, *w, *weight, *i));
    }

    Ok(brute_force(
        edges_with_order,
        &mut graph,
        &mut locked_edges,
        &filtered_edges,
        0,
        n,
    ))
}

fn main() {
    let mut input_string = String::new();

    let n = read_int(&mut input_string);
    let m = read_int(&mut input_string);

    let mut edges_with_order = Vec::from([(0, 0, 0, 0)]);

    for i in 0..m {
        let line = read_line_of_ints(&mut input_string);
        let edge = (line[0], line[1], line[2], i + 1);
        edges_with_order.push(edge);
    }

    match minimum_mirror_spanning_tree(&edges_with_order, n) {
        Err(_) => println!("NO"),
        Ok((B, spanning_tree)) => {
            for i in spanning_tree {
                println!("{:?}", i);
            }
            println!("{:?}", B);
        }
    }
}
