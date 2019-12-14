#![deny(rust_2018_idioms)]

use num_integer::Integer;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::ops::{DivAssign, MulAssign};

type Reactions<'a> = HashMap<&'a str, Reaction<'a>>;

#[derive(Debug, Clone)]
struct Reaction<'a> {
    inputs: Vec<Value<'a>>,
    output: Value<'a>,
}

impl<'a> Reaction<'a> {
    fn parse(line: &'a str) -> Option<Reaction<'a>> {
        let mut iter = line.trim().split("=>");
        Some(Reaction {
            inputs: iter
                .next()?
                .split(", ")
                .map(Value::parse)
                .collect::<Option<_>>()?,
            output: Value::parse(iter.next()?)?,
        })
    }

    fn flatten(&mut self) {
        let mut map = HashMap::new();
        for input in &self.inputs {
            map.entry(input.unit)
                .and_modify(|v| *v += input.amount)
                .or_insert(input.amount);
        }
        let mut inputs = Vec::new();
        for input in &self.inputs {
            if let Some(amount) = map.remove(input.unit) {
                inputs.push(Value {
                    amount,
                    unit: input.unit,
                });
            }
        }
        self.inputs = inputs;

        self.div_assign(
            self.inputs
                .iter()
                .map(|input| input.amount)
                .fold(self.output.amount, |a, b| a.gcd(&b)),
        );
    }
}

impl Display for Reaction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for input in &self.inputs {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", input)?;
            first = false;
        }
        write!(f, " => {}", self.output)
    }
}

impl<'a> MulAssign<u64> for Reaction<'a> {
    fn mul_assign(&mut self, rhs: u64) {
        for input in self.inputs.iter_mut() {
            *input *= rhs;
        }
        self.output *= rhs;
    }
}

impl<'a> DivAssign<u64> for Reaction<'a> {
    fn div_assign(&mut self, rhs: u64) {
        for input in self.inputs.iter_mut() {
            *input /= rhs;
        }
        self.output /= rhs;
    }
}

#[derive(Debug, Clone)]
struct Value<'a> {
    amount: u64,
    unit: &'a str,
}

impl Value<'_> {
    fn parse(value: &str) -> Option<Value<'_>> {
        let mut iter = value.trim().split_whitespace();
        Some(Value {
            amount: iter.next()?.parse().ok()?,
            unit: iter.next()?,
        })
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.unit)
    }
}

impl<'a> MulAssign<u64> for Value<'a> {
    fn mul_assign(&mut self, rhs: u64) {
        self.amount *= rhs;
    }
}

impl<'a> DivAssign<u64> for Value<'a> {
    fn div_assign(&mut self, rhs: u64) {
        self.amount /= rhs;
    }
}

fn parse_reactions(input: &str) -> Option<Reactions<'_>> {
    input
        .lines()
        .map(|line| Reaction::parse(line).map(|rxn| (rxn.output.unit, rxn)))
        .collect()
}

fn ore_for_fuel(rxns: &Reactions<'_>) -> u64 {
    let mut required = HashMap::new();
    required.insert("FUEL", 1i64);
    loop {
        let units = required
            .iter()
            .filter_map(|(unit, amount)| {
                if *unit == "ORE" || *amount <= 0 {
                    None
                } else {
                    Some(*unit)
                }
            })
            .collect::<Vec<_>>();
        if units.is_empty() {
            break *required.get("ORE").unwrap() as u64;
        }
        let mut extra = HashMap::new();
        for unit in units {
            let amount = required.get_mut(unit).unwrap();
            let rxn = rxns.get(&unit).unwrap();
            let factor = (*amount / rxn.output.amount as i64)
                + if (*amount % rxn.output.amount as i64) == 0 {
                    0
                } else {
                    1
                };
            match (rxn.output.amount as i64 * factor) - *amount {
                0 => {}
                e => {
                    extra.insert(unit, e);
                }
            }
            *amount = 0;
            for input in &rxn.inputs {
                let amount = input.amount as i64 * factor;
                required
                    .entry(input.unit)
                    .and_modify(|v| *v += amount)
                    .or_insert(amount);
            }
        }
        for (unit, extra) in extra {
            *required.get_mut(unit).unwrap() -= extra as i64;
        }
    }
}

fn ore_for_fuel_exact(rxns: &Reactions<'_>, input_ore: u64) -> u64 {
    let mut rxns = rxns.clone();
    for rxn in rxns.values_mut() {
        rxn.flatten();
    }

    while let Some(rxn) = rxns.values().find(|rxn| {
        rxn.inputs.len() == 1 && rxn.inputs[0].unit == "ORE" && rxn.output.unit != "FUEL"
    }) {
        // {ore} ORE => {amount} {unit}
        let ore = rxn.inputs[0].amount;
        let amount = rxn.output.amount;
        let unit = rxn.output.unit;

        for rxn in rxns
            .values_mut()
            .filter(|rxn| rxn.inputs.iter().any(|input| input.unit == unit))
        {
            // {other} _, ... => _ {unit}
            let other = rxn
                .inputs
                .iter()
                .find(|input| input.unit == unit)
                .unwrap()
                .amount;
            let lcm = amount.lcm(&other);
            *rxn *= lcm / other;
            let input = rxn
                .inputs
                .iter_mut()
                .find(|input| input.unit == unit)
                .unwrap();
            *input = Value {
                amount: lcm / amount * ore,
                unit: "ORE",
            };
            rxn.flatten();
        }

        rxns.remove(unit);
    }

    let rxn = rxns.get("FUEL").unwrap();
    assert!(rxn.inputs.len() == 1 && rxn.inputs[0].unit == "ORE");
    let ore = rxn.inputs[0].amount;
    let amount = rxn.output.amount;
    (input_ore as f64 / ore as f64 * amount as f64) as u64
}

fn main() {
    let input = util::read_input();
    let rxns = parse_reactions(&input).unwrap();
    println!("part 1: {}", ore_for_fuel(&rxns));
    // this answer wasn't actually right, but it minus 1 was. oh well
    println!("part 2: {}", ore_for_fuel_exact(&rxns, 1000000000000));
}

#[cfg(test)]
#[test]
fn test() {
    let input = "9 ORE => 2 A
                 8 ORE => 3 B
                 7 ORE => 5 C
                 3 A, 4 B => 1 AB
                 5 B, 7 C => 1 BC
                 4 C, 1 A => 1 CA
                 2 AB, 3 BC, 4 CA => 1 FUEL";
    let rxns = parse_reactions(input).unwrap();
    assert_eq!(ore_for_fuel(&rxns), 165);

    let input = "171 ORE => 8 CNZTR
                 7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                 114 ORE => 4 BHXH
                 14 VRPVC => 6 BMBT
                 6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                 6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                 15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                 13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                 5 BMBT => 4 WPTQ
                 189 ORE => 9 KTJDG
                 1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                 12 VRPVC, 27 CNZTR => 2 XDBXC
                 15 KTJDG, 12 BHXH => 5 XCVML
                 3 BHXH, 2 VRPVC => 7 MZWV
                 121 ORE => 7 VRPVC
                 7 XCVML => 6 RJRHP
                 5 BHXH, 4 VRPVC => 5 LTCX";
    let rxns = parse_reactions(input).unwrap();
    assert_eq!(ore_for_fuel(&rxns), 2210736);
    assert_eq!(ore_for_fuel_exact(&rxns, 1000000000000), 460664);
}
