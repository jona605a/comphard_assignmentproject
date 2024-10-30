use fastset::{set, Set};
use outils::prelude::DynamicConnectivity;
use outils::types::{Edge, Edges};
use std::vec::Vec;
use std::{cmp::max, io};
use union_find::{QuickFindUf, QuickUnionUf, UnionBySize, UnionFind};
use outils::graph::dynconn::hdt::DynamicGraph;
use outils::prelude::VertexIndex;

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
    edges: &Vec<(usize, usize, usize, usize)>,
    n : usize,
) -> Result<(Vec<usize>, usize, usize, usize, usize), String> {
    let m = edges.len()-1;
    let mut edges_in_st = Vec::new();

    let mut u = QuickUnionUf::<UnionBySize>::new(n+1);
    let mut max_tree_edge = 0;
    let mut max_mirror_edge = 0;

    for (x, y, _, i) in &edges[1..m+1] {
        if u.find(*x) != u.find(*y) {
            u.union(*x, *y);
            edges_in_st.push(*i);
    
            max_tree_edge = max(edges[*i].3,max_tree_edge);
            max_mirror_edge = max(edges[*i].3,max_mirror_edge);
            
            if edges_in_st.len() == n - 1 {
                let minimum_st_sum: usize = edges_in_st.iter().map(|x| edges[*x ].2).sum();
                let mirror_sum: usize = edges_in_st.iter().map(|x| edges[m +1 - *x].2).sum();
                return Ok((edges_in_st, minimum_st_sum, mirror_sum, max_tree_edge, max_mirror_edge));
            }
        }
    }

    Err("No spanning tree".to_string())
}

fn find_locked_edges(graph: &mut Vec<Vec<usize>>, degrees: &mut Vec<usize>, edges: &Vec<(usize, usize, usize, usize)>,) -> Set{
    let mut locked_edges = Set::new(edges.len()-1);
   

    loop{
        let edge_set_size_before = locked_edges.len();

        for i in 1..edges.len(){
            if degrees[edges[i].0] == 1 || degrees[edges[i].1] == 1{
                locked_edges.insert(i);
                degrees[edges[i].0] -= 1;
                degrees[edges[i].1] -= 1;
            }
        }

        if edge_set_size_before == locked_edges.len(){
            return locked_edges;
        }
    }
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
    graph: &mut Vec<Vec<usize>>,
    degrees: &mut Vec<usize>,
    edges: &Vec<(usize, usize, usize, usize)>,
) -> Result<(usize, Set), String> {
    let mut edges_sorted_by_weight = edges.clone();
    let mut edges_sorted_by_mirror_weight = edges.clone();

    edges_sorted_by_weight.sort_by(|edge_i, edge_j| edge_i.2.cmp(&edge_j.2));
    println!("Edges sorted: {:?}",edges_sorted_by_weight);
    // edges_sorted_by_mirror_weight
    //     .sort_by(|edge_i, edge_j| edges[edge_i.3 - 1].2.cmp(&edges[edge_j.3 - 1].2));

    let (spanning_tree, tree_weight, mirror_weight,max_tree_edge,max_mirror_edge) = match minium_spanning_tree(&edges_sorted_by_weight,graph.len()-1) {
        Err(s) => return Err(s),
        Ok((spanning_tree, spanning_tree_weigth, mirror_weigth,max_tree_edge, max_mirror_edge)) => {
            (spanning_tree, spanning_tree_weigth, mirror_weigth,max_tree_edge, max_mirror_edge)
        }
    };

    println!("{:?} {:?} {:?}",tree_weight,mirror_weight,spanning_tree);
    let mut blacklisted_edges = (0..edges.len()).map(|_| false).collect::<Vec<bool>>();
    let mut locked_edges = find_locked_edges(graph, degrees, edges);

    for index in &locked_edges{
        blacklisted_edges[*index] = true;
    }

    println!("Locked edges (edge idx): {:?}",locked_edges);

    Ok(brute_force(
        edges,
        &mut DynamicGraph::new(graph.len(),100),
        &mut locked_edges,
        &blacklisted_edges,
        1,
        graph.len()-1,
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
    
    let sample = vec![15, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 17, 18, 19, 20, 25, 28, 29, 30, 31, 32, 33, 34, 35, 38];

    let mut sample_sum = 0;

    for index in sample{
        sample_sum += edges_with_order[index].2;
    }

    println!("This is the sample sum: {:?}",sample_sum);

    let mut sorted_edges = edges_with_order.clone();
    sorted_edges.sort_by(|e1, e2| e1.2.cmp(&e2.2));

    println!("{:?}", minimum_mirror_spanning_tree(&mut graph, &mut degrees, &edges_with_order));

}
