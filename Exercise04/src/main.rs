
use std::{env, fs::{self, File}, time::Instant, io::{BufWriter, Write}};





mod new_nearest_neighbor;

fn main() {
    // The main function collects command line arguments into a vector of strings, `args`.
    let args: Vec<String> = env::args().collect();

    // It creates a new file named "prints.txt" and creates a new BufWriter to write to the file.
    let print_file = File::create("prints.txt").unwrap();
    let mut print_writer = BufWriter::new(print_file);

    // It checks if the first argument passed in is a directory.
    if fs::metadata(&args[1]).unwrap().is_dir(){

        // If it is, it prints "starting..."
        println!("starting...");
        
        // It then iterates through all entries in the directory passed in as the first argument.
        for entry in fs::read_dir(&args[1]).unwrap(){
            
            let entry = entry.unwrap();
            let entry_path = entry.path();

            // If the entry is a gitkeep file it continues to the next iteration
            if entry_path.to_str().unwrap().contains(".gitkeep"){
                continue;
            }
            // Writes the name of the file in the prints.txt file
            print_writer.write_fmt(format_args!("<{}>\n", entry.file_name().to_str().unwrap())).unwrap();

            // It prints the name of the current file being processed.
            println!("Processing file: {}", entry.file_name().to_str().unwrap());
            // creates a nearest neighbor tour from the current file
            let mut file_tour = new_nearest_neighbor::tour_from_file(&entry_path);
            // it measure the time it takes to create the nearest neighbor tour
            let nn_timer = Instant::now();
            new_nearest_neighbor::create_nearest_neighbor_tour(&mut file_tour);
            // write the time it takes to create the tour, the tour length and the method used to the prints.txt file
            print_writer.write_fmt(format_args!("Nearest neighbor tour time: {:.5} seconds\nNearest neighbor tour length: {}\n", nn_timer.elapsed().as_secs_f32(), new_nearest_neighbor::tour_length(&file_tour))).unwrap();
            // measure the time it takes to perform two opt heuristic on the nearest neighbor tour
            let nn_to_timer = Instant::now();
            let mut swaps = new_nearest_neighbor::two_opt_heuristic(&mut file_tour, -1);
            // write the time it takes to perform the two opt heuristic, the tour length, the number of swaps made and the method used to the prints.txt file
            print_writer.write_fmt(format_args!("Two Opt Heuristic (Nearest Neighbor Tour) -- Time: {:.5} -- Length: {} -- Swaps: {}\n", nn_to_timer.elapsed().as_secs_f32(), new_nearest_neighbor::tour_length(&file_tour), swaps)).unwrap();
            // Create a tour from the current file
            file_tour = new_nearest_neighbor::tour_from_file(&entry_path);
            // measure the time it takes to perform two opt heuristic on the file order tour
            let fo_to_timer = Instant::now();
            // perform the two opt heuristic
            swaps = new_nearest_neighbor::two_opt_heuristic(&mut file_tour, -1);
            // write the time it takes to perform the two opt heuristic, the tour length, the number of swaps made and the method used to the prints.txt file along with the name of the file being processed
            print_writer.write_fmt(format_args!("Two Opt Heuristic (File Order Tour) -- Time: {:.5} -- Length: {} -- Swaps: {}\n</{}>\n\n\n", fo_to_timer.elapsed().as_secs_f32(), new_nearest_neighbor::tour_length(&file_tour), swaps, entry.file_name().to_str().unwrap())).unwrap();
        }
    }else {
        // if the first argument passed in is not a directory, it prints "arg 1 is not a directory."
        println!("arg 1 is not a directory.");
    }
}



    //ONLY FOR TEMPORARY CODE FOR CREATING THE TOURS (Djibouti, Qatar, Luxembourg)

    /*let file_name = "qa194.tsp";
    let mut file_tour = new_nearest_neighbor::tour_from_file(Path::new(&file_name));

    let output_init = "init_".to_owned() + file_name;
    new_nearest_neighbor::write_tour(Path::new(&output_init), &file_tour);


    new_nearest_neighbor::create_nearest_neighbor_tour(&mut file_tour);
    let output_nn = "nn_".to_owned() + file_name;
    new_nearest_neighbor::write_tour(Path::new(&output_nn), &file_tour);

    file_tour = new_nearest_neighbor::tour_from_file(Path::new(&file_name));
    let k = new_nearest_neighbor::two_opt_heuristic(&mut file_tour, -1);
    let output_full_swap_init = "full_swap_nn".to_owned() + file_name;
    new_nearest_neighbor::write_tour(Path::new(&output_full_swap_init), &file_tour);


    file_tour = new_nearest_neighbor::tour_from_file(Path::new(&file_name));
    new_nearest_neighbor::two_opt_heuristic(&mut file_tour, k/2);
    let output_half_swaps_init = "half_swaps_".to_owned() + file_name;
    new_nearest_neighbor::write_tour(Path::new(&output_half_swaps_init), &file_tour);


    file_tour = new_nearest_neighbor::tour_from_file(Path::new(&file_name));
    new_nearest_neighbor::create_nearest_neighbor_tour(&mut file_tour);
    new_nearest_neighbor::two_opt_heuristic(&mut file_tour, -1);
    let output_full_swap_nn = "full_swap_init".to_owned() + file_name;
    new_nearest_neighbor::write_tour(Path::new(&output_full_swap_nn), &file_tour);*/


    

    

    

