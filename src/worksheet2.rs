
pub fn run(question: u32){
    match question{
        1 => person_name(),
        2 => {
            let color = Color::Green;
            println!("RGB: {:?}", get_color(color));
        },
        3 => {
            let tup = (-29, 44);
            println!("The sum of {:?} is {}", tup, sum_tuple(tup));
        }, 
        4 => {
            let option = Option::Number(45);
            print_if_num(option);
        }
        5 => {
            let mut s = String::from("Hello");
            append_world(&mut s);
            println!("After appending string: {:?}", append_world(&mut s));
        },
        6 => {
            let book = Book{
                title: String::from("Harry Potter"),
                author: String::from("JK Rowling"),
                pages: 593,
            };
            println!("{}", book.get_title());
        }
        _ => println!("Not implemented yet!")
    }
}

// Question 1 -> Struct called Person. Create an instance of the Person struct and print its name field.


struct Person{
    name: String,
    age: u32,
}

fn person_name(){
    let person = Person{
        name: String::from("Mehak"),
        age: 20,
    };

    println!("Name: {}", person.name);
}

// Question 2 -> Enum called Color.

enum Color{
    Red, 
    Blue, 
    Green,
}

fn get_color(color: Color) -> (u32, u32, u32){
    match color{
        Color::Red => (255, 0, 0),
        Color::Blue => (0, 0, 255),
        Color::Green => (0, 255, 0)
    }
}

// Question 3 -> sum of tuple
fn sum_tuple(tup: (i32, i32)) -> i32{
    tup.0 + tup.1
}

// Question 4 -> print if number

enum Option{
    String,
    Number(i32)
}

fn print_if_num(option: Option){
    match option {
        Option::Number(num) => print!("{}", num),
        Option::String => {}
    }
}

// Question 5 -> borrows and appends " World" to a string
fn append_world(s: &mut String){
    s.push_str(" World!");
}

// Question6 -> Book struct with method that returns reference to title

struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn get_title(&self) -> &String {
        &self.title
    }
}


// Question 7 -> status enum

enum Status {
    Active,
    Inactive,
    Suspended,
}

fn title_and_status(){
    
}