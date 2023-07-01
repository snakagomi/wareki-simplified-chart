fn main() {
    let mut shouwa_count = 1;
    let mut heisei_count = 1;
    let mut reiwa_count = 1;
    for i in 1926..2027 {
        if 1926 <= i && i < 1989 {
            wareki_changer(i, "昭和", &mut shouwa_count);
        } else if 1989 <= i && i < 2020 {
            wareki_changer(i, "平成", &mut heisei_count);
        } else {
            wareki_changer(i, "令和", &mut reiwa_count);
        }
    }
}

fn wareki_changer(seireki: i32, wareki: &str, age_count: &mut i32) {
    if *age_count == 1 {
        println!("西暦：{}年 = {}：元年", seireki, wareki);
    } else {
        println!("西暦：{}年 = {}：{}年", seireki, wareki, age_count);
    }
    *age_count += 1;
}
