use std::env;
use std::io;
use unit_converter::{help, Unit};
use unit_converter::UnitConversion;

fn main() {
    let args:Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("Hello, world!");
    loop {
        help();
        println!("Please input your number: ");
        
        let mut input = String::new();
        let mut value = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input:u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        match input {
            1 => {
                println!("Please enter values you want to convert to kilometer: ");
                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read line.");
                let value:f32 = match value.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid value:");
                        continue;
                    }
                };
                let unit = Unit::new(value);
                let kilometer = unit.meter_to_kilometer();
                println!("{} meters is {} kilometers", unit.unit, kilometer);
            },
            2 => {
                println!("Please enter values you want to convert to meter: ");
                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read line.");
                let value:f32 = match value.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid value:");
                        continue;
                    }
                };
                let unit = Unit::new(value);
                let meters = unit.kilometer_to_meter();
                println!("{} Kilometer is {} meters", unit.unit, meters);
            },
            3 => {
                println!("Please enter values you want to Convert centimeter(cm) to meter(m): ");
                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read line.");
                let value:f32 = match value.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid value:");
                        continue;
                    }
                };
                let unit = Unit::new(value);
                let result = unit.centimeter_to_meter();
                println!("{} Kilometer is {} meters", unit.unit, result);
            },
            4 => {
                println!("Please enter values you want to Convert meter(m) to centimeter(cm): ");
                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read line.");
                let value:f32 = match value.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid value:");
                        continue;
                    }
                };
                let unit = Unit::new(value);
                let result = unit.meter_to_centimeter();
                println!("{} Kilometer is {} meters", unit.unit, result);
            },
            0 => {
                println!("Exiting from application.");
                break;
            },
            _ => {
                println!("Not a valid choice. Do you want to exit from application? Press 0 to exit.");
                continue;
            }
            
        }
        
        break;
    }
    

}





