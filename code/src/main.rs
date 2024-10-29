use std::io;
use std::vec::{Vec};
use union_find::{QuickFindUf, QuickUnionUf, UnionBySize, UnionFind};

fn read_int(input : &mut String) -> usize{
    *input = "".to_string();
    
    io::stdin().read_line(input);
    return input.trim().parse::<usize>().unwrap();
}


fn read_line_of_ints(input : &mut String) -> Vec<usize>{
    *input = "".to_string();
    
    io::stdin().read_line(input).unwrap();
    input.trim().split(" ").map(|number| {number.trim().parse::<usize>().unwrap()}).collect()
}



fn minium_spanning_tree(graph : &mut Vec<Vec<(usize,usize)>>, edges : &Vec<(usize, usize, usize, usize)>) -> Result<(Vec<usize>,usize,usize),String>{


    let n = graph.len() - 1;
    let m = edges.len();
    let mut edges_in_st = Vec::new();

    let mut u = QuickUnionUf::<UnionBySize>::new(graph.len());

    for (x,y,w,i) in edges{
        if u.find(*x) != u.find(*y){
            u.union(*x, *y);
            edges_in_st.push(*i);
            if edges_in_st.len() == n-1{
                let minimum_st_sum : usize = edges_in_st.iter().map(|x| {edges[*x-1].2}).sum(); 
                let mirror_sum : usize = edges_in_st.iter().map(|x| {edges[m-*x].2}).sum(); 
                return Ok((edges_in_st,minimum_st_sum,mirror_sum))
            }
        }
    }


    Err("No spanning tree".to_string())

}



fn remove_edges(edges : &Vec<(usize,usize,usize,usize)>, edge_max : usize, mirror_max : usize) -> Vec<(usize,usize,usize,usize)>{

    let mut new_edges = Vec::new();

    for i in 0..edges.len(){
        let (x,y,w,index) = edges[i];
        if w <= edge_max && edges[edges.len()-index].2 <= mirror_max{
            new_edges.push((x,y,w,index));
        }
    }
    


    return new_edges
}



fn minimum_mirror_spanning_tree(graph : &mut Vec<Vec<(usize,usize)>>, edges : &Vec<(usize, usize, usize, usize)>) -> (){

    let mut edges_sorted_by_weight = edges.clone();
    let mut edges_sorted_by_mirror_weight = edges.clone();
    let mut edges_sorted_by_sum_of_weights = edges.clone();
}


fn main() {
    let mut input_string = String::new();


    let n = read_int(&mut input_string);
    let m = read_int(&mut input_string);

    let mut edges_with_order = Vec::new();
    let mut graph : Vec<Vec<(usize,usize)>> = (0..n + 1).map(|_| {Vec::new()}).collect();

    for i in 0..m{
        let line = read_line_of_ints(&mut input_string);
        let edge = (line[0], line[1], line[2], i + 1);
        edges_with_order.push(edge);
        graph[line[0]].push((line[1],line[2]));
    }


    let mut sorted_edges = edges_with_order.clone();
    sorted_edges.sort_by(|e1,e2| {e1.2.cmp(&e2.2)});

    println!("{:?}",edges_with_order);
    println!("{:?}",minium_spanning_tree(&mut graph, &sorted_edges).unwrap());
    
}


