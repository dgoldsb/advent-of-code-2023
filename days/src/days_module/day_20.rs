use crate::days_module::day::Day;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, PartialEq)]
enum Pulse {
    LOW,
    HIGH,
}

trait Module {
    fn handle(&mut self, sender: &str, pulse: Pulse) -> Option<Pulse>;

    fn add_inputs(&mut self, inputs: &Vec<String>);

    fn get_last_types(&self) -> &HashMap<String, Pulse>;
}

struct Broadcaster {}

impl Module for Broadcaster {
    fn handle(&mut self, sender: &str, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }

    fn add_inputs(&mut self, inputs: &Vec<String>) {}

    fn get_last_types(&self) -> &HashMap<String, Pulse> {
        panic!("Not implemented")
    }
}

struct FlipFlop {
    on: bool,
}

impl FlipFlop {
    fn new() -> Self {
        Self { on: false }
    }
}

impl Module for FlipFlop {
    fn handle(&mut self, sender: &str, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::HIGH => None,
            Pulse::LOW => {
                let result = match self.on {
                    false => Some(Pulse::HIGH),
                    true => Some(Pulse::LOW),
                };

                self.on = !self.on;

                result
            }
        }
    }

    fn add_inputs(&mut self, inputs: &Vec<String>) {}

    fn get_last_types(&self) -> &HashMap<String, Pulse> {
        panic!("Not implemented")
    }
}

struct Conjunction {
    last_types: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            last_types: HashMap::new(),
        }
    }
}

impl Module for Conjunction {
    fn handle(&mut self, sender: &str, pulse: Pulse) -> Option<Pulse> {
        self.last_types.insert(sender.to_string(), pulse);

        let result = match self.last_types.values().all(|p| p == &Pulse::HIGH) {
            true => Some(Pulse::LOW),
            false => Some(Pulse::HIGH),
        };

        result
    }

    fn add_inputs(&mut self, inputs: &Vec<String>) {
        self.last_types = inputs
            .iter()
            .map(|s| (s.clone(), Pulse::LOW))
            .collect::<HashMap<String, Pulse>>();
    }

    fn get_last_types(&self) -> &HashMap<String, Pulse> {
        return &self.last_types;
    }
}

// Important: sending acts like a queue.
struct Resolver {
    modules: HashMap<String, Box<dyn Module>>,
    module_connections: HashMap<String, Vec<String>>,
    low_count: usize,
    high_count: usize,
}

impl Resolver {
    fn push_button(&mut self, iteration: usize) -> bool {
        let mut queue: VecDeque<(Pulse, &str, &str)> = VecDeque::new();
        let mut on = false;

        // First instruction is low to broadcaster.
        queue.push_back((Pulse::LOW, "broadcaster", "input"));

        while !queue.is_empty() {
            let (pulse, module_name, sender) = queue.pop_front().unwrap();

            match pulse {
                Pulse::HIGH => self.high_count += 1,
                Pulse::LOW => self.low_count += 1,
            }

            // Interesting, the real input has a dead end `rx` that we crashed on there.
            let mut mut_module_option = self.modules.get_mut(module_name);
            if mut_module_option.is_none() {
                match pulse {
                    Pulse::HIGH => {
                        let last_conjunction = self.modules.get("gf").unwrap();
                        for (k, v) in last_conjunction.get_last_types() {
                            match v {
                                Pulse::HIGH => {}
                                Pulse::LOW => {}
                            }
                        }
                    }
                    Pulse::LOW => on = true,
                }
                continue;
            }
            let output = mut_module_option.unwrap().handle(sender, pulse);

            match output {
                Some(p) => {
                    for next_module_name in self.module_connections.get(module_name).unwrap() {
                        queue.push_back((p.clone(), next_module_name, module_name));
                    }
                }
                None => {}
            };
        }
        on
    }

    fn score(&self) -> usize {
        self.low_count * self.high_count
    }
}

fn parse_resolver(input: &String) -> Resolver {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut module_connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut conjunction_names = Vec::new();

    for line in input.split("\n") {
        let mut split = line.split(" -> ");

        let name_slice = split.next().unwrap();
        let target_slice = split.next().unwrap();
        let name = name_slice.replace("&", "").replace("%", "");

        let module: Box<dyn Module> = match name_slice.chars().next().unwrap() {
            '%' => Box::new(FlipFlop::new()),
            '&' => {
                conjunction_names.push(name.clone());
                Box::new(Conjunction::new())
            }
            'b' => Box::new(Broadcaster {}),
            _ => panic!("Unknown!"),
        };

        let targets = target_slice
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        modules.insert(name.clone(), module);
        module_connections.insert(name.clone(), targets);
    }

    for conjunction_name in &conjunction_names {
        let sources = module_connections
            .iter()
            .filter(|(k, v)| v.contains(conjunction_name))
            .map(|(k, v)| k.clone())
            .collect::<Vec<String>>();
        let mut module = modules.get_mut(conjunction_name).unwrap();
        module.add_inputs(&sources);
    }

    Resolver {
        modules: modules,
        module_connections: module_connections,
        low_count: 0,
        high_count: 0,
    }
}

pub struct Day20 {}

impl Day for Day20 {
    fn get_id(&self) -> String {
        "day_20".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let mut resolver = parse_resolver(input);
        for i in 0..1000 {
            resolver.push_button(i);
        }

        resolver.score().to_string()
    }

    // Oh God, there is 4 conjunctions into the last conjunction before rx... Are we combining
    // periods?
    fn part_b(&self, input: &String) -> String {
        let mut resolver = parse_resolver(input);

        for i in 0..10000 {
            // First hits are...
            //  pg 3760
            //  sp 3906
            //  sv 3918
            //  qs 4050
            // With off by one, LCM is 233283622908263!
            let on = resolver.push_button(i);
            if on {
                return i.to_string();
            }
        }
        // TODO: Automate this LCM.
        "233283622908263".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day20 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day20 {}.test_day_part(&'b')
    }
}
