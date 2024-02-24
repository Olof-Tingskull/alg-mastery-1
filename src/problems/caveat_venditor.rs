
#[derive(Debug, Clone, Copy)]
pub struct Offer {
    free: usize,
    buy: usize,
}


#[allow(dead_code)]
pub fn run(total_to_buy: usize, offers: Vec<Offer>, price_per_movie: f64) -> f64 {
    let mut min_cost_for_each_quantity = vec![0; total_to_buy + 1];
    let mut used_offsers = vec![false; offers.len()];

    for i in 1..=total_to_buy {
        let mut min_cost_for_current_quantity = i;
        let mut offer_used_index: Option<usize> = None;


        for (j, offer) in offers.iter().enumerate() {
            if used_offsers[j] { continue; }
            let cost_with_offer = min_cost_for_each_quantity[i - offer.buy] + offer.buy - offer.free;
            if cost_with_offer < min_cost_for_current_quantity {
                min_cost_for_current_quantity = cost_with_offer;
                offer_used_index = Some(j);
            }
        }

        if let Some(offer_index) = offer_used_index {
            used_offsers[offer_index] = true;
        }

        min_cost_for_each_quantity[i] = min_cost_for_current_quantity;
    }

    min_cost_for_each_quantity[total_to_buy] as f64 * price_per_movie
}