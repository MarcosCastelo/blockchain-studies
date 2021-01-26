use blockchainlib::*;

fn main() {
    let block = Block::new(12, now(), vec![0; 32], 0, "Marcos block!".to_owned());

    println!("{:?}", &block);
}

