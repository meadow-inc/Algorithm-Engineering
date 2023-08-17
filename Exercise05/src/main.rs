use std::{io::{BufReader, BufRead, BufWriter, Write}, fs::File, path::Path, time::Instant};
use good_lp::*;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = Path::new(&args[1]);
    let k = args[2].parse::<i32>();
    let lazy = match args[3].parse::<i32>(){
        Ok(1) => true,
        Ok(0) => false,
        _ => {println!("wrong arguments!!");
        std::process::abort()},
    };
    //create a Vec of cities from a file, specifying how many cities should be included
    let cities = tour_from_file(file, k.unwrap());

    //create a Vec of edges containing the weights of the edges
    let edges = get_edges_lengths(&cities);
    
    if lazy{
        //call the solve function 
        let result = solve_lazy(&edges, cities.len());
        //create the city tour from the result
        let tour = create_final_tour(&result.2, &cities);
        let path = Path::new("lu200-lazy.txt");
        //write the tour to the file
        write_tour(&path, &tour);
        println!("number of subtour constraints = {} -- number of rounds = {}", result.0, result.1);
    }else{
        //call the solve function
        let result = solve_full(&edges, cities.len());
        //create the city tour from the result
        let tour = create_final_tour(&result.2, &cities);
        let path = Path::new("file.txt");
        //write the tour to the file
        write_tour(&path, &tour);
        println!("setup time = {} -- solve time = {}", result.0,result.1);
    }
}

/**
 * This function creates a City Vec so that we have real cities, instead of only indexes
 */
fn create_final_tour(index_cities: &Vec<usize>, cities: &Vec<City>) -> Vec<City>{
    let mut result = Vec::new();
    for i in 0..index_cities.len(){
        result.push(cities[index_cities[i]]);
    }
    result
}



/**
 * This function returns all subsets of a given set
 */
fn powerset<T>(s: &Vec<T>) -> Vec<Vec<T>> where T: Clone {
    (0..2usize.pow(s.len() as u32)).map(|i| {
         s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
                             .map(|(_, element)| element.clone())
                             .collect()
     }).collect()
}   


/**
 * This is the solver for full ILP
 * it returns the setup and solving time, as well as the result city Vector 
 */
fn solve_full(edges: &Vec<f64>, l: usize) -> (f64,f64,Vec<usize>){
    let result_setup_time: f64;
    let result_solve_time: f64;
    let mut timer = Instant::now();//initiate timer for measuring setup time
    let mut problem = ProblemVariables::new();//create Vec for the variables
        //create binary variables for every edge
        let y: Vec<Variable> = problem.add_vector(variable().binary(), edges.len());
        //define the objective (minimising the length)
        let objective: Expression = target_func(edges, &y);
        let mut ccp = problem.minimise(objective).using(default_solver);//create the problem
        //add the basic constraints (each city only connected to exactly two other cities)
        for i in 0..l{
             ccp.add_constraint(constraint!(two_city(&y, l, i) == 2));
        }
        //create subsets of cities
        let subtours = powerset(&(0..l).collect());

        //each subset bigger than 2 and unequal to the number of total cities should be added as constraint
        for s in subtours{
            if s.len() < 3 || s.len() == l{
                continue;
            }
            let constraint = sub_tour_constraint(&s, &y, l);
            ccp.add_constraint(constraint!(constraint <= (s.len() - 1) as u32));
        }
        //save the setup time and start a new timer
        result_setup_time = timer.elapsed().as_secs_f64();
        timer = Instant::now();
        
        //let the solve begin
        let solution = ccp.solve().unwrap();

        result_solve_time = timer.elapsed().as_secs_f64();

        let mut tour = Vec::new();
        for i in 0..y.len(){
            tour.push(solution.value(y[i]) as u32);
        }

        let checked = check_for_subtours(&tour, l);

        (result_setup_time,result_solve_time, checked[0].clone())//return



}

/**This function uses the ILP solver to solve our problem */
fn solve_lazy(edges: &Vec<f64>, l: usize) -> (usize, i32, Vec<usize>){
    //create two Vecs for storing the constraints
    let mut base_constraints: Vec<Expression> = Vec::new();//base constraints are making sure that each city only has to adjacent edges
    let mut subtour_constraints: Vec<(Expression, usize)> = Vec::new();//subtour constraints avoid finding the same subtours multiple times
    //init bools
    let mut finished = false; 
    let mut first_run = true;

    let mut sc_counter = 0;
    let mut round_counter = 0;
    let mut tour: Vec<usize> = Vec::new();
    //while loop only ends when a whole tour (without cycles) has been found
    while !finished{
        round_counter += 1;
        finished = true;//set finished to true
        let mut problem = ProblemVariables::new();//create Vec for the variables
        //create binary variables for every edge
        let y: Vec<Variable> = problem.add_vector(variable().binary(), edges.len());
        //define the objective (minimising the length)
        let objective: Expression = target_func(edges, &y);
        let mut ccp = problem.minimise(objective).using(default_solver);//create the problem
        //if this is the first run, create the base constraints
        if first_run{
            for i in 0..l{
                let constraint = two_city(&y, l, i);
                base_constraints.push(constraint);
            }
            first_run = false;
        }
        //add all base constraints to our problem
        for i in  0..base_constraints.len(){
            //println!("{:#?}", constraint!(base_constraints[i].clone() == 2));
            ccp.add_constraint(constraint!(base_constraints[i].clone() == 2));
        }
        //add all subtour constraints to our problem
        for i in 0..subtour_constraints.len(){
            ccp.add_constraint(constraint!(subtour_constraints[i].0.clone() <= subtour_constraints[i].1.clone() as u32));
        } 
        sc_counter = subtour_constraints.len();
        let solution = ccp.solve().unwrap();//let the solve begin!
        //push the results into a subtour Vec
        let mut subtour = Vec::new();
        for i in 0..y.len(){
            subtour.push(solution.value(y[i]) as u32);
        }
        //check the result for cycles (returns all cycles)
        let checked = check_for_subtours(&subtour, l);
        //if we have more than one cycle, there are subtours
        if checked.len() > 1{
            finished = false;//set finished to false (because we are not yet finished due to the subtours)
            //for each subtour, add a constraint
            for c in checked{
                let constraint = sub_tour_constraint(&c, &y, l);
                subtour_constraints.push((constraint, c.len() - 1));
            }
        }else {
            tour = checked[0].clone();
        }
    }
    (sc_counter, round_counter, tour)
}

/**
 * This is a function which helps to create a subtour constraint, in case subtours have been found by the solver. 
 * In the solver it will then be used as something like v1 + v4 + v8 + v2 + v15 + v12 <= 5.
 * This means that only 5 of these 6 edges may be selected
 */


fn sub_tour_constraint(subtour: &Vec<usize>, vars: &Vec<Variable>, l: usize) -> Expression{
    let mut result = 0.into_expression();//neutral Expression
    let n = subtour.len();
    //for each city pair of the subtour, add the corresponding variable to the sum
    for i in 0..(n-1){
        for j in (i+1)..n{
            let edge_index = magic(subtour[i] as i32, subtour[j] as i32, l as i32);
            result = result + vars[edge_index];
        }
    }result// example result: v1 + v4 + v8 + v2 + v15 + v12 
}

/** 
 * This function checks the resulting tour from the solver.
 * If there are subtours, check_tour() will find all of them
 */
fn check_for_subtours(results: &Vec<u32>, l: usize) -> Vec<Vec<usize>> { 
    //init result
    let mut result: Vec<Vec<usize>> = Vec::new();
    //mirror our triangle matrix
    let mirrored = mirror(results, l);
    //init a Vec for storing visited cities
    let mut visited = Vec::new();
    //for every city do:
    for j in 0..l{
        //init subtour Vec
        let mut subtour = Vec::new();
        //if the city has already been visited, proceed with next city
        if visited.contains(&j){
            continue;
        }
        //push both cities which are connected with the current city in the subtour Vec
        for i in 0..l{
            if mirrored[j][i] == 1{
                subtour.push(i);
            }
        }
        //now push the city itself in the middle of the Vec
        subtour.insert(1, j);
        //get the first element of the Vec
        let first = subtour.first().unwrap().to_owned();
        //init counter
        let mut i: i32 = 0;
        //while loop continues until we come out at the first element (meaning we have a cycle)
        while subtour.last().unwrap().to_owned() != first{
            //go through the city which is last in the vec and look to which other city it is connected
            if mirrored[subtour.last().unwrap().to_owned()][i as usize] == 1 && i as usize != subtour[subtour.len()-2]{
                subtour.push(i as usize);//push that other city
                i = 0;//reset i
                continue;
            }
            //end if we completed the cycle
            if subtour.last().unwrap().to_owned() == first{
                break;
            }
            i += 1;
        }
        //pop the last element. 
        subtour.pop();
        //push all cities into "visited" which we have seen for the current subtour
        for k in 0..subtour.len(){
            visited.push(subtour[k]);
        }
        //push the subtour into our result
        result.push(subtour);
    }
    result//return
}

/**
 * This function mirrors a triangle matrix.
 * 
 * --  1  3  4        0  1  3  4
 * -- --  2  8    =>  1  0  2  8
 * -- --  -- 5        3  2  0  5
 * -- --  -- --       4  8  5  0
 * This function will get the a series of 0 and 1, which represent the variable values of the solver
 */
fn mirror(results: &Vec<u32>, l: usize) -> Vec<Vec<u32>>{
    //create iterator over the Vector of u32
    let mut iter = results.iter();
    //init 2D-Vec for our matrix
    let mut result: Vec<Vec<u32>> = Vec::new();
    //push new Vecs (which will be the rows of our matrix)
    for _ in 0..l{
        result.push(Vec::new());
    }
    
    for i in 0..l{
        //push a zero (this 0 represents the edge that each city has with itself, this should always be zero)
        result[i].push(0);
        //add the rows for our matrix
        for j in 0..(l-i-1){
            let subtour = iter.next().unwrap().to_owned();
            result[i].push(subtour);
            result[i+j+1].push(subtour);
        }
    }
    result//return
}

/**
 * Target function for creating the function which the solver should minimise
 * Example result: 5 v1 + 3 v3 + v2 + 10 v0
 */
fn target_func(edges: &Vec<f64>, vars: &Vec<Variable>) -> Expression{
    //init result with neutral expression
    let mut result: Expression = 0.0.into_expression();
    //for all variables, add the variable multiplied with the edge weight (the lenght of the edges)
    for i in 0..vars.len(){
        result = result + vars[i].into_expression() * edges[i]
    }
    result//return
}


/**
 * This function is used for creating the base constraint of a specific city for our solver
 * These constraints make sure, that each city is only connected to exactly to other cities
 * Example result: v1 + v4 + v3 + v2 + v0
 * The example result will then be used: v1 + v4 + v3 + v2 + v0 == 2
 * This means that of all edges which a city can have, only (and exactly) two edges have to be used
 */
fn two_city(vars: &Vec<Variable>, l: usize, city_num: usize) -> Expression{
    let mut result = 0.into_expression();
    //start with pairs of cities where the second city is larger than our current city
    for i in (city_num+1)..l{
        result = result + vars[magic(city_num as i32, i as i32, l as i32)];
    } 
    //continue (if necessary) with pairs of cities where the second city is smaller than our current city
    if city_num != 0{
        for i in 0..(city_num){
            result = result + vars[magic(i as i32, city_num as i32, l as i32)]
        }
    }
    result//return
}

/**
 * This function allows us to enter two cities, and get the exact point where the corresponding edge is located in 
 * our Vec of edges
 */
fn magic( x: i32, y: i32, l: i32) -> usize{
    if x > y{
        return magic(y,x,l)
    }
    (l*x+y-x*(x+1)/2-x-1) as usize
}

/**
 * This function creates a Vec of edge lengths, where the lengths of all edges are saved (without sqrt)
 */
fn get_edges_lengths(cities: &Vec<City>) -> Vec<f64>{
    let mut edge_lengths: Vec<f64> = Vec::new();
    for i in 0..cities.len(){
        for j in (i+1)..cities.len(){
            let dist = cities[i].distance_to(&cities[j]).sqrt();
            edge_lengths.push(dist);
        }
    }
    edge_lengths
}

//old code from previous assignment
#[derive(Clone, Copy)]
pub struct City {
    pub id: i32,
    pub longitude: f64,
    pub latitude: f64,
}

impl City {
    pub fn new(id: i32, longitude: f64, latitude: f64) -> City {
        City {
            id,
            longitude,
            latitude,
        }
    }

    pub fn distance_to(&self, other_city: &City) -> f64 {
        (self.longitude - other_city.longitude).powi(2)
            + (self.latitude - other_city.latitude).powi(2)
    }

    pub fn fmt(&self) -> String {
        format!("{} {} {}\n", self.id, self.longitude, self.latitude)
    }

    pub fn eq_coord(&self, other: &City) -> bool {
        if self.latitude == other.latitude && self.longitude == other.longitude {
            return true;
        }
        false
    }
}

pub fn tour_from_file(path: &Path, subtour: i32) -> Vec<City> {
    // It opens the file at the given path and creates a new BufReader to read the file.
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    let city_quant;
    if subtour == -1{
        city_quant = i32::MAX;
    }else{
        city_quant = subtour;
    }

    // It initializes an empty vector, `cities`, to store the City objects.
    let mut cities: Vec<City> = Vec::new();

    let mut counter = 0;

    // The function iterates through each line in the file.
    for line in reader.lines() {

        if counter >= city_quant {
            break;
        }

        let line = line.unwrap();
        // It splits the line by spaces and collects the resulting substrings into a vector.
        let splitted: Vec<&str> = line.split(" ").collect();
        // It checks if the first element in the vector is a valid integer, if not it continues to the next iteration
        match splitted[0].parse::<i32>() {
            Ok(_) => {}
            Err(_) => continue,
        }
        // It creates a new City object with the parsed values from the splitted vector
        let city = City::new(
            splitted[0].parse::<i32>().unwrap(),
            splitted[1].parse::<f64>().unwrap(),
            splitted[2].parse::<f64>().unwrap(),
        );

        // it checks if there is already a City object in `cities` with the same coordinates as the newly created `city`.
        let city_with_same_id = cities.iter().find(|x| x.eq_coord(&city));
        match city_with_same_id {
            // If there is, the new City is not added to the vector
            Some(_) => continue,
            _ => {
                cities.push(city);
                counter += 1;
            }
        }
    }
    // The function returns the vector of City objects
    cities
}

fn write_tour(path: &Path, cities: &Vec<City>) {
    let file = File::create(&path).unwrap();
    let mut writer = BufWriter::new(file);

    for city in cities {
        writer.write_all(city.fmt().as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod test{
    use good_lp::{ProblemVariables, variable, Expression, Variable};

    use crate::{magic, City, get_edges_lengths, target_func};

    


    #[test]
    fn test_magic() {
        assert_eq!(magic(6, 7, 8),27);
        assert_eq!(magic(5,8,11),42);
        assert_eq!(magic(8,5,11),42);
        assert_eq!(magic(10, 3, 11),33);
    }

    #[test]
    fn test_edges_length_array_objective(){
        let city0 = City {
            id: 0,
            latitude: 20.0,
            longitude: 0.0,
        };
        let city1 = City {
            id: 1,
            latitude: 0.0,
            longitude: 0.0,
        };
        let city2 = City {
            id: 2,
            latitude: 3.0,
            longitude: 0.0,
        };
        let city3 = City {
            id: 3,
            latitude: 25.0,
            longitude: 0.0,
        };
        let city4 = City {
            id: 4,
            latitude: 26.0,
            longitude: 0.0,
        };
        let city5 = City {
            id: 5,
            latitude: 29.0,
            longitude: 0.0,
        };
        let city6 = City {
            id: 6,
            latitude: 30.0,
            longitude: 0.0,
        };
        let city7 = City {
            id: 7,
            latitude: 50.0,
            longitude: 0.0,
        };
    
        let mut cities = Vec::new();
        cities.push(city0);
        cities.push(city7);
        cities.push(city5);
        cities.push(city2);
        cities.push(city4);
        cities.push(city3);
        cities.push(city6);
        cities.push(city1);
    
        let edges = get_edges_lengths(&cities);
        assert_eq!(edges,vec![30.0,9.0,17.0,6.0,5.0,10.0,20.0,21.0,47.0,24.0,25.0,20.0,50.0,26.0,3.0,4.0,1.0,29.0,23.0,22.0,27.0,3.0,1.0,4.0,26.0,5.0,25.0,30.0]);
        let mut problem = ProblemVariables::new();
        let y: Vec<Variable> = problem.add_vector(variable().binary(), edges.len());
        let objective: Expression = target_func(&edges, &y);
        assert_eq!(format!("{:#?}",objective),"10 v5 + 5 v4 + 21 v7 + 20 v6 + 9 v1 + 30 v0 + 6 v3 + 17 v2 + 26 v13 + 50 v12 + 4 v15 + 3 v14 + 24 v9 + 47 v8 + 20 v11 + 25 v10 + 3 v21 + 27 v20 + 4 v23 + v22 + 29 v17 + v16 + 22 v19 + 23 v18 + 5 v25 + 26 v24 + 30 v27 + 25 v26");
    }

    

}
