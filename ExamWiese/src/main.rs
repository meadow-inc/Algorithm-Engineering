use std::{thread, time::{Instant},io::{BufReader, BufRead, BufWriter, Write}, vec, path::Path, fs::{File, self}, cmp::min, env::args};
use std::sync::{Arc, RwLock};
use good_lp::*;
use rand::*;
use num_cpus;

fn main() {

    

}

/// This function is used to test the approximation algorithm regarding the solution and solve time
/// It uses the g05_.. files of the Big Mac library.
/// It writes the results to a file
#[allow(dead_code)]
fn approximation_solution(){

    //create the result file
    let file = File::create("result_approximation.txt").unwrap();
    let mut writer = BufWriter::new(file);

    //loop through all files
    for i in (60..=100).step_by(20){
        for j in 0..=9{
            //create the file name
            let mut file_name = String::from("files/rudy/g05_");
            file_name.push_str(&(i.to_string()));
            file_name.push_str(".");
            file_name.push_str(&(j.to_string()));
            let path = Path::new(&file_name);
            //create the graph from the file
            let tuple = create_graph_from_file(&path);
            //start timer
            let timer = Instant::now();
            //do approximation and save the resulting cut
            let cut = approximation_parallel(&tuple.0, tuple.1, true);
            let duration = timer.elapsed().as_nanos();
            let line = &mut (i.to_string());
            //write the result to the file
            line.push_str(".");
            line.push_str(&(j.to_string()));
            line.push_str(" ");
            line.push_str(&(cut.to_string()));
            line.push_str(" ");
            line.push_str(&(duration.to_string()));
            line.push_str("\n");
            writer.write_all(line.as_bytes()).unwrap();
        }
    }
}

/// This function is used to test the solution and running time of the heuristic algorithm
/// It uses the g05_... graphs from the Big Mac library
/// It writes the results to a file
#[allow(dead_code)]
fn heuristic_solution(){
    //create result file
    let file = File::create("result_heuristic.txt").unwrap();
    let mut writer = BufWriter::new(file);

    //loop through all graphs from the library
    for i in (60..=100).step_by(20){
        for j in 0..=9{
            //create the file name
            let mut file_name = String::from("files/rudy/g05_");
            file_name.push_str(&(i.to_string()));
            file_name.push_str(".");
            file_name.push_str(&(j.to_string()));
            let path = Path::new(&file_name);
            //create a graph from the file
            let tuple = create_graph_from_file(&path);
            //start timer
            let timer = Instant::now();
            //run heuristic and save the result
            let partition = heuristic(&tuple.0, tuple.1);
            let duration = timer.elapsed().as_nanos();
            //calculate the resulting cut
            let cut = get_cut_size(&tuple.0, tuple.1, &partition.0);
            //write results to file
            let line = &mut (i.to_string());
            line.push_str(".");
            line.push_str(&(j.to_string()));
            line.push_str(" ");
            line.push_str(&(cut.to_string()));
            line.push_str(" ");
            line.push_str(&(duration.to_string()));
            line.push_str("\n");
            writer.write_all(line.as_bytes()).unwrap();
        }
    }

}

/// This function benchmarks the ILP solver on randomly generated graphs
#[allow(dead_code)]
fn benchmark_full_solve(min_nodes: i32, max_nodes: i32, step_nodes: i32, prob_start: i32, prob_end: i32,prob_step: i32, output_name: &str){
    let mut timer;
    //create result file
    let file = File::create(output_name).unwrap();
    let mut writer = BufWriter::new(file);

    //prepare path to a temporary file
    let graph_name = "tmp";

    //create graphs of various sizes (regarding the number of nodes)
    for i in (min_nodes..=max_nodes).rev().step_by(step_nodes as usize){
        let mut times: Vec<u128> = Vec::new();
        //for each number of nodes, create graphs of various densities
        for j in (prob_start..=prob_end).rev().step_by(prob_step as usize){
            //create a random graph and save it in "tmp"
            create_random_graph(i, j, String::from(graph_name));
            let path = Path::new("tmp");
            //create a graph from "tmp"
            let tuple = create_graph_from_file(&path);
            //start timer
            timer = Instant::now();
            //solve the Max Cut
            full_solve(&tuple.0, tuple.1);
            let duration = timer.elapsed().as_millis();
            //push the duration to the vector
            times.push(duration);
            //delete the file
            fs::remove_file("tmp").unwrap();
        }
        //calculate the average running time of the solver for the current number of nodes
        let sum: u128 = times.iter().sum();
        let avg = sum / times.len() as u128;
        let line = &mut (i.to_string());
        //write the result to the file
        line.push_str(" ");
        line.push_str(&avg.to_string());
        line.push_str("\n");
        writer.write_all(line.as_bytes()).unwrap();
        writer.flush().unwrap();
        println!("{}/{} finished", i, max_nodes);
    }
}

/// This function is used to benchmark the running time of the approximation algorithm
/// The results are written to a file
#[allow(dead_code)]
fn benchmark_approx(do_parallel: bool, min_nodes: i32, max_nodes: i32, step_nodes: i32, prob_start: i32, prob_end: i32, prob_step: i32, output_name: &str){
    //create a timer
    let mut timer;
    //create the result file
    let file = File::create(output_name).unwrap();
    let mut writer = BufWriter::new(file);

    let graph_name = "tmp";
    
    //create random graphs with various number of nodes
    for i in (min_nodes..=max_nodes).rev().step_by(step_nodes as usize){
        //collect the running times for each graph in a vec
        let mut times: Vec<u128> = Vec::new();
        //for each number of nodes, create graphs of various densities
        for j in (prob_start..=prob_end).step_by(prob_step as usize){
            //create the graph file
            create_random_graph(i, j, String::from(graph_name));
            let path = Path::new("tmp");
            //create a graph from the file
            let tuple = create_graph_from_file(&path);
            //start the timer
            timer = Instant::now();
            //run the approximation
            approximation_parallel(&tuple.0, tuple.1, do_parallel);
            let duration = timer.elapsed().as_millis();
            //push the result for the crrent graph instance to the vector
            times.push(duration);
            //remove the file
            fs::remove_file("tmp").unwrap();
        }
        //calculate the average running time
        let sum: u128 = times.iter().sum();
        let avg = sum / times.len() as u128;
        //write the result to a file
        let line = &mut (i.to_string());
        line.push_str(" ");
        line.push_str(&avg.to_string());
        line.push_str("\n");
        writer.write_all(line.as_bytes()).unwrap();
        writer.flush().unwrap();
        println!("{}/{} finished", i, max_nodes);


    }

}

///This function is used to benchmark the approximation algorithm with regard to different chunk sizes
/// The results are stored in a file
#[allow(dead_code)]
fn benchmark_approx_parallel_cs(cs_min: i32, cs_max: i32, cs_step: i32, num_nodes: i32, output_name: &str){
    //instantiate a timer
    let mut timer;
    //create the result file
    let file = File::create(output_name).unwrap();
    let mut writer = BufWriter::new(file);

    let graph_name = "tmp";
    let path = Path::new(&graph_name);
    //create a random graph file
    create_random_graph(num_nodes, 70, String::from(graph_name));
    //create a graph from the file
    let tuple = create_graph_from_file(path);

    //test the parallel approximation algorithm with different chunk sizes
    for i in (cs_min..=cs_max).step_by(cs_step as usize){
        //create a vector for storing the running times
        let mut times: Vec<u128> = Vec::new();
        //run the approximation algorithm 5 times to get more precise results
        for _ in 0..5{
            //start the timer
            timer = Instant::now();
            //run the approximation
            approximation_parallel(&tuple.0, tuple.1, true);
            let duration = timer.elapsed().as_millis();
            //push the duration to the vector
            times.push(duration);
        }
        //calculate the average running time
        let sum: u128 = times.iter().sum();
        let avg = sum / times.len() as u128;
        let line = &mut (i as f64).to_string();
        //write the result to the result file
        line.push_str(" ");
        line.push_str(&avg.to_string());
        line.push_str("\n");
        writer.write_all(line.as_bytes()).unwrap();
        
        println!("{}/{} finished", i, num_nodes);
        writer.flush().unwrap();
        
        
    }
}

// This function is used to benchmark the running time of the heuristic
/// The results are stored in a file
#[allow(dead_code)]
fn benchmark_heuristic(min: i32, max: i32, step: i32, prob_start: i32, prob_end: i32, prob_step: i32){
   
    //create a timer
    let mut timer;
    //create the result file
    let file = File::create("times_reversed_heuristic.txt").unwrap();
    let mut writer = BufWriter::new(file);

    let graph_name = "tmp";

    //create graph with various number of nodes
    for i in (min..=max).rev().step_by(step as usize){
        //instantiate a vector for storing the running times
        let mut times: Vec<u128> = Vec::new();
        //for each number of nodes, create a graph file of various densities
        for j in (prob_start..=prob_end).step_by(prob_step as usize){
            let path = Path::new(&graph_name);
            //create the random graph file
            create_random_graph(i, j, String::from(graph_name));
            //create the graph from the file
            let tuple = create_graph_from_file(path);
            //start the timer
            timer = Instant::now();
            //run the heuristic algorithm
            heuristic(&tuple.0, tuple.1);
            let duration = timer.elapsed().as_nanos();
            //push the duration to the vector
            times.push(duration);
            //remove the graph file
            fs::remove_file(graph_name).unwrap();
        }
        //calculate the average running time
        let sum: u128 = times.iter().sum();
        let count = times.len();
        let avg = sum / count as u128;
        //write the result to the result file
        let line = &mut (i as f64).to_string();
        line.push_str(" ");
        line.push_str(&avg.to_string());
        line.push_str("\n");
        writer.write_all(line.as_bytes()).unwrap();
        println!("{} finished", i);
        writer.flush().unwrap();
    }
}


//this function is used 
#[allow(dead_code)]
fn create_random_graph(num_nodes: i32, edge_probability: i32, graph_name: String){

    let file = File::create(&graph_name).unwrap();
    let mut writer = BufWriter::new(file);

    //let mut first_line = String::from("p td ");
    let mut first_line = String::from("");
    first_line.push_str(&num_nodes.to_string());
    first_line.push_str("\n");
    writer.write_all(first_line.as_bytes()).unwrap();

    let mut rng = rand::thread_rng();

    for i in 1..=num_nodes{
        for j in (i+1)..=num_nodes{
            if rng.gen_range(0..100) < edge_probability{
                let mut line = String::new();
                line.push_str(&i.to_string());
                line.push_str(" ");
                line.push_str(&j.to_string());
                line.push_str("\n");
                writer.write_all(line.as_bytes()).unwrap();
            }
        }
    }
}

/// This function uses an ILP solver to calculate the size of the maximum cut
#[allow(dead_code)]
fn full_solve(graph_bool: &Vec<bool>, num_nodes: i32) -> Vec<bool>{

    //transform the graph from a bool vector to a binary vector
    let mut graph = Vec::new();
    for gb in graph_bool{
        graph.push(match gb {
            true => 1,
            _ => 0,
        });
    }

    //create a vector for the problemvariables
   let mut problem = ProblemVariables::new();
   //for each possible edge (regardless of whether the edge is present or not) create a binary variable
   let edge_variables: Vec<Variable> = problem.add_vector(variable().binary(), graph.len());
   //for each vertex, create a binary variable
   let vertex_variables: Vec<Variable> = problem.add_vector(variable().binary(), num_nodes as usize);

    //create the objective by calling objectve_func()
   let objective: Expression = objective_func(&graph, &edge_variables);
   //create a CoinCbcProblem
   let mut ccp = problem.maximise(objective).using(default_solver);
   //initialize counter to keep track of the current position in the graph vector
    let mut pos = 0;  
    //loop through each entry in the graph vector. i and j represent the vertices
   for i in 0..num_nodes{
       for j in (i+1)..num_nodes{
        //if an edje between i and j exist:
           if graph_bool[pos]{
            //create the two neccessary constraints
               let constraint1 = constraint!(edge_variables[pos] <= vertex_variables[i as usize] + vertex_variables[j as usize]);
               let constraint2 = constraint!(edge_variables[pos] <= 2 - vertex_variables[i as usize] - vertex_variables[j as usize]);
               //add the constraint to the CoinCbcProblem
               ccp.add_constraint(constraint1);
               ccp.add_constraint(constraint2);
           }
           //increment the position
           pos += 1;
       }
   }
   //solve the problem
   let solution = ccp.solve().unwrap();
   //create a vector to store the results
   let mut result = Vec::new();
   //loop through the vertex variables, adding a bool value, indicating the set of the vertex
   for vv in vertex_variables{
       if solution.value(vv).eq(&0.0){
           result.push(false);
       }else{
           result.push(true);
       }
   }
   //return the result
   result
}

/// This function creates the objective for the solver
#[allow(dead_code)]
fn objective_func(graph: &Vec<i32>, edge_variables: &Vec<Variable>) -> Expression{
    
    //create the base result (neutral)
    let mut result: Expression = 0.0.into_expression();
    //loop through all edge variables
    for i in 0..edge_variables.len(){
        //add the edge variable (0 or 1) multiplied with the value in the graph vector
        //if the edge is not present, graph[i] will be 0, which always leads to 0 value being added
        result = result + edge_variables[i].into_expression() * graph[i];
    }
    result
}

/// This function is an approximation algorithm, which can be run concurrently
#[allow(dead_code)]
fn approximation_parallel(graph_reference: &Vec<bool>, num_nodes: i32, do_parallel: bool) -> i32{

    
    let graph = graph_reference.clone();
    drop(graph_reference);

    //if run concurrently, calculate the chunk size based on the number of cores available.
    //if not, set the chunk size to twice the number of nodes, simulating a non parallel approximation
    let chunk_size = match do_parallel{
        true => (num_nodes / (2 * num_cpus::get() as i32)) as usize,
        _ => (num_nodes * 2) as usize,
    };

    //create a vector where for each vertex a bool is stored, indicating the set of which the vertex belongs to
    let mut bool_partition = vec![false; num_nodes as usize];

    //create to random sets of vertices (by just adding the first half of vertices to one partition, the other half to a second partition)
    let partition1: Vec<i32> = (0..(num_nodes/2)).collect();
    let partition2: Vec<i32> = ((num_nodes/2)..(num_nodes)).collect();

    //update the bool_partition vector 
    for p in &partition1{
        bool_partition[*p as usize] = true;
    }

    //calculate the current cut
    let current_cut: i32 = get_cut_size(&graph, num_nodes, &bool_partition) as i32;
    
    //create atomic references, allowing each thread to read from the memory concurrently
    //since the partitions need to be changed during the algorithm, they need to be able to be locked for write
    //That is why we use RW-Locks for them
    let arcl_partition1 = Arc::new(RwLock::new(partition1));
    let arcl_partition2 = Arc::new(RwLock::new(partition2));
    let arcl_bool_partition = Arc::new(RwLock::new(bool_partition));
    let arcl_current_cut = Arc::new(RwLock::new(current_cut));
    let arc_graph = Arc::new(graph);
    let arc_num_nodes = Arc::new(num_nodes);
    let arc_chunk_size = Arc::new(chunk_size);
    let arc_chunk_size_main_clone = Arc::clone(&arc_chunk_size);
    
    //create a variable which keeps track of whether the solution improved 
    let mut improved = true;

    //do until no improvement is possible
    while improved{
        
        improved = false;
        //for each for loop, create variables which store the best swap of vertices
        let left_best_swap = (0,0);
        let right_best_swap = (0,0);
        
        //create references to allow each thread to read the left and right best swap
        let arcl_left_best_swap = Arc::new(RwLock::new(left_best_swap));
        let arcl_right_best_swap = Arc::new(RwLock::new(right_best_swap));
        let arcl_right_best_swap_clone = Arc::clone(&arcl_right_best_swap);
        let arcl_left_best_swap_clone  = Arc::clone(&arcl_left_best_swap);
        let mut handles = vec![];
        let arcl_partition1_main_clone = Arc::clone(&arcl_partition1);
        let p1lock = arcl_partition1_main_clone.read().unwrap();
        let arcl_partition2_main_clone = Arc::clone(&arcl_partition2);
        
        //divide the vertices to be swapped into different parts, in order to allow each thread to deal with some of them
        for i in 0..=(p1lock.len()/chunk_size){
            //create references for concurrent read and write for each thread. These will be moved into the thread as closures
            let arcl_partition1_clone = Arc::clone(&arcl_partition1);
            let arcl_bool_partition_clone = Arc::clone(&arcl_bool_partition);
            let arc_graph_clone = Arc::clone(&arc_graph);
            let arc_num_nodes_clone = Arc::clone(&arc_num_nodes);
            let arc_chunk_size_clone = Arc::clone(&arc_chunk_size_main_clone);
            let arcl_left_best_swap_clone = Arc::clone(&arcl_left_best_swap);

            //spawn a thread
            let handle = thread::spawn(move || {
                
                //create a variable for the current best swap of the thread
                let mut thread_best_swap = (0,0);
                let p1lock = arcl_partition1_clone.read().unwrap();
                let boollock = arcl_bool_partition_clone.read().unwrap();
                //loop through the vertices for which the thread is responsible
                for v in (i * *arc_chunk_size_clone)..min((i+1) * *arc_chunk_size_clone, p1lock.len()){
                    //acquire a read lock 
                    let swap_candidate = p1lock[v];
                    let mut change: i32 = 0;
                    let horizontal_start = swap_candidate * *arc_num_nodes_clone - ((swap_candidate * (swap_candidate + 1)) / 2);
                    let horizontal_end = horizontal_start+(*arc_num_nodes_clone - 1 - swap_candidate);
                    let mut counter = 1;

                    //for each vertex, check how the cut changes when the vertex is swapped from partition1 to partition2
                    //this is done by 2 for loops
                    //the first for loop checks all edges i,j with i < j
                    for j in horizontal_start..horizontal_end{
                        if arc_graph_clone[j as usize]{
                            if boollock[swap_candidate as usize] == boollock[(swap_candidate+counter) as usize]{
                                change += 1;
                            }else{
                                change -= 1;
                            }
                        }counter += 1;
                    }
                    
                    //the second for loop checks all edges i,j with i > j
                    let mut pos: i32 = 0;
                    for k in 0..swap_candidate{
                        pos = pos as i32  + swap_candidate as i32 - 1 - k as i32;
                        if arc_graph_clone[pos as usize]{
                            if boollock[swap_candidate as usize] == boollock[k as usize]{
                                change += 1;
                            }else{
                                change -= 1;
                            }
                        }
                        pos = pos + *arc_num_nodes_clone as i32 - swap_candidate as i32;
                    }
                    //if the swap is better than the current best swap of the thread, update the swap
                    if change > thread_best_swap.1{
                        thread_best_swap = (v, change);
                    }
                }
                //at the end, check the best swap from the main thread. 
                //if the thread found a better swap, update the main threads swap
                let mut lbs = arcl_left_best_swap_clone.write().unwrap();
                if lbs.1 < thread_best_swap.1{
                    *lbs = thread_best_swap;
                }
            });
            //push thread to handles
            handles.push(handle);
        }

        //ensure that the main thread waits for all threads to be finished
        for handle in handles{
            handle.join().unwrap();
        }

        let left_best_swap = arcl_left_best_swap_clone.write().unwrap();
        drop(p1lock);
        //if the best swap yields a positive change in cut, do the swap
        if left_best_swap.1 > 0{
            let mut cclock = arcl_current_cut.write().unwrap();
            //add the change in cut to the current cut
            *cclock += left_best_swap.1;
            //update the partitions (since one vertex is now in the other partition)
            let mut p1lock = arcl_partition1.write().unwrap();
            let mut p2lock = arcl_partition2.write().unwrap();
            let changed = p1lock[left_best_swap.0 as usize];
            p1lock.remove(left_best_swap.0 as usize);
            p2lock.push(changed);
            let mut boollock = arcl_bool_partition.write().unwrap();
            boollock[changed as usize] = !boollock[changed as usize];
            improved = true;
        }

        //repeat the step for swapping a vertex from partition2 to partition1
        
        let p2lock = arcl_partition2_main_clone.read().unwrap();
        handles = vec![];
        for i in 0..=(p2lock.len()/chunk_size){
            let arcl_partition1_clone = Arc::clone(&arcl_partition1);
            let arcl_partition2_clone = Arc::clone(&arcl_partition2);
            let arcl_bool_partition_clone = Arc::clone(&arcl_bool_partition);
            let arc_graph_clone = Arc::clone(&arc_graph);
            let arc_num_nodes_clone = Arc::clone(&arc_num_nodes);
            let arc_chunk_size_clone = Arc::clone(&arc_chunk_size_main_clone);
            let arcl_right_best_swap_clone = Arc::clone(&arcl_right_best_swap);

            let handle = thread::spawn(move || {
                let mut thread_best_swap = (0,0);
                let p1lock = arcl_partition1_clone.read().unwrap();
                let p2lock = arcl_partition2_clone.read().unwrap();
                let boollock = arcl_bool_partition_clone.read().unwrap();
                for v in (i * *arc_chunk_size_clone)..min((i+1) * *arc_chunk_size_clone, p2lock.len()){
                    let swap_candidate = p2lock[v];
                    let mut change: i32 = 0;
                    let horizontal_start = swap_candidate * *arc_num_nodes_clone - ((swap_candidate * (swap_candidate + 1)) / 2);
                    let horizontal_end: i32 = horizontal_start as i32 + (*arc_num_nodes_clone as i32 - 1 - swap_candidate as i32);
                    let mut counter = 1;
                    for j in (horizontal_start as i32)..horizontal_end{
                        if arc_graph_clone[j as usize]{
                            if boollock[swap_candidate as usize] == boollock[(swap_candidate+counter) as usize]{
                                change += 1;
                            }else{
                                change -= 1;
                            }
                        }counter += 1;
                    }
                    let mut pos = 0;
                    for k in 0..swap_candidate{
                        pos = pos + swap_candidate - 1 - k;
                        if arc_graph_clone[pos as usize]{
                            if boollock[swap_candidate as usize] == boollock[k as usize]{
                                change += 1;
                            }else{
                                change -= 1;
                            }
                        }
                        pos = pos + *arc_num_nodes_clone - swap_candidate;
                    }
                    if change > thread_best_swap.1{
                        thread_best_swap = (v, change);
                    }
                    let mut rbs = arcl_right_best_swap_clone.write().unwrap();
                    if rbs.1 < thread_best_swap.1{
                        *rbs = thread_best_swap;
                    }
                } 
                drop(p1lock);
                drop(p2lock);
            });
            handles.push(handle);
        }
        for handle in handles{
            handle.join().unwrap();
        }

        
        let right_best_swap = arcl_right_best_swap_clone.write().unwrap();
        drop(p2lock);
        if right_best_swap.1 > 0{
            let mut cclock = arcl_current_cut.write().unwrap();
            *cclock += right_best_swap.1;
            let mut p1lock = arcl_partition1.write().unwrap();
            let mut p2lock = arcl_partition2.write().unwrap();
            let changed = p2lock[right_best_swap.0 as usize];
            p2lock.remove(right_best_swap.0 as usize);
            p1lock.push(changed);
            let mut boollock = arcl_bool_partition.write().unwrap();
            boollock[changed as usize] = !boollock[changed as usize];
            improved = true;
        }
    }
    let cclock = arcl_current_cut.read().unwrap();
    let result = *cclock;
    result as i32
}


/// This function creates a graph from a graph file
#[allow(dead_code)]
fn create_graph_from_file(path: &Path) -> (Vec<bool>, i32){

    //open the file
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    //initialize a vector to store the graph
    let mut graph: Vec<bool>;

    //create line iterator
    let mut lines = reader.lines();
    let line = lines.next();

    let line = line.unwrap().unwrap();
    let splitted: Vec<&str> = line.split(" ").collect();
    //the first line of a graph file contains how many nodes the graph has
    let num_nodes = splitted[0].parse::<i32>().unwrap();

    //calculate how big the vector has to be
    let size = num_nodes.pow(2) - (num_nodes * (num_nodes + 1))/2;
    //create a vector with proper size depending on the numberof nodes
    graph = vec![false; size as usize];

    //if a line is present in the graph, set the appropriate value at the position to true
    for line in lines{
        let line = line.unwrap();
        let splitted: Vec<&str> = line.split(" ").collect();
        let first = splitted[0].parse::<i32>().unwrap();
        let second = splitted[1].parse::<i32>().unwrap();
        let location = get_edge(first - 1, second -1, num_nodes);
        graph[location] = true;
    }
    //return
    (graph, num_nodes)
}


///This function is used to calculate the index, where the edge value of vertex x and y is located
#[allow(dead_code)]
fn get_edge(x: i32, y: i32, num_nodes: i32) -> usize{
    if x > y{
        return get_edge(y, x, num_nodes) as usize
    }
    (x * (num_nodes - 1) - (x * (x + 1)) / 2 + y - 1) as usize
}

///This function is the heuristic algorithm
#[allow(dead_code)]
fn heuristic(graph: &Vec<bool>, num_nodes: i32) -> (Vec<bool>, i32){
    //create a partition in which all vertices are in the same set
   let mut partition: Vec<bool> = vec![false; num_nodes as usize];
   //order the vertices by degree (descending)
   let ordered = order_degree_desc(graph, num_nodes);
   let mut switch = true;
   //add the vertices to different partitions in change (true and false being the two partitions)
   for o in ordered{
       partition[o.1 as usize] = switch;
       switch = !switch;
   }
   //return
    (partition, num_nodes)
}

///This function is used to calculate the cut of a graph
#[allow(dead_code)]
fn get_cut_size(graph: &Vec<bool>,num_nodes: i32, partition: &Vec<bool>) -> i32{
    let mut result = 0; 
    let mut position = 0;
    //for every possible edge in the graph do:
    for i in 0..num_nodes{
        for j in (i+1)..num_nodes{
            //if the edge is present, and the vertices are in different partitions, add 1 to the cut
            if graph[position] && partition[i as usize] != partition[j as usize]{
               result += 1;
            }
            position += 1;
        }
    }
    result
}


///This function is used to order the vertices of a graph by degree
#[allow(dead_code)]
fn order_degree_desc(graph: &Vec<bool>, num_nodes: i32) -> Vec<(i32, i32)>{
    //create a vector for storing a tuple. The first value contains the vertex, the second value the degree
   let mut degrees: Vec<(i32, i32)> = vec![(0, 0); num_nodes as usize];
   let mut position = 0;

    //for each vertex, count how many neighbours the vertex has
    for i in 0..(num_nodes){
        degrees[i as usize].1 = i;
        for j in 0..(num_nodes-1-i){
            if graph[position]{
                degrees[i as usize].0 = degrees[i as usize].0 + 1;
                degrees[(j+i+1) as usize].0 = degrees[(j+i+1) as usize].0 + 1;
            }            
            position = position + 1;
        }
    }
    //sort them by 
    degrees.sort_by_key(|tuple| tuple.0);
    degrees.reverse();
    degrees 
}

























