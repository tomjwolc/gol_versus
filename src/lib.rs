use std::thread;

use rand::{Rng, prelude::*};

mod grid;
use grid::*;

mod cell;
use cell::*;

#[allow(dead_code)]
const HELLO_WORLD: &str = "\
|                                                                                    |\
|                                                                                    |\
|                                                                                    |\
|   #   #   #####   #       #       ###      #   #    ###    ###     #       ####    |\
|   #   #   #       #       #      #   #     #   #   #   #   #  #    #       #   #   |\
|   #####   ###     #       #      #   #     # # #   #   #   ###     #       #   #   |\
|   #   #   #       #       #      #   #     ## ##   #   #   #  #    #       #   #   |\
|   #   #   #####   #####   #####   ###      #   #    ###    #   #   #####   ####    |\
|                                                                                    |\
|                                                                                    |\
|                                                                                    |\
";

#[allow(dead_code)]
const FOOBAR: &str = "\
|                                                  |\
|                                                  |\
|                                                  |\
|   #####    ###     ###    ####     #     ###     |\
|   #       #   #   #   #   #   #   # #    #  #    |\
|   #       #   #   #   #   ####   #####   ###     |\
|   ###     #   #   #   #   #   #  #   #   #  #    |\
|   #        ###     ###    ####   #   #   #   #   |\
|   #                                              |\
|                                                  |\
|                                                  |\
";

#[allow(dead_code)]
const BIG_HELLO_WORLD: &str = "\
|                                                                                                                                                                 |\
|                                                                                                                                                                 |\
|                                                                                                                                                                 |\
|                                                                                                                                                                 |\
|                                                                                                                                                                 |\
|                                                                                                                                                                 |\
|           #     #    ########    #             #             #####          #         #      ####      #####       #            #######                         |\
|           #     #    #           #             #            #     #         #         #     #    #     #    ##     #            #  ######                       |\
|           #     #    #           #             #           #       #        #         #    #      #    #      #    #            #       ##                      |\
|           #     #    #           #             #           #       #        #         #    #      #    #    ##     #            #       ##                      |\
|           #######    #####       #             #           #       #        #   ###   #    #      #    #####       #            #       ##                      |\
|           #     #    #           #             #           #       #        ##  # #  ##    #      #    #   #       #            #       ##                      |\
|           #     #    #           #########     #########    #     #         ##  # #  ##     #    #     #    #      #########    #  ######                       |\
|           #     #    ########    #########     #########     #####           ###   ###       ####      #     #     #########    #######                         |\
|                                                                                                                                                                 |\
|                                                                                                                                                                 |\
|                                                                                                                                                                 |\
|                                                                                                                                                                 |\
|                                                                                                                                                                 |\
";

pub fn get_rng() -> ThreadRng {
    // match SEED {
    //     Some(seed) => ChaCha8Rng::seed_from_u64(seed),
    //     None => ChaCha8Rng::from_entropy()
    // }
    rand::thread_rng()
}

pub fn gol_step(grid: &mut Grid<Cell>) {
    for (i, j) in grid.reorder_indeces().into_iter() {
        let cell = grid.get_mut(i as isize, j as isize).unwrap();
        let prev = cell.state;

        *cell = match *cell {
            Cell { state: true, num_on: 2, num_on_next } => Cell { state: true , num_on: num_on_next, num_on_next: num_on_next },
            Cell { state: _   , num_on: 3, num_on_next } => Cell { state: true , num_on: num_on_next, num_on_next: num_on_next },
            Cell { state: _   , num_on: _, num_on_next } => Cell { state: false, num_on: num_on_next, num_on_next: num_on_next }
        };

        if prev != cell.state { update_change(grid, (i, j), !prev) }
    }
}

pub fn update_change(grid: &mut Grid<Cell>, pos: (usize, usize), do_inc: bool) {
    let (i, j) = (pos.0 as isize, pos.1 as isize);

    if let Some(cell) = grid.get_mut(i - 1, j - 0) { cell.adjust_all(do_inc)  }
    if let Some(cell) = grid.get_mut(i - 0, j - 1) { cell.adjust_all(do_inc)  }
    if let Some(cell) = grid.get_mut(i + 1, j + 0) { cell.adjust_next(do_inc) }
    if let Some(cell) = grid.get_mut(i + 0, j + 1) { cell.adjust_next(do_inc) }

    if let Some(cell) = grid.get_mut(i - 1, j - 1) { cell.adjust_all(do_inc)  }
    if let Some(cell) = grid.get_mut(i - 1, j + 1) { cell.adjust_all(do_inc)  }
    if let Some(cell) = grid.get_mut(i + 1, j - 1) { cell.adjust_next(do_inc) }
    if let Some(cell) = grid.get_mut(i + 1, j + 1) { cell.adjust_next(do_inc) }
}

pub fn rand_grid(width: usize, height: usize, mut density: Option<f64>) -> Grid<Cell> {
    let mut grid = Grid::new(width, height, |_, _| Cell::new());
    let mut rng = get_rng();

    if density == None { density = Some(rng.gen::<f64>()) }

    for (i, j) in grid.reorder_indeces().into_iter() {
        let cell = grid.get_mut(i as isize, j as isize).unwrap();
        cell.num_on = cell.num_on_next;

        if rng.gen::<f64>() < density.unwrap() {
            cell.state = true;

            update_change(&mut grid, (i, j), true);
        }
    }

    grid
}

pub fn initialize_grid(grid: &mut Grid<Cell>) {
    for (i, j) in grid.reorder_indeces().into_iter() {
        let cell = grid.get_mut(i as isize, j as isize).unwrap();
        cell.num_on = cell.num_on_next;

        if cell.state {
            update_change(grid, (i, j), true);
        }
    }
}

#[test]
pub fn gol_test() {
    // let mut grid = rand_grid(60, 18, Some(0.5));
    let mut grid: Grid<Cell> = Grid::from(HELLO_WORLD);

    initialize_grid(&mut grid);

    println!("{}", grid);
    std::thread::sleep(std::time::Duration::from_millis(2000));

    for i in 0..200 {
        std::thread::sleep(std::time::Duration::from_millis(50));
        std::process::Command::new("clear").status().unwrap();
        gol_step(&mut grid);

        let mut on_tiles = 0;

        for (_, _, cell) in grid.reorder().into_iter() {
            if cell.state { on_tiles += 1 }
        }

        println!("Step: {}, Total Alive Tiles: {}\n{}", i + 1, on_tiles, grid);
    }
}

#[test]
pub fn gol_step_on_text_because_im_stupid() {
    let mut grid: Grid<Cell> = Grid::from("                                                                                    |                                                                                    |               █                                                                    |           █  █                     ██               ███     ██              ██     |           ██ █                   █ █ █             █ █ █     █             ██ ██   |    █  █            ██     █      ██ ██      █ █    ██  █  █ ██             █   █   |   ██ ██   █  █    ███     ███    █ ██     ██   ██   ███   █        █ ███   █   █   |   █ ██    ██ █     █       █       █        █ █      ██            ██       ███    |              █                                                              █      |                                                                                    |                                                                                    \
    ");

    initialize_grid(&mut grid);
    println!("{}\n to \n", grid);

    gol_step(&mut grid);

    println!("{}", grid);
}

#[test]
pub fn solutions_test() {
    let grid: Grid<Cell> = Grid::from(BIG_HELLO_WORLD);
    let steps = 2;

    println!("{}", grid);
    let best = gen_alg_run(
                 1000, 
                &grid,
               400, 
                        steps, 
        0.25, 
        |i, best_of_gen, copy, mut_chance, local_best_score| {
            std::process::Command::new("clear").status().unwrap();
            println!("Generation #: {}, Mutation chance: {:.2}, Recent best score: {}, Best of generation: \n{}\n to {}", i + 1, mut_chance, local_best_score, best_of_gen.grid, copy);
        }
    );

    let mut copy = best.clone();
    for _ in 0..steps { gol_step(&mut copy.grid) };

    println!("Overall Best: \n{} to \n{}", best, copy);
}

pub struct Solution {
    grid: Grid<Cell>,
    score: f64,
    preserved: bool
}

impl Clone for Solution {
    fn clone(&self) -> Self {
        Solution { grid: self.grid.clone(), score: self.score, preserved: self.preserved }
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Score: {}\n{}", if self.preserved { "*" } else { "" }, self.score, self.grid)
    }
}

impl std::fmt::Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Score: {}\n{}", if self.preserved { "*" } else { "" }, self.score, self.grid)
    }
}

pub fn generate_solutions(problem: &Grid<Cell>, steps: usize, density_range: (f64, f64), amount: usize) -> Vec<Solution> {
    let mut solutions = Vec::new();
    let mut rng = get_rng();

    for _ in 0..amount {
        let grid = rand_grid(problem.width(), problem.height(), Some(rng.gen::<f64>() * (density_range.1 - density_range.0) + density_range.0));
        let score = score_solution(problem, &grid, steps);

        solutions.push(Solution { grid, score, preserved: false })
    }

    solutions
}

pub fn score_solution(problem: &Grid<Cell>, solution: &Grid<Cell>, steps: usize) -> f64 {
    let mut solution_clone = solution.clone();

    for _ in 0..steps {
        gol_step(&mut solution_clone);
    }

    let problem_cells = problem.reorder();
    let solution_cells = solution.reorder();
    let stepped_solution_cells = solution_clone.reorder();
    let mut score = 0.0;

    for i in 0..problem_cells.len() { // The best score a cell can get is 1.01, and the worst is -1.00
        if solution_cells[i].2.state == false { score += 0.1 / (problem.width() * problem.height()) as f64 };

        score += match (problem_cells[i].2.state, stepped_solution_cells[i].2.state) {
            (true, true) => 1.0,
            (true, false) => -1.0,
            (false, true) => 0.0,
            (false, false) => 1.0
        }
    }

    100.0 * score / ((problem.width() * problem.height()) as f64 * 1.01)
}

pub fn mix_solutions(problem: &Grid<Cell>, sol1: &Solution, sol2: &Solution, steps: usize, mutation_chance: f64) -> Solution {
    let mut grid_result = Grid::new(problem.width(), problem.height(), |i: usize, j: usize| {
        let mut rng = get_rng();

        if rng.gen::<f64>() < mutation_chance / (problem.width() * problem.height()) as f64 { 
            return Cell::from(rng.gen::<f64>() > 0.5); // Mutation
        }
        
        match (sol1.grid.get(i as isize, j as isize).unwrap().state, sol2.grid.get(i as isize, j as isize).unwrap().state) {
            (a, b) if a == b => Cell::from(a),
            (a, b)           => Cell::from(if rng.gen::<f64>() > 0.8 { a } else { b })
        }
    });

    initialize_grid(&mut grid_result);

    let score = score_solution(problem, &grid_result, steps);

    Solution { grid: grid_result, score, preserved: false }
}

pub fn cull_solutions(solutions: &mut Vec<Solution>) {
    sort(solutions, |a, b| if a.score - b.score < 0.0 { 1 } else { -1 });

    let initial_length = solutions.len();
    let mut rng = get_rng();

    while solutions.len() > initial_length / 2 {
        let weight = rng.gen::<f64>().powf(0.06);

        solutions.remove((solutions.len() as f64 * weight) as usize);
    }
}

pub fn reproduce_solutions(problem: &Grid<Cell>, solutions: &mut Vec<Solution>, steps: usize, mutation_chance: f64) {
    let mut new_solutions = Vec::new();

    let initial_length = solutions.len();

    for _ in 0..(2 * solutions.len() / 100) {
        let mut copy = solutions[0].clone();
        copy.preserved = true;
        new_solutions.push(copy);  // Copy straight over the 5th percentile
    }

    if solutions.len() % 2 == 1 { new_solutions.push(solutions.remove(0)) };

    while new_solutions.len() < 2 * initial_length {
        shuffle(solutions);
        
        for i in 0..(solutions.len() / 2) {
            new_solutions.push(mix_solutions(problem, &solutions[2 * i], &solutions[2 * i + 1], steps, mutation_chance));

            if new_solutions.len() >= initial_length {break;}
        }
    }



    *solutions = new_solutions;
}

pub fn gen_alg_run(
    length: usize, 
    problem: &Grid<Cell>, 
    pop_size: usize, 
    steps: usize, 
    mut mutation_chance: f64, 
    step_func: impl Fn(usize, &Solution, &Solution, f64, f64)
) -> Solution {
    let mut solutions = generate_solutions(problem, steps, (0.1, 0.6), pop_size);
    let mut recent_best_score = 0.0;
    let mut best_score = 0.0;
    let mut best_solution = solutions[0].clone();
    let mut stagnate_dur = 0;
    let initial_mut_chance = mutation_chance;

    let worker_count = 8;

    for i in 0..length {
        let mut handles = Vec::new();

        for sols_slice in solutions.chunks((solutions.len() / worker_count).max(1)) {
            let prob = problem.clone();
            let mut sols = sols_slice.to_vec();

            handles.push(
                thread::spawn(move || {
                    cull_solutions(&mut sols);
                    reproduce_solutions(&prob, &mut sols, steps, mutation_chance);
                    let blank = Solution { grid: Grid::empty(), score: 0.0, preserved: false};
                    let best_of_handle = sols.iter().fold(&blank, |best, sol| if !sol.preserved && best.score < sol.score { sol } else { &best });

                    (best_of_handle.clone(), sols)
                })
            );
        }

        solutions = Vec::new();

        let mut best_of_gen = Solution { grid: Grid::empty(), score: 0.0, preserved: false };

        for handle in handles.into_iter() {
            let (best_of_handle, mut sols) = handle.join().unwrap();
            solutions.append(&mut sols);

            if best_of_gen.score < best_of_handle.score { best_of_gen = best_of_handle }
        }

        shuffle(&mut solutions);

        if best_of_gen.score > best_score { // ----------------- The score is increasing absolutely
            stagnate_dur = 0;
            mutation_chance = initial_mut_chance;

            best_score = best_of_gen.score;
            recent_best_score = best_of_gen.score;
            best_solution = best_of_gen.clone();
        } else if best_of_gen.score > recent_best_score { // --- The score is increasing in the local area
            stagnate_dur = 0;
            mutation_chance = initial_mut_chance;

            recent_best_score = best_of_gen.score;
        } else if best_of_gen.score > recent_best_score - 5.0 { // --- The current state is stagnating below the best score
            stagnate_dur += 1;

            if stagnate_dur > 5 { 
                mutation_chance = (stagnate_dur / 5) as f64;
            }
        } else { // --------------------------------------------- The current state has dropped far lower than the best score
            recent_best_score = 0.0;
            stagnate_dur = 0;
            mutation_chance = initial_mut_chance;
        }

        let mut copy = best_of_gen.clone();

        for _ in 0..steps { gol_step(&mut copy.grid); }

        if i % 5 == 0 {
            step_func(i + 1, &best_of_gen, &copy, mutation_chance, recent_best_score);
            println!("Best_score: {:.5}, \nRecent_best_score: {:.5}, \nBest_of_gen.score: {:.5}, \nStagnate_dur: {:.5}", best_score, recent_best_score, best_of_gen.score, stagnate_dur);
        }

        if best_of_gen.score > 99.999 {break};
    };

    best_solution
}

fn shuffle<T>(arr: &mut Vec<T>) {
    let mut shuffled = Vec::new();
    let mut rng = get_rng();

    while arr.len() > 0 {
        let index = (arr.len() as f64 * rng.gen::<f64>()) as usize;

        shuffled.push(arr.remove(index));
    }

    *arr = shuffled;
}

fn sort<T>(arr: &mut Vec<T>, func: fn(&T, &T) -> isize) {
    if arr.len() < 2 {return;}

    let mut indeces: Vec<usize> = vec![0; arr.len()].iter().enumerate().map(|en| en.0).collect();

    indeces = sort_with_indeces(&arr.iter().map(|x| x).collect::<Vec<&T>>()[..], &indeces[..], func);

    let mut result: Vec<T> = Vec::new();

    for i in 0..indeces.len() {
        let index = indeces[i];

        result.push(arr.remove(index));

        indeces = indeces.iter().map(|&x| if x > index {x - 1} else {x}).collect();
    }

    *arr = result
}

fn sort_with_indeces<'a, T: 'a>(arr: &[&T], indeces: &[usize], func: fn(&T, &T) -> isize) -> Vec<usize> {
    let mid = indeces.len() / 2;

    let mut arr1: Vec<usize> = if mid == 1 { vec![indeces[0]] } else { sort_with_indeces(arr, &indeces[..mid], func) };
    let mut arr2: Vec<usize> = if indeces.len() == 2 { vec![indeces[1]] } else { sort_with_indeces(arr, &indeces[mid..], func) };
    let mut result: Vec<usize> = Vec::new();

    while arr1.len() + arr2.len() > 0 {
        if arr1.len() == 0 {
            result.push(arr2.remove(0));
            continue;
        } else if arr2.len() == 0 {
            result.push(arr1.remove(0));
            continue;
        }

        if func(arr[arr1[0]], arr[arr2[0]]) < 0 {
            result.push(arr1.remove(0));
        } else {
            result.push(arr2.remove(0));
        }
    };

    result
}