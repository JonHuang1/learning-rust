mod task1;
mod task2;
mod task3;

fn main() {
    // Given a list of integers, use a vector and return the median 
    // (when sorted, the value in the middle position) and mode 
    // (the value that occurs most often; a hash map will be helpful here) of the list.
    let arr = [1, 2, 3, 4, 5, 6, 1];
    task1::task1_print(&arr);
    let arr = [9, 1];
    task1::task1_print(&arr);
    let arr = [6, 2, 6, 2, 4, 4, 3, 2, 1, 5];
    task1::task1_print(&arr);
    let arr = [6];
    task1::task1_print(&arr);
    let arr = [3, 3, 5, 5, 6, 6, 5, 3];
    task1::task1_print(&arr);

    // Convert strings to pig latin. The first consonant of each word 
    // is moved to the end of the word and “ay” is added, so “first” becomes 
    // “irst-fay.” Words that start with a vowel have “hay” added to the end instead 
    // (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    let string = String::from("hello");
    task2::task2_print(string);
    let string = String::from("first");
    task2::task2_print(string);   
    let string = String::from("apple");
    task2::task2_print(string); 

    // Using a hash map and vectors, create a text interface to allow a user to 
    // add employee names to a department in a company. For example, “Add Sally to 
    // Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all 
    // people in a department or all people in the company by department, sorted alphabetically.
    task3::text_interface();
}