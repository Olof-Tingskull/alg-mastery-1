use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn get_surplus(resources: &Vec<i32>, goals: &Vec<i32>) -> Vec<i32> {
    resources
        .iter()
        .zip(goals.iter())
        .map(|(r, g)| r - g)
        .collect()
}

#[derive(Debug)]
struct Trade {
    with: usize,
    get: usize,
    give: usize,
}

struct State {
    player_resources: Vec<i32>,
    character_resources: Vec<Vec<i32>>,
    character_goals: Vec<Vec<i32>>,
}

impl State {
    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        for &resource in &self.player_resources {
            (resource > 0).hash(&mut hasher);
        }

        for character_resource in &self.character_resources {
            for &resource in character_resource {
                resource.hash(&mut hasher);
            }
        }

        for character_goal in &self.character_goals {
            for &goal in character_goal {
                goal.hash(&mut hasher);
            }
        }

        hasher.finish()
    }
}

fn search(
    state: State,
    explored_states: &mut HashSet<u64>,
    successful_trades: &mut Vec<Trade>,
) -> bool {
    explored_states.insert(state.hash());

    let State {
        player_resources,
        character_resources,
        character_goals,
    } = state;

    let n = player_resources.len();
    let m = character_resources.len();
    if player_resources.iter().sum::<i32>() < n as i32 {
        return false;
    }
    if player_resources.iter().all(|&r| r != 0) {
        return true;
    }

    let mut possible_trades = Vec::<Trade>::new();

    let character_surplus: Vec<Vec<i32>> = (0..m)
        .map(|i| get_surplus(&character_resources[i], &character_goals[i]))
        .collect();

    for i in 0..n {
        let player_get = player_resources[i];
        if player_get != 0 {
            continue;
        }

        let mut can_trade = false;

        for j in 0..n {
            let player_give = player_resources[j];
            if player_give < 1 {
                continue;
            }

            for k in 0..m {
                if character_surplus[k][i] > 0 && character_surplus[k][j] < 0 {
                    let trade = Trade {
                        with: k,
                        get: i,
                        give: j,
                    };

                    possible_trades.push(trade);
                    can_trade = true;
                }
            }
        }

        if !can_trade {
            return false;
        }
    }

    for trade in possible_trades {
        let Trade { with, get, give } = trade;

        let player_resource_after_trade = {
            let mut r = player_resources.clone();
            r[get] += 1;
            r[give] -= 1;
            r
        };

        let character_resource_after_trade = {
            let mut r = character_resources.clone();
            r[with][get] -= 1;
            r[with][give] += 1;
            r
        };

        let character_goals_after_trade = character_goals.clone();

        let new_state = State {
            player_resources: player_resource_after_trade,
            character_resources: character_resource_after_trade,
            character_goals: character_goals_after_trade,
        };

        if explored_states.contains(&new_state.hash()) {
            continue;
        }
        let could_trade = search(new_state, explored_states, successful_trades);

        if could_trade {
            successful_trades.push(trade);
            return true;
        }
    }

    return false;
}

#[allow(dead_code)]
pub fn run(
    player_resources: Vec<i32>,
    character_resources: Vec<Vec<i32>>,
    character_goals: Vec<Vec<i32>>,
) -> bool {
    let state = State {
        player_resources,
        character_resources,
        character_goals,
    };

    let mut explored_states = HashSet::new();
    let mut successful_trades = Vec::new();

    let res = search(state, &mut explored_states, &mut successful_trades);

    if !res {
        println!("successful_trades: {:?}", successful_trades.len());
        println!("explored_states: {:?}", explored_states.len());
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_trade() {
        let mut rng = rand::thread_rng();
        let n = 10;
        let m = 5;

        loop {
            let player_resources: Vec<i32> = (0..n)
                .map(|_| {
                    if rng.gen_bool(0.5) {
                        rng.gen_range(1..5)
                    } else {
                        0
                    }
                })
                .collect();
            let character_resources: Vec<Vec<i32>> = (0..m)
                .map(|_| (0..n).map(|_| rng.gen_range(0..n as i32)).collect())
                .collect();
            let character_goals: Vec<Vec<i32>> = (0..m)
                .map(|_| (0..n).map(|_| rng.gen_range(0..n as i32)).collect())
                .collect();

            let _ = run(player_resources, character_resources, character_goals);
        }
    }
}
