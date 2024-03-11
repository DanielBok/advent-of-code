const DOOR_PK: usize = 1717001;
const CARD_PK: usize = 523731;

const SUB_NO: usize = 7;
const REM_NO: usize = 20201227;

pub fn solve_a() {
    let door_loop = get_loop_size(DOOR_PK);
    let card_loop = get_loop_size(CARD_PK);

    let other_enc_key = get_encryption_key(CARD_PK, door_loop);
    let ans = get_encryption_key(DOOR_PK, card_loop);

    assert_eq!(other_enc_key, ans, "Encryption keys must match");

    println!("Solution A: {}", ans);
}

fn get_loop_size(target: usize) -> usize {
    let mut value = 1;

    let mut loop_size = 0;
    while value != target {
        loop_size += 1;
        value = (value * SUB_NO) % REM_NO;
    }

    loop_size
}

fn get_encryption_key(public_key: usize, loop_size: usize) -> usize {
    let mut key = 1;
    for _ in 0..loop_size {
        key = (key * public_key) % REM_NO;
    }

    key
}

pub fn solve_b() {
    println!("Completed AOC 2020");
}