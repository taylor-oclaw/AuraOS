extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    let routine = smart_home_routine_morning::SmartHomeRoutineMorning::new();
    routine.wake_up();
    routine.make_coffee();
    routine.check_weather();
    routine.start_workday();
    routine.prepare_lunch();

    loop {}
}

mod smart_home_routine_morning {

    use super::*;

    pub struct SmartHomeRoutineMorning {
        tasks: Vec<String>,
        weather: String,
    }

    impl SmartHomeRoutineMorning {
        pub fn new() -> Self {
            SmartHomeRoutineMorning {
                tasks: Vec::new(),
                weather: String::from("Unknown"),
            }
        }

        pub fn wake_up(&mut self) {
            self.tasks.push(String::from("Wake up"));
            println!("Good morning! Time to start your day.");
        }

        pub fn make_coffee(&mut self) {
            self.tasks.push(String::from("Make coffee"));
            println!("Enjoy your freshly brewed coffee!");
        }

        pub fn check_weather(&mut self) {
            // Simulate fetching weather data
            self.weather = String::from("Sunny");
            println!("Today's weather is {}", self.weather);
        }

        pub fn start_workday(&mut self) {
            self.tasks.push(String::from("Start workday"));
            println!("Time to get ready for work!");
        }

        pub fn prepare_lunch(&mut self) {
            self.tasks.push(String::from("Prepare lunch"));
            println!("Lunch is ready. Enjoy your meal!");
        }
    }
}
