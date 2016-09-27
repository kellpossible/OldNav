extern crate oldnav_lib;
use oldnav_lib::navdata::database::Database;
use std::env::current_exe;


fn main() {
    
    let mut exe_dir = current_exe().unwrap();
    exe_dir.pop();

    let resources_dir = exe_dir.clone().join("resources");
    let navdata_dir = resources_dir.join("navdata");


    let database = Database::new(navdata_dir, resources_dir);
    
    println!("{:?}", database);
}
