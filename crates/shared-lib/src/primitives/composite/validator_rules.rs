use crate::primitives::frozen::implements::{Bic, RasAcc, CorAcc};

fn validate_checksum(bic_part: &str, acc: &str) -> bool {
    if acc.starts_with("03100") || acc.starts_with("03200") {
        return true;
    }
    let weights = [
        7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1
    ];

    let sum: u32 = bic_part.chars()
        .chain(acc.chars())
        .enumerate()
        .map(|(i, c)| {
            let digit = c.to_digit(10).unwrap_or(0);
            (digit * weights[i]) % 10
        })
        .sum();
    sum.is_multiple_of(10) 
}

pub(crate) fn validate_ras_bic_acc(bic: &Bic, ras_acc: &RasAcc) -> bool {
    validate_checksum(&bic[6..9], ras_acc)
}

pub(crate) fn validate_cor_bic_acc(bic: &Bic, cor_acc: &CorAcc) -> bool {
    validate_checksum(&bic[3..6], cor_acc)
}
