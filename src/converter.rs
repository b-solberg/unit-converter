fn check_unit_type(unit:&mut Unit) {
    match unit.unit.as_str() {
        "ft" | "feet" => unit.unit_type = UnitType::Length,
        "mi" | "mile" => unit.unit_type = UnitType::Length,
        "m" | "meter" => unit.unit_type = UnitType::Length,
        "in" | "inches" => unit.unit_type = UnitType::Length,
        "yd" | "yards" => unit.unit_type = UnitType::Length,
        "mm" | "millimeter" => unit.unit_type = UnitType::Length,
        "mum" | "micrometer" => unit.unit_type = UnitType::Length,
        "nm" | "nanometer" => unit.unit_type = UnitType::Length,
        "cm" | "centimeter" => unit.unit_type = UnitType::Length,
        "km" | "kilometer" => unit.unit_type = UnitType::Length,
        "pm" | "picometer" => unit.unit_type = UnitType::Length,
        "Angstrom" => unit.unit_type = UnitType::Length,
        "min" | "minute" => unit.unit_type = UnitType::Time,
        "s" | "second" => unit.unit_type = UnitType::Time,
        "hr" | "hour" => unit.unit_type = UnitType::Time,
        "day" => unit.unit_type = UnitType::Time,
        "yr" | "year" => unit.unit_type = UnitType::Time,
        "week" => unit.unit_type = UnitType::Time,
        "ms" | "millisecond" => unit.unit_type = UnitType::Time,
        "mus" | "microsecond" => unit.unit_type = UnitType::Time,
        "ns" | "nanosecond" => unit.unit_type = UnitType::Time,
        "g" | "gram" => unit.unit_type = UnitType::Mass,
        "kg" | "kilogram" => unit.unit_type = UnitType::Mass,
        "Mg" | "megagram" | "t" | "metric ton" => unit.unit_type = UnitType::Mass,
        "mg" | "milligram" => unit.unit_type = UnitType::Mass,
        "lb" | "lbs" | "pound" => unit.unit_type = UnitType::Mass,
        "oz" | "ounces" => unit.unit_type = UnitType::Mass,
        "ton" => unit.unit_type = UnitType::Mass,
        "F" | "Fahrenheit" => unit.unit_type = UnitType::Temperature,
        "C" | "Celsius" => unit.unit_type = UnitType::Temperature,
        "K" | "Kelvin" => unit.unit_type = UnitType::Temperature,
        _ => {println!("Unknown unit"); unit.unit_type = UnitType::Unknown },
    }
}
pub struct Unit {
    pub unit: String,
    pub unit_type: UnitType
}
#[derive(PartialEq)]
#[derive(Debug)]
pub enum UnitType {
    Unknown,
    Length,
    Time,
    Mass,
    Temperature
}  
fn match_unit_type(unit_in:&Unit,unit_out:&Unit ) -> bool {
    
    if unit_in.unit_type == unit_out.unit_type {
        true
    }
    else {
        false
    }
}
fn convert_lengths(dist:f64,unit_in:&Unit,unit_out:&Unit) -> f64 {
    let mut intermediate_multiplier: f64 = 0.0;
    match unit_in.unit.as_str(){
        "ft" | "feet" => intermediate_multiplier = 0.3048,
        "m" | "meter" => intermediate_multiplier = 1.0,
        "in" | "inches" => intermediate_multiplier = 0.0254,
        "yd" | "yards" => intermediate_multiplier = 0.9144,
        "mi" | "mile" => intermediate_multiplier = 1609.34,
        "mm" | "millimeter" => intermediate_multiplier = 1.0e-3,
        "mum" | "micrometer" => intermediate_multiplier = 1.0e-6,
        "nm" | "nanometer" => intermediate_multiplier = 1.0e-9,
        "cm" | "centimeter" => intermediate_multiplier = 1.0e-2,
        "km" | "kilometer" => intermediate_multiplier = 1.0e3,
        "pm" | "picometer" => intermediate_multiplier = 1.0e-12,
        "Angstrom" => intermediate_multiplier = 1.0e-10,
        _ => println!("Conversion Not Implemented Properly"),
    }
    let normalized_dist: f64= dist*intermediate_multiplier;
    match unit_out.unit.as_str(){
        "ft" | "feet" => intermediate_multiplier = 0.3048,
        "m" | "meter" => intermediate_multiplier = 1.0,
        "in" | "inches" => intermediate_multiplier = 0.0254,
        "yd" | "yards" => intermediate_multiplier = 0.9144,
        "mi" | "mile" => intermediate_multiplier = 1609.34,
        "mm" | "millimeter" => intermediate_multiplier = 1.0e-3,
        "mum" | "micrometer" => intermediate_multiplier = 1.0e-6,
        "nm" | "nanometer" => intermediate_multiplier = 1.0e-9,
        "cm" | "centimeter" => intermediate_multiplier = 1.0e-2,
        "km" | "kilometer" => intermediate_multiplier = 1.0e3,
        "pm" | "picometer" => intermediate_multiplier = 1.0e-12,
        "Angstrom" => intermediate_multiplier = 1.0e-10,
        _ => println!("Conversion Not Implemented Properly"),
    }
    normalized_dist/intermediate_multiplier

} 

fn convert_masses(mass:f64,unit_in:&Unit,unit_out:&Unit) -> f64 {
    let mut intermediate_multiplier: f64 = 0.0;
    match unit_in.unit.as_str(){
        "g" | "gram" => intermediate_multiplier = 1.0,
        "kg" | "kilogram" => intermediate_multiplier = 1.0e3,
        "Mg" | "megagram" | "t" | "metric ton" => intermediate_multiplier = 1.0e6,
        "mg" | "milligram" => intermediate_multiplier = 1.0e-3,
        "lb" | "lbs" | "pound" => intermediate_multiplier = 453.592,
        "oz" | "ounces" => intermediate_multiplier = 28.3495,
        "ton" => intermediate_multiplier = 907185.0,
        _ => println!("Conversion Not Implemented Properly"),
    }
    let normalized_mass: f64= mass*intermediate_multiplier;
    match unit_out.unit.as_str(){
        "g" | "gram" => intermediate_multiplier = 1.0,
        "kg" | "kilogram" => intermediate_multiplier = 1.0e3,
        "Mg" | "megagram" | "t" | "metric ton" => intermediate_multiplier = 1.0e6,
        "mg" | "milligram" => intermediate_multiplier = 1.0e-3,
        "lb" | "lbs" | "pound" => intermediate_multiplier = 453.592,
        "oz" | "ounces" => intermediate_multiplier = 28.3495,
        "ton" => intermediate_multiplier = 907185.0,
        _ => println!("Conversion Not Implemented Properly"),
    }
    normalized_mass/intermediate_multiplier
}

fn convert_temperature(temperature:f64,unit_in:&Unit,unit_out:&Unit) -> f64 {
    match unit_in.unit.as_str() {
        "F" | "Fahrenheit" => {
            match unit_out.unit.as_str() {
                "C" | "Celsius" => return (temperature-32.0)*5.0/9.0,
                "K" | "Kelvin" => return (temperature-32.0)*5.0/9.0+273.0,
                _ => {println!("Conversion Not Implemented Properly"); 0.0 }
            }
        },
        "C" | "Celsius" => {
            match unit_out.unit.as_str() {
                "F" | "Fahrenheit" => return temperature*9.0/5.0+32.0,
                "K" | "Kelvin" => return temperature+273.0,
                _ => {println!("Conversion Not Implemented Properly"); 0.0}
            }
        },
        "K" | "Kelvin" => {
            match unit_out.unit.as_str() {
                "F" | "Fahrenheit" => return (temperature-273.0)*9.0/5.0+32.0,
                "C" | "Celsius" => return temperature-273.0,
                _ => {println!("Conversion Not Implemented Properly"); 0.0},
            }
        }, 
        _ => {println!("Conversion Not Implemented Properly"); 0.0},
    }
}

fn convert_time(time:f64,unit_in:&Unit,unit_out:&Unit)-> f64 {
    let mut intermediate_multiplier: f64 = 0.0;
    match unit_in.unit.as_str(){
       "min" | "minute" => intermediate_multiplier = 60.0,
        "s" | "second" => intermediate_multiplier = 1.0,
        "hr" | "hour" => intermediate_multiplier = 360.0,
        "day" => intermediate_multiplier = 360.0*24.0,
        "yr" | "year" => intermediate_multiplier = 360.0*24.0*365.0,
        "week" => intermediate_multiplier = 360.0*24.0*7.0,
        "ms" | "millisecond" => intermediate_multiplier = 1.0e-3,
        "mus" | "microsecond" => intermediate_multiplier = 1.0e-6,
        "ns" | "nanosecond" => intermediate_multiplier = 1.0e-9,
        _ => println!("Conversion Not Implemented Properly"),
    }
    let normalized_time = time*intermediate_multiplier;
    match unit_out.unit.as_str(){
        "min" | "minute" => intermediate_multiplier = 60.0,
         "s" | "second" => intermediate_multiplier = 1.0,
         "hr" | "hour" => intermediate_multiplier = 360.0,
         "day" => intermediate_multiplier = 360.0*24.0,
         "yr" | "year" => intermediate_multiplier = 360.0*24.0*365.0,
         "week" => intermediate_multiplier = 360.0*24.0*7.0,
         "ms" | "millisecond" => intermediate_multiplier = 1.0e-3,
         "mus" | "microsecond" => intermediate_multiplier = 1.0e-6,
         "ns" | "nanosecond" => intermediate_multiplier = 1.0e-9,
         _ => println!("Conversion Not Implemented Properly"),
     }
     normalized_time/intermediate_multiplier

}
pub fn convert_units(scalar:f64,unit_in:&mut Unit,unit_out:&mut Unit) -> f64 {
    check_unit_type(unit_in);
    check_unit_type(unit_out);
    
    if match_unit_type(unit_in,unit_out) == true{
        match unit_in.unit_type {
            UnitType::Length => return (1.0e4*convert_lengths(scalar, unit_in, unit_out)).round()/1.0e4,
            UnitType::Mass => return (1.0e4*convert_masses(scalar, unit_in, unit_out).round())/1.0e4,
            UnitType::Temperature => return (1.0e4*convert_temperature(scalar, unit_in, unit_out).round())/1.0e4,
            UnitType::Time => return (1.0e4*convert_time(scalar, unit_in, unit_out).round())/1.0e4,
            UnitType::Unknown => return 0.0,
        }
    }
    else {
        return 0.0
    }
}