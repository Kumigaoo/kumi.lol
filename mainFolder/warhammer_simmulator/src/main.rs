use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    attacks: u32,
    #[arg(short, long)]
    weapon_skill: u8,
    #[arg(short, long)]
    strength: u8,
    #[arg(short, long)]
    toughness: u8,
    #[arg(short, long)]
    save: u8,
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    let mut hits = 0;
    let mut wounds = 0;
    let mut failed_saves = 0;

    for _ in 0..args.attacks {
        let roll = rng.gen_range(1..=6);
        if roll >= args.weapon_skill {
            hits += 1;
            let wound_roll = rng.gen_range(1..=6);
            if wound_roll >= wound_threshold(args.strength, args.toughness) {
                wounds += 1;
                let save_roll = rng.gen_range(1..=6);
                if save_roll < args.save {
                    failed_saves += 1;
                }
            }
        }
    }

    println!("Impactos: {}", hits);
    println!("Heridas: {}", wounds);
    println!("Heridas no salvadas: {}", failed_saves);
}

fn wound_threshold(strength: u8, toughness: u8) -> u8 {
    match strength.cmp(&toughness) {
        std::cmp::Ordering::Greater if strength >= toughness * 2 => 2,
        std::cmp::Ordering::Greater => 3,
        std::cmp::Ordering::Equal => 4,
        std::cmp::Ordering::Less if toughness >= strength * 2 => 6,
        std::cmp::Ordering::Less => 5,
    }
}