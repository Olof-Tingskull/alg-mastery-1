use std::{collections::HashSet, iter::once};


#[derive(Debug, Clone, Copy)]
pub struct Offer {
    free: usize,
    buy: usize,
}

#[derive(Debug, Clone)]
struct Payment {
    pay_for_amount: usize,
    using_offers: HashSet<usize>,
}

#[allow(dead_code)]
pub fn run(total_to_buy: usize, offers: &Vec<Offer>, price_per_movie: f64) -> f64 {
    let mut quantity_payment_dp = (0..=total_to_buy)
        .map(|i| Payment { pay_for_amount: i, using_offers: HashSet::new() })
        .collect::<Vec<_>>();

    for current_quentity in 1..=total_to_buy {
        let no_offer_payment = {
            let mut p = quantity_payment_dp[current_quentity - 1].clone();
            p.pay_for_amount += 1;
            p
        };

         let payment_options: Vec<_> = offers
            .iter()
            .enumerate()
            .filter_map(|(index, offer)| {
                if offer.buy >= current_quentity {
                    let mut used_offers = HashSet::new();
                    used_offers.insert(index);

                    let p =  Payment {
                        pay_for_amount: offer.buy - offer.free,
                        using_offers: used_offers,
                    };

                    return Some(p);
                } else {
                    let remaining_payment = &quantity_payment_dp[current_quentity - offer.buy];
                    if remaining_payment.using_offers.contains(&index) {
                        return None;
                    }

                    let mut used_offers = remaining_payment.using_offers.clone();
                    used_offers.insert(index);

                    let p =  Payment {
                        pay_for_amount: remaining_payment.pay_for_amount + offer.buy - offer.free,
                        using_offers: used_offers,
                    };

                    return Some(p);
                }
            })
            .chain(once(no_offer_payment))
            .collect();

        let best_payment = payment_options
            .iter()
            .min_by(|a, b| a.pay_for_amount.cmp(&b.pay_for_amount))
            .unwrap();

        quantity_payment_dp[current_quentity] = best_payment.clone();
    }

    quantity_payment_dp[total_to_buy].pay_for_amount as f64 * price_per_movie
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_offers() {
        use super::*;

        let offers = vec![
            Offer { free: 2, buy: 4 },
            Offer { free: 5, buy: 9 },
            Offer { free: 2, buy: 3 },
        ];

        let price_per_movie = 99.;

        for i in 1..=18 {
            println!("{}", run(i, &offers, price_per_movie));
        }
    }
}
