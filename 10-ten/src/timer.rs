#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Noop,
    AddX(i64),
}

#[derive(Debug)]
pub struct Timer {
    ctr: String,
    current_cycle: usize,
    value: i64,
    next_value: Option<i64>,
    signial_strengh_over_time: Vec<i64>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            ctr: String::new(),
            current_cycle: 0,
            value: 1,
            next_value: None,
            signial_strengh_over_time: vec![],
        }
    }
    pub fn next_cycle(&mut self, command: Command) {
        self.increment();
        match command {
            Command::Noop => {
                self.next_value = None;
            }
            Command::AddX(value) => {
                let _ = self.next_value.insert(value);
                self.consume_next_value();
            }
        }
    }

    pub fn get_sum(&self) -> i64 {
        self.signial_strengh_over_time.iter().sum()
    }

    pub fn get_ctr(&self) -> String {
        self.ctr.clone()
    }

    fn increment(&mut self) {
        self.current_cycle += 1;
        self.add_signal_strengh();
        self.draw();
    }

    fn consume_next_value(&mut self) {
        if let Some(value) = self.next_value {
            self.value += value;
        }
        self.increment();
    }

    fn is_sharp(&self) -> bool {
        let spirit_start = self.value - 1;
        let spirit_end = self.value + 1;
        let cycle = self.current_cycle as i64 % 40;
        spirit_end >= cycle && cycle >= spirit_start
    }

    fn draw(&mut self) {
        if self.is_sharp() {
            self.ctr.push('#');
        } else {
            self.ctr.push('.');
        }
        if self.current_cycle % 40 == 0 {
            self.ctr.push('\n');
        }
    }

    fn is_observed(&self) -> bool {
        let observed_cycle = [20, 60, 100, 140, 180, 220];
        observed_cycle.contains(&self.current_cycle)
    }

    fn add_signal_strengh(&mut self) {
        if self.is_observed() {
            self.signial_strengh_over_time
                .push(self.current_cycle as i64 * self.value);
            println!(
                "cycle : {}, {:?}",
                self.current_cycle, self.signial_strengh_over_time
            );
        }
    }
}
