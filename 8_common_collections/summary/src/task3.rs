use std::io;
use std::collections::HashMap;
use sorted_vec::*;

pub fn text_interface() {
  let mut directory: HashMap<String, SortedVec<String>> = HashMap::new();

  loop {
    let option = get_option();

    match option {
      1 => add_employee(&mut directory),
      2 => list_in_department(&directory),
      3 => list_all_by_department(&directory),
      4 => {
        println!("Quiting...\nHave a nice day");
        break;
      },
      _ => {
        println!("Invalid option number. Try again.\n");
        continue;
      },
    }
    // option 1: add employee
    // option 2: list of all people in a department, alphabetically
    // option 3: list of all people by department, alphabetically
    // option 4: type quit

  }
}

fn list_all_by_department(directory: &HashMap<String, SortedVec<String>>) {
  println!("List of all people by department");

  let mut department_list = Vec::new();

  for department in directory.keys() {
    department_list.push(department);
  }

  department_list.sort();

  for department in department_list {
    println!("Department: {department}");

    let list: &SortedVec<String> = match directory.get(department) {
      Some(vec) => vec,
      None => {
        println!("Error retrieving department");
        return;
      },
    };

    for name in &mut *list.clone().into_vec() {
      println!("{:?}", name);
    }
  }
}

fn list_in_department(directory: &HashMap<String, SortedVec<String>>) {
  loop {
    println!("Type department name or \"back\"\nAvailable departments:");
    for dep in directory.keys() {
      println!("{dep}");
    }
    
    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let input = input.trim();

    if input == String::from("back") {
      return;
    }

    let list: &SortedVec<String> = match directory.get(input) {
      Some(vec) => vec,
      None => {
        println!("Invalid department. Try again.");
        continue;
      },
    };

    for name in &mut *list.clone().into_vec() {
      println!("{:?}", name);
    }
  }
}

fn add_employee(directory: &mut HashMap<String, SortedVec<String>>) {
  loop {
    println!("Follow the format: \"Add {{first name}} to {{department}}\"");
    println!("Or type \"back\"");

    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let mut words = Vec::new();
    
    for word in input.to_lowercase().split_whitespace() {
      words.push(String::from(word));
    }

    if words[0] == String::from("back") {
      println!("Returning to menu\n");
      break;
    } else if (words.len() < 4 || words[0] != String::from("add")) || (words[2] != String::from("to")) {
      println!("Invalid format");
      continue;
    } else {
      let (name, department) = get_info(words);
      directory.entry(department)
        .and_modify(|e| { e.push(name.clone()); })
        .or_insert_with(|| SortedVec::from_unsorted(vec![name]));
    }
  }
}

pub fn get_info(mut vec: Vec<String>) -> (String, String) {
  vec.remove(0);
  let name = vec.remove(0);
  vec.remove(0);
  let mut department = String::from(vec.remove(0));
  for word in vec {
    department = format!("{} {}", department, word);
  }
  return (name, department);
}

fn get_option() -> u8 {
  loop {
    println!("List of Options");
    println!("Option 1: add employee");
    println!("Option 2: list all people in a department");
    println!("Option 3: list all people by department");
    println!("Option 4: Quit");
    println!("\nEnter option number: ");

    let mut option: String = String::new();

    io::stdin()
      .read_line(&mut option)
      .expect("Invalid input");

    let option = match option.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    return option;
  }
}