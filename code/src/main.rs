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
    edges_with_order: &Vec<(usize,usize,usize,usize)>,
    sorted_edges: &Vec<(usize, usize, usize, usize)>,
    n : usize,
) -> Result<(Vec<usize>, usize, usize, usize, usize), String> {
    let m = sorted_edges.len()-1;
    let mut edges_in_st = Vec::new();

    let mut u = QuickUnionUf::<UnionBySize>::new(n+1);
    let mut max_tree_edge = 0;
    let mut max_mirror_edge = 0;

    for (x, y, _, i) in &sorted_edges[1..m+1] {
        if u.find(*x) != u.find(*y) {
            u.union(*x, *y);
            edges_in_st.push(*i);
    
            max_tree_edge = max(edges_with_order[*i].3,max_tree_edge);
            max_mirror_edge = max(edges_with_order[m+1-i].3,max_mirror_edge);
            
            if edges_in_st.len() == n - 1 {
                let minimum_st_sum: usize = edges_in_st.iter().map(|x| edges_with_order[*x ].2).sum();
                let mirror_sum: usize = edges_in_st.iter().map(|x| edges_with_order[m +1 - *x].2).sum();
                return Ok((edges_in_st, minimum_st_sum, mirror_sum, max_tree_edge, max_mirror_edge));
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
    println!("{:?}", edges_in_graph.len());

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
    edges: &Vec<(usize, usize, usize, usize)>,
    spanning_tree: &mut DynamicGraph,
    spanning_tree_edge_set : &mut Set,
    blacklisted_edges: &Vec<bool>,
    index: usize,
    n : usize
) -> (usize, Set) {
    let m = edges.len() - 1;

    if  spanning_tree_edge_set.len() == n-1{
        let (tree_weight, mirror_weight) = spanning_tree_edge_set.iter().fold((0,0), |(tw,mw),i| {(tw + edges[*i].2, mw + edges[m+1-i].2)});
        return (max(tree_weight,mirror_weight),spanning_tree_edge_set.clone());
    }

    if index > m+1-(n-1) + spanning_tree_edge_set.len(){
        return (usize::MAX,Set::new(0));
    }


    let result_without_index = brute_force(edges, spanning_tree, spanning_tree_edge_set, blacklisted_edges, index + 1,n);

    let (v,w) = (VertexIndex(edges[index].0),VertexIndex(edges[index].1));

    if !spanning_tree.is_connected(v,w) && !blacklisted_edges[index]{
        let edge = spanning_tree.insert_edge(v,w).unwrap();
        spanning_tree_edge_set.insert(index);
        let result_with_index = brute_force(edges, spanning_tree, spanning_tree_edge_set, blacklisted_edges, index+1, n);
        spanning_tree.delete_edge(edge);
        spanning_tree_edge_set.remove(&index);
        if result_with_index.0 <= result_without_index.0 {
            return result_with_index;
        } else {
            return result_without_index;
        }

    }

    return result_without_index;
}


fn minimum_mirror_spanning_tree(
    edges: &Vec<(usize, usize, usize, usize)>,
    n: usize,
) -> Result<(usize, Set), String> {
    let mut edges_sorted_by_weight = edges.clone();
    // let mut edges_sorted_by_mirror_weight = edges.clone();

    edges_sorted_by_weight.sort_by(|edge_i, edge_j| edge_i.2.cmp(&edge_j.2));
    println!("Edges sorted: {:?}",edges_sorted_by_weight);
    // edges_sorted_by_mirror_weight
    //     .sort_by(|edge_i, edge_j| edges[edge_i.3 - 1].2.cmp(&edges[edge_j.3 - 1].2));

    // If no spanning tree exists, there is no solution. If the mst function returns an error, the ? operator here also returns that error. 
    let (spanning_tree, tree_weight, mirror_weight,_max_tree_edge,_max_mirror_edge) = minium_spanning_tree(edges, &edges_sorted_by_weight,n)?; 

    println!("MST: {:?} {:?} {:?}",tree_weight,mirror_weight,spanning_tree);
    let mut blacklisted_edges = (0..edges.len()).map(|_| false).collect::<Vec<bool>>();
    let mut locked_edges = find_locked_edges(edges,n);
    // The DynamicGraph structure supports very fast (log²n) dynamic connectivity
    let mut graph = DynamicGraph::new(n+1,100);

    for index in &locked_edges{
        blacklisted_edges[*index] = true;
        graph.insert_edge(VertexIndex(edges[*index].0), VertexIndex(edges[*index].1));
    }

    println!("Locked edges (edge idx): {:?}",locked_edges);

    Ok(brute_force(
        edges,
        &mut graph,
        &mut locked_edges,
        &blacklisted_edges,
        1,
        n,
    ))
}

fn main() {
    let mut input_string = String::new();

    let n = read_int(&mut input_string);
    let m = read_int(&mut input_string);

    let mut edges_with_order = Vec::from([(0, 0, 0, 0)]);
    let mut graph: Vec<Vec<usize>> = (0..n + 1).map(|_| Vec::new()).collect();
    let mut degrees = vec![0;n+1];

    for i in 0..m {
        let line = read_line_of_ints(&mut input_string);
        let edge = (line[0], line[1], line[2], i + 1);
        edges_with_order.push(edge);
        graph[line[0]].push(line[1]);
        degrees[line[0]] += 1;
        degrees[line[1]] += 1;
    }
    
    //let sample = vec![15, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 17, 18, 19, 20, 25, 28, 29, 30, 31, 32, 33, 34, 35, 38];

    // let sample = vec![7, 5, 32, 20, 30, 33, 31, 25, 14, 8, 11, 38, 35, 19, 1, 3, 6, 17, 36, 37, 28, 18, 2, 15, 26, 29, 10, 23, 13];

    // let mut sample_sum = 0;

    // for index in sample{
    //     sample_sum += edges_with_order[index].2;
    // }

    // println!("This is the sample sum: {:?}",sample_sum);

    let mut sorted_edges = edges_with_order.clone();
    sorted_edges.sort_by(|e1, e2| e1.2.cmp(&e2.2));

    println!("{:?}", minimum_mirror_spanning_tree(&edges_with_order,n));

}
