use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<(Vec<Rule>, Vec<Vec<u32>>), u32, u32> for Impl {
    fn parse(&self, input: String) -> (Vec<Rule>, Vec<Vec<u32>>) {
        let mut rules: Vec<Rule> = vec![];
        let mut pages: Vec<Vec<u32>> = vec![];

        let mut reading_rules = true;

        for line in input.lines() {
            if line.is_empty() {
                reading_rules = false;
                continue;
            }

            if reading_rules {
                if let [left, right] = line.split('|').collect::<Vec<&str>>()[..] {
                    rules.push(Rule {
                        left: left.parse().unwrap(),
                        right: right.parse().unwrap(),
                    });
                }
            } else {
                pages.push(line.split(',').map(|v| v.parse().unwrap()).collect())
            }
        }

        (rules, pages)
    }

    fn part_1(&self, (rules, piles): &(Vec<Rule>, Vec<Vec<u32>>)) -> u32 {
        piles
            .iter()
            .filter(|pile| is_pile_ordered(pile, rules))
            .map(|pile| get_center_page(pile))
            .sum()
    }

    fn part_2(&self, (rules, piles): &(Vec<Rule>, Vec<Vec<u32>>)) -> u32 {
        piles
            .iter()
            .filter(|pile| !is_pile_ordered(pile, rules))
            .map(|pile| order_pile(pile, rules))
            .map(|pile| get_center_page(&pile))
            .sum()
    }
}

fn is_pile_ordered(pages: &Vec<u32>, rules: &[Rule]) -> bool {
    rules.iter().all(|rule| follows_rule(pages, rule))
}

fn follows_rule(pages: &Vec<u32>, rule: &Rule) -> bool {
    let mut found_left: bool = false;
    let mut found_right: bool = false;

    for page in pages {
        if *page == rule.left {
            if found_right {
                return false;
            }
            found_left = true;
        } else if *page == rule.right {
            if found_left {
                return true;
            }
            found_right = true;
        }
    }

    true
}

fn order_pile(pages: &Vec<u32>, rules: &[Rule]) -> Vec<u32> {
    let mut temp = pages.to_vec();

    while !is_pile_ordered(&temp, rules) {
        for rule in rules {
            if !follows_rule(&temp, rule) {
                let left_index = temp.iter().position(|page| *page == rule.left).unwrap();
                let right_index = temp.iter().position(|page| *page == rule.right).unwrap();

                let left = temp[left_index];
                let right = temp[right_index];

                temp[left_index] = right;
                temp[right_index] = left;
            }
        }
    }

    temp
}

fn get_center_page(pages: &[u32]) -> u32 {
    *pages
        .get(pages.len() / 2)
        .expect("Failed to get center page")
}

pub(crate) struct Rule {
    left: u32,
    right: u32,
}
