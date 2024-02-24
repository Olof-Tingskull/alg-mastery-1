use std::cell::RefCell;
type Train = (usize, usize);

#[allow(dead_code)]
fn run(capacities: Vec<f64>) {
    let n = capacities.len();

    let trains_dp: Vec<RefCell<Option<Vec<Train>>>> = vec![RefCell::new(None); n + 1];
    trains_dp[0].borrow_mut().replace(vec![]);

    for i in 0..n {
        if let Some(start) = trains_dp[i].borrow_mut().as_mut() {
            let mut capacity = 0.;

            for j in i+1..=n {
                capacity += capacities[j-1];
                let maintenance = ((j - i) as f64).powi(2);
                if capacity < maintenance { continue; }
      
                let trains_here = {
                    let mut t = start.clone();
                    t.push((i, j));
                    t
                };
                
                let mut end_ref = trains_dp[j].borrow_mut();
                if !end_ref.as_mut().is_some_and(|t| t.len() < trains_here.len()) {
                    end_ref.replace(trains_here);
                }
            }
        };
    }

    if let Some(last_split) = trains_dp[n].borrow().as_ref() {
        println!("{:?}", last_split);
    } else {
        println!("No solution found");
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_split_train() {
        let mut rng = rand::thread_rng();
        let size = 2000;
        let carriages: Vec<f64> = (0..size).map(|_| rng.gen_range(0.0..size as f64 / 2.)).collect();

        run(carriages);
    }
}

