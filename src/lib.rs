extern crate rand;

use rand::Rng;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum DishCategory {
    Green,
    Carb,
    Protein,
}

impl DishCategory {
    pub fn num_variations() -> usize {
        3
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Dish {
    name : String,
    ingredients : Vec<String>,
    dish_type : Vec<DishCategory>,
}

impl Dish {
    fn new() -> Dish {
        Dish{name : String::new(), ingredients : vec![], dish_type : vec![] }
    }

    fn add_category(&mut self, c : DishCategory) {
        self.dish_type.push(c);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SideDish {
    name : String,
    ingredients : Vec<String>,
    dish_type : Vec<DishCategory>,
}

impl SideDish {
    fn new() -> SideDish {
        SideDish{name : String::new(), ingredients : vec![], dish_type : vec![] }
    }

    fn add_category(&mut self, c : DishCategory) {
        self.dish_type.push(c);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Dinner {
    main : Dish,
    appetizer : SideDish,
}

struct CookBook {
    main_dishes : Vec<Dish>,
    side_dishes : Vec<SideDish>,
}

impl CookBook {
     pub fn new() -> CookBook {
        CookBook{main_dishes : Vec::new(), 
                 side_dishes : Vec::new()}
     }

    pub fn build_mock_cb() -> CookBook {
        let mut cb = CookBook::new();

        //Dishes
        let mut dish = Dish::new();
        dish.name = String::from("Carne con papas");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Protein);
        dish.add_category(DishCategory::Carb);
        cb.main_dishes.push(dish);

        let mut dish = Dish::new();
        dish.name = String::from("Espagheti Bolognese");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Protein);
        dish.add_category(DishCategory::Carb);
        cb.main_dishes.push(dish);

        let mut dish = Dish::new();
        dish.name = String::from("Pollo paprika");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Protein);
        cb.main_dishes.push(dish);

        let mut dish = Dish::new();
        dish.name = String::from("Ensalada de atun");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Protein);
        dish.add_category(DishCategory::Green);
        cb.main_dishes.push(dish);

        let mut dish = Dish::new();
        dish.name = String::from("Pollos con espinacas y queso");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Protein);
        cb.main_dishes.push(dish);

        let mut dish = Dish::new();
        dish.name = String::from("Espaghetti Aglio");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Protein);
        dish.add_category(DishCategory::Carb);
        cb.main_dishes.push(dish);

        let mut dish = Dish::new();
        dish.name = String::from("Carne asada");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Protein);
        cb.main_dishes.push(dish);

        let mut dish = Dish::new();
        dish.name = String::from("Salmon con mantequilla y cilantro");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Protein);
        cb.main_dishes.push(dish);

        let mut dish = SideDish::new();
        dish.name = String::from("Pescado al mojo de ajo");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Protein);
        cb.side_dishes.push(dish);

        //Sides
        let mut dish = SideDish::new();
        dish.name = String::from("Ensalada verde");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Green);
        cb.side_dishes.push(dish);

        let mut dish = SideDish::new();
        dish.name = String::from("Esparragos italianos");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Green);
        cb.side_dishes.push(dish);

        let mut dish = SideDish::new();
        dish.name = String::from("Arroz blanco");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Carb);
        cb.side_dishes.push(dish);

        let mut dish = SideDish::new();
        dish.name = String::from("Esparragos con tocino");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Green);
        dish.add_category(DishCategory::Protein);
        cb.side_dishes.push(dish);

        let mut dish = SideDish::new();
        dish.name = String::from("Sopa");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Carb);
        cb.side_dishes.push(dish);

        let mut dish = SideDish::new();
        dish.name = String::from("Pure de papa");
        dish.ingredients = vec![String::from("")];
        dish.add_category(DishCategory::Carb);
        cb.side_dishes.push(dish);
        cb
     }

}


fn get_main(cur_week : &HashSet<Dish>, cb : &CookBook) -> Option<Dish> {
    let mut rng = rand::thread_rng();
    let mut dish_found = false;
    let mut dish : Dish = Dish::new();
    let mut ret = None;
    let mut i = 0;

    while !dish_found {
        let dish_selection = rng.gen::<usize>() % (cb.main_dishes.len());
        dish = cb.main_dishes[dish_selection].clone();

        //Check we are not already cooking that this week
        if !cur_week.contains(&dish) {
            dish_found = true;
            ret = Some(dish);
        }
        
        i = i + 1;
        if i >= cb.main_dishes.len() {
            dish_found = true;
        }
    }

    ret
}

fn get_side(dish : &Dish, cb : &CookBook) -> Option<SideDish> {
    let mut rng = rand::thread_rng();
    let selection_size = cb.side_dishes.len();
    let mut current_elements = HashSet::new();
    let mut ret = None;
    //Initial random index
    let mut seed : usize = rng.gen::<usize>() % (selection_size);
    
    //Add all current dish types. We'll look for a complimentary dish based on
    //all dish categories.
    //let res = dish.dish_type.clone().into_iter().map(|mut x| current_elements.insert(x));
    for category in dish.dish_type.clone().into_iter() {
        current_elements.insert(category);
    }

    for i in 0..selection_size {
        let side_selection = (seed + i) % selection_size;
        let side = cb.side_dishes[side_selection].clone();
        //let _ = side.dish_type.clone().into_iter().map(|x| current_elements.insert(x));
        for category in side.dish_type.clone().into_iter() {
            current_elements.insert(category);
        }

        if  current_elements.contains(&DishCategory::Carb) && 
            current_elements.contains(&DishCategory::Green) &&
            current_elements.contains(&DishCategory::Protein) {
            //Found a suitable side dish
            ret = Some(side);
            break;
        }
    }
    ret
}

fn plan_week(cook_book : &CookBook) -> Result<HashSet<Dinner>, ()> {
    let mut week_dishes : HashSet<Dish>  = HashSet::new();
    let mut week_sides : HashSet<SideDish> = HashSet::new();
    let mut week : HashSet<Dinner> = HashSet::new();

    while week.len() < 7 {
        let dish = get_main(&week_dishes, &cook_book).unwrap();
        week_dishes.insert(dish.clone());
        let side = get_side(&dish, &cook_book).unwrap();
        week_sides.insert(side.clone());
        println!("Dish: {:?} \n Side: {:?} \n\n", &dish.name, &side.name);
        week.insert(Dinner{ main : dish, appetizer : side});
    }

    Ok(week)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_week() {
        let cb = CookBook::build_mock_cb();
        let week = plan_week(&cb).unwrap();

        print_week(&week);
        assert!(week.len() == 7);
    }

    #[test]
    fn test_get_main() {
        let week = HashSet::new();
        let cb = CookBook::build_mock_cb();
        
        let dish = get_main(&week, &cb);
        assert!(dish.is_some());
    }

    #[test]
    fn test_get_side() {
        let week = HashSet::new();
        let cb = CookBook::build_mock_cb();
        
        let dish = get_main(&week, &cb).unwrap();
        let side = get_side(&dish, &cb);
        assert!(side.is_some());
    }

    fn print_week(week : &HashSet<Dinner>) {
        for d in week {
            println!("Dish: {:?} \n Side: {:?} \n\n", d.main.name, d.appetizer.name);
        }
    }
}
