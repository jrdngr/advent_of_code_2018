use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::str::FromStr;

use regex::Regex;

use crate::Result;

pub fn day7_1() -> Result<()> {
    let inputs: Vec<RulePair> = crate::utils::get_inputs(7);

    let rules = Rules::new(&inputs);

    let mut steps: Vec<char> = rules.keys().cloned().collect();
    steps.sort_by(|s1, s2| rules[&s2][&s1]);

    let answer: String = steps.iter().collect();

    println!("7-1: {}", answer);

    // Not BFGIKLNRTXHPUMWQVZOYJACDSE

    Ok(())
}

pub fn day7_2() -> Result<()> {
    let answer = 0;
    println!("7-2: {}", answer);

    Ok(())
}

#[derive(Debug)]
struct RulePair(char, char);

impl FromStr for RulePair {
    type Err = std::char::ParseCharError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
        let captures = re.captures(s).unwrap();

        let first = captures[1].parse::<char>()?;
        let second = captures[2].parse::<char>()?;

        Ok(RulePair(first, second))
    }
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<char, HashMap<char, Ordering>>,
}

impl Deref for Rules {
    type Target = HashMap<char, HashMap<char, Ordering>>;

    fn deref(&self) -> &HashMap<char, HashMap<char, Ordering>> {
        &self.rules
    }
}

impl Rules {
    pub fn new(rule_pairs: &[RulePair]) -> Self {
        let mut rules = HashMap::new();
        let mut steps: HashSet<char> = HashSet::new();

        // Fill based on inputs
        for rule in rule_pairs {
            steps.insert(rule.0);
            steps.insert(rule.1);
            let entry = rules.entry(rule.0).or_insert(HashMap::new());
            entry.insert(rule.1, Ordering::Greater);
        }

        for step in &steps {
            if !rules.contains_key(step) {
                rules.insert(*step, HashMap::new());
            }
        }

        // Fill based on existing orderings
        let mut new_pairs = Vec::new();
        for (step, children) in &rules {
            for (child, _) in children {
                if rules.contains_key(&child) {
                    let child_rules = &rules[&child];
                    for child_child in child_rules.keys() {
                        new_pairs.push(RulePair(*step, *child_child));
                    }
                }
            }
        }
        for rule in new_pairs {
            let entry = rules.entry(rule.0).or_insert(HashMap::new());
            entry.insert(rule.1, Ordering::Greater);
        }

        // Fill based on incomparables
        let mut new_entries: Vec<(char, char, Ordering)> = Vec::new();
        for (current_step, children) in &rules {
            for step in &steps {
                if !children.contains_key(step) {
                    if rules[step].contains_key(current_step) {
                        new_entries.push((*current_step, *step, Ordering::Less))
                    } else {
                        let ordering = step.cmp(current_step);
                        new_entries.push((*current_step, *step, ordering))
                    }
                }
            }
        }
        for rule in new_entries {
            let entry = rules.entry(rule.0).or_insert(HashMap::new());
            entry.insert(rule.1, rule.2);
        }

        Self { rules }
    }
}
