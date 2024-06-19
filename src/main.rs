mod converter; use converter::{convert_units,Unit,UnitType};
use std::io;
fn get_user_float(prompt:&str) -> f64{
    println!("{}",prompt);
    let mut num_str=String::new();
    io::stdin()
        .read_line(&mut num_str)
        .expect("Failed to read line");
    let user_float:f64 = num_str.trim().parse().expect("Please type a number!");
    user_float

}

fn get_user_unit(prompt:&str) -> String{
    println!("{}",prompt);
    let mut unit_str=String::new();
    io::stdin()
        .read_line(&mut unit_str)
        .expect("Failed to read line");
    let user_unit:String = unit_str.trim().parse().expect("Choose A Unit From The List");
    
    user_unit
}
fn main() {
    println!("List of Implemented Units: ");
    let unit_list = vec!{"   in or inches    |     ft or feet     |     yd or yards    |     mi or mile    |  pm or picometer  |     Angstrom",
                                    "  nm or nanometer  |  mum or micrometer |  mm or millimeter  |  cm or centimeter |     m or meter    |  km or kilometer",
                                    " ns or nanosecond  | mus or microsecond |  ms or millisecond |    s or second    |    min or minute  |     hr or hour    ",
                                    "       day         |        week        |     yr or year     |  mg or milligram  |     g or gram     |  kg or kilogram", 
                                    "   Mg or megagram or t or metric ton    |     lb or pound    |    oz or ounces   |        ton        |  F or Fahrenheit",
                                    "   C or Celsius    |    K or Kelvin"
                                };
    for i in 0..unit_list.len() {
        println!("{}",unit_list[i]);
    }
    println!("");
    println!("Rounds to 4th decimal place for simplicity");
    println!("");
    let scalar = get_user_float("Enter Unit Value");
    let mut unit_in = Unit{
        unit: get_user_unit("Enter Unit To Convert From"),
        unit_type: UnitType::Unknown}
        ;

    let mut unit_out = Unit{
            unit: get_user_unit("Enter Unit To Convert To"),
            unit_type: UnitType::Unknown}
            ;
    let num = convert_units(scalar, &mut unit_in, &mut unit_out);
    println!("{:?}",num);

    
}