// Usin a hash map and vectors, create a text interface to allow a user
// to add employee names to a department in a company. For example, 
// "Add Sally to Engineering" or "Add Amir to Sales". Then let the user
// retrieve a list of all people in a department or all people in the 
// company by department, sorted alphabetically.

use std::collections::HashMap;

fn main() {
    let mut _company: HashMap<String,Vec<String>> = HashMap::new();
    let mut v: Vec<String> = vec!["fran".to_string(), "pedro".to_string(), "pepe".to_string()];
    let nombre = String::from("juan"); 
    let mut v2 = add_employee(nombre, &mut v);

    println!("{:#?}", v);
}

fn add_employee(name: String, listado: &mut Vec<String>) -> &Vec<String> {
    listado.push(name);
    listado
}

fn get_department_employees(department_name: String, map: &HashMap<String,Vec<String>> ) -> &Vec<String> {
    let department_employees = map.get(&department_name).expect("El departamento no existe");
    department_employees
}
