use crate::workflow::Operator::LessThan;
use std::ops::{Range, RangeBounds, RangeInclusive};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Hash, Clone, Copy, Eq, PartialEq, Debug)]
pub(crate) struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    pub fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Part { x, m, a, s }
    }

    pub fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub(crate) struct PartRange {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl PartRange {
    pub fn new(
        x: RangeInclusive<usize>,
        m: RangeInclusive<usize>,
        a: RangeInclusive<usize>,
        s: RangeInclusive<usize>,
    ) -> Self {
        PartRange { x, m, a, s }
    }

    pub fn default() -> Self {
        PartRange {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }

    pub fn split_at(
        &self,
        field: char,
        value: usize,
        operator: Operator,
    ) -> (PartRange, PartRange) {
        if !(1..=4000).contains(&value) {
            panic!("Bad value chosen for range 1..=4000: {}", value);
        }

        let adjusted_value = if operator == LessThan {
            value - 1
        } else {
            value
        };

        match field {
            'x' => (
                PartRange::new(
                    *self.x.start()..=adjusted_value,
                    self.m.clone(),
                    self.a.clone(),
                    self.s.clone(),
                ),
                PartRange::new(
                    (adjusted_value + 1)..=*self.x.end(),
                    self.m.clone(),
                    self.a.clone(),
                    self.s.clone(),
                ),
            ),
            'm' => (
                PartRange::new(
                    self.x.clone(),
                    *self.m.start()..=adjusted_value,
                    self.a.clone(),
                    self.s.clone(),
                ),
                PartRange::new(
                    self.x.clone(),
                    (adjusted_value + 1)..=*self.m.end(),
                    self.a.clone(),
                    self.s.clone(),
                ),
            ),
            'a' => (
                PartRange::new(
                    self.x.clone(),
                    self.m.clone(),
                    *self.a.start()..=adjusted_value,
                    self.s.clone(),
                ),
                PartRange::new(
                    self.x.clone(),
                    self.m.clone(),
                    (adjusted_value + 1)..=*self.a.end(),
                    self.s.clone(),
                ),
            ),
            's' => (
                PartRange::new(
                    self.x.clone(),
                    self.m.clone(),
                    self.a.clone(),
                    *self.s.start()..=adjusted_value,
                ),
                PartRange::new(
                    self.x.clone(),
                    self.m.clone(),
                    self.a.clone(),
                    (adjusted_value + 1)..=*self.s.end(),
                ),
            ),
            _ => unreachable!(),
        }
    }

    pub fn compute_combos(&self) -> u64 {
        (*self.x.end() as u64 - *self.x.start() as u64 + 1)
            * (*self.m.end() as u64 - *self.m.start() as u64 + 1)
            * (*self.a.end() as u64 - *self.a.start() as u64 + 1)
            * (*self.s.end() as u64 - *self.s.start() as u64 + 1)
    }
}

#[derive(Debug)]
pub(crate) struct Instruction {
    pub destination: String,
    pub operator: Operator,
    pub value: usize,
    pub part_attribute: char,
}

impl Instruction {
    pub fn new(
        destination: String,
        operator: Operator,
        value: usize,
        part_attribute: char,
    ) -> Self {
        match part_attribute {
            'x' | 'm' | 'a' | 's' => Instruction {
                destination,
                operator,
                value,
                part_attribute,
            },
            _ => unreachable!(),
        }
    }

    pub fn matches(&self, part: Part) -> bool {
        let value_to_compare = match self.part_attribute {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => unreachable!(),
        };

        match self.operator {
            Operator::LessThan => value_to_compare < self.value,
            Operator::GreaterThan => value_to_compare > self.value,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Workflow {
    pub name: String,
    pub instructions: Vec<Instruction>,
    pub default: String,
}

impl Workflow {
    pub fn new(name: String, instructions: Vec<Instruction>, default: String) -> Self {
        Workflow {
            name,
            instructions,
            default,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::workflow::{Operator, PartRange};

    #[test]
    fn part_range_test() {
        assert_eq!(4000_u64.pow(4), PartRange::default().compute_combos());
        assert_eq!(
            1415 * 4000 * 2005 * 1350,
            PartRange::new(1..=1415, 1..=4000, 1..=2005, 1..=1350).compute_combos()
        );
    }

    #[test]
    fn part_range_split_test() {
        let part_range = PartRange::new(1..=100, 1..=100, 1..=100, 1..=100);

        assert_eq!(
            part_range.split_at('x', 50, Operator::LessThan),
            (
                PartRange::new(1..=49, 1..=100, 1..=100, 1..=100),
                PartRange::new(50..=100, 1..=100, 1..=100, 1..=100)
            )
        );
        assert_eq!(
            part_range.split_at('x', 50, Operator::GreaterThan),
            (
                PartRange::new(1..=50, 1..=100, 1..=100, 1..=100),
                PartRange::new(51..=100, 1..=100, 1..=100, 1..=100)
            )
        );
    }
}
