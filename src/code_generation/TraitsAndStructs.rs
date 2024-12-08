trait Vehicle
    {
        fn mileage(&self) -> u32;
        fn fuel_type(&self) -> String;
    }

struct Car
    {
        name: String,
        manufacturer: String,
    }

struct Airplane
    {
        manufacturer: String,
        plane_type: String,
    }
