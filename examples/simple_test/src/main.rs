use carbono::Carbono;

fn main() {
    let carbono = Carbono::now();

    println!("rfc3339: {}", carbono.datetime.to_rfc3339());
    println!("timestamp: {}", carbono.datetime.timestamp());
    println!("rfc2822: {}", carbono.datetime.to_rfc2822());
    println!("date: {}", carbono.datetime.date_naive());
    println!("time: {}", carbono.datetime.time());
}
