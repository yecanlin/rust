use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T, U> where T: Fn(U) -> usize {
    calculation: T,
    value: HashMap<U, usize>,
}

impl<T, U> Cacher<T, U> where T: Fn(U) -> usize {
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> usize {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let _expensive_result = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut expensive_result = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    if intensity < 25 {
        println!("Today, do {} pushups!",
                 expensive_result.value(intensity)
                 );
        println!("Next, do {} situps!",
                 expensive_result.value(39)
                 );
    } else {
        if random_number == 3 {
            println!("Tank a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
                );
        }
    }
}
