pub struct Unit {
    pub unit: f32,
}

// Defining the trait for unit conversions.
pub trait UnitConversion {
    fn meter_to_kilometer(&self) -> f32;
    fn kilometer_to_meter(&self) -> f32;
    fn centimeter_to_meter(&self) -> f32;
    fn meter_to_centimeter(&self) -> f32;
}

// Implementing Unit structure.
impl Unit {
    pub fn new(unit: f32) -> Self {
        Unit { unit }
    }
}

// Implementing the conversion trait for the Unit struct.
impl UnitConversion for Unit {
    fn meter_to_kilometer(&self) -> f32 {
        self.unit / 1000.0
    }

    fn kilometer_to_meter(&self) -> f32 {
        self.unit * 1000.0
    }

    fn centimeter_to_meter(&self) -> f32 {
        self.unit / 100.0
    }

    fn meter_to_centimeter(&self) -> f32 {
        self.unit * 100.0
    }
}

// Function to display help text for unit conversion.
pub fn help() {
    let help_text = "
    =====================================================================================================
    ||  If you want to convert any units to desired units, please follow these instructions and select ||
    ||  an option. For example: If you want to convert meter to kilometer, please press 1.             ||
    ||  1. Press 1 to Convert Meter (m) to Kilometer (Km)                                              ||
    ||  2. Press 2 to Convert Kilometer (Km) to Meter (m)                                              ||
    ||  3. Press 3 to Convert Centimeter (cm) to Meter (m)                                             ||
    ||  4. Press 4 to Convert Meter (m) to Centimeter (cm)                                             ||
    =====================================================================================================
    ";
    println!("{}", help_text);
}