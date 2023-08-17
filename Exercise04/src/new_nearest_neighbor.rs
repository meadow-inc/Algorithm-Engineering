use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

pub struct City {
    pub id: i32,
    pub longitude: f64,
    pub latitude: f64,
    pub visited: bool,
}

impl City {
    pub fn new(id: i32, longitude: f64, latitude: f64) -> City {
        City {
            id,
            longitude,
            latitude,
            visited: false,
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

// This function calculates the total length of a tour of a given list of cities
pub fn tour_length(cities: &Vec<City>) -> f64 {
    // Initialize a variable to keep track of the total length
    let mut length = 0.0;
    // Get the number of cities in the list
    let cities_length = cities.len();
    // Iterate through each city in the list
    for i in 0..cities_length {
        // Add the distance from the current city to the next city (wrapping around to the first city if on the last city)
        // And take the square root of the distance 
        length += cities[i]
            .distance_to(&cities[(i + 1) % cities_length])
            .sqrt();
    }
    // Return the total length
    length
}

/*
This function performs a 2-opt swap on a given path of cities.
It takes in a mutable reference to a vector of City structs, as well as the indices of the two points to be swapped.
The function then reverses the order of the elements in the path between the two given indices (inclusive).
This can be used for optimization in pathfinding algorithms such as the traveling salesman problem.
*/

pub fn do2_opt_swap(path: &mut Vec<City>, i: usize, j: usize) {
    path[i + 1..j + 1].reverse();
}

// The `two_opt_heuristic` function takes in a mutable reference to a vector of City objects, `start_tour`, and an integer, `swap_limit`.
pub fn two_opt_heuristic(start_tour: &mut Vec<City>, swap_limit: i32) -> i32 {
    let path_length = start_tour.len();
    let mut swaps_count = 0;

    // The function sets a boolean variable `improved` to true, indicating that there is potential for improvement
    let mut improved = true;

    // The function enters a while loop that continues as long as `improved` is true
    while improved {

        improved = false;
        // The function then iterates through all pairs of indices, `i` and `j`, in the `start_tour` vector.
        for i in 0..=(path_length-2) {
            for j in (i+2)..=(path_length-1) {
                // The function calculates the change in total distance caused by swapping the edges between these indices.
                let delta = -start_tour[i].distance_to(&start_tour[(i + 1) % path_length])
                    - start_tour[j].distance_to(&start_tour[(j + 1) % path_length])
                    + start_tour[i].distance_to(&start_tour[j])
                    + start_tour[(i + 1) % path_length]
                        .distance_to(&start_tour[(j + 1) % path_length]);
                // If this change in distance is negative, indicating an improvement, the function calls `do2_opt_swap` to swap the edges.
                if delta < -0.00000001 {
                    do2_opt_swap(start_tour, i, j);
                    swaps_count += 1;
                    improved = true;
                }
            }
        }
    }
    // The function returns the number of swaps made
    swaps_count
}

// The `tour_from_file` function takes in a reference to a Path object, `path`.
pub fn tour_from_file(path: &Path) -> Vec<City> {
    // It opens the file at the given path and creates a new BufReader to read the file.
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // It initializes an empty vector, `cities`, to store the City objects.
    let mut cities: Vec<City> = Vec::new();

    // The function iterates through each line in the file.
    for line in reader.lines() {
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
            _ => cities.push(city),
        }
    }
    // The function returns the vector of City objects
    cities
}

// The `create_nearest_neighbor_tour` function takes in a mutable reference to a vector of City objects, `cities`.
pub fn create_nearest_neighbor_tour(cities: &mut Vec<City>) {
    // It initializes two variables, `nearest_city_pos` and `min_distance`, to keep track of the nearest city and its distance.
    let mut nearest_city_pos = 0;
    let mut min_distance = f64::MAX;

    // The function iterates through all pairs of City objects in the `cities` vector.
    for i in 0..cities.len() - 2 {
        for j in i + 1..cities.len() {
            // It calculates the distance between the current pair of City objects
            let distance = cities[i].distance_to(&cities[j]);
            // If the distance is less than the current minimum distance, it updates the `min_distance` and `nearest_city_pos` variables.
            if distance < min_distance {
                min_distance = distance;
                nearest_city_pos = j;
            }
        }
        // it then swap the nearest city with the next position in the vector
        cities.swap(i + 1, nearest_city_pos);
        // reset min_distance
        min_distance = f64::MAX;
    }
}


pub fn write_tour(path: &Path, cities: &Vec<City>) {
    let file = File::create(&path).unwrap();
    let mut writer = BufWriter::new(file);

    for city in cities {
        writer.write_all(city.fmt().as_bytes()).unwrap();
    }
}
