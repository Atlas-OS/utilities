fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_ref() {
            "parse" => parse(args[2].as_ref()),
            "add" => add(args[2].as_ref(), args[3].as_ref()),
            "rnd" => println!("{}", args[2].parse::<f64>().unwrap().round()),
            "div" => div(args[2].as_ref(), args[3].as_ref()),
            "divint" => divint(args[2].as_ref(), args[3].as_ref()),
            "help" => help(),
            _ => help(),
        }
    } else {
        help();
    }
}

fn parse(path: &str) {
    // OCAT CSVs change amount of headers after the first line, so make reader flexible
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_path(path)
        .unwrap();
    let mut sum: f64 = 0.0;
    let mut sorted_values: Vec<f64> = Vec::new();
    for result in reader.records() {
        let value = result.unwrap()[12].parse::<f64>().unwrap();
        // push into vector for sorting later
        sorted_values.push(value);
        // for counting benchmark time
        sum+=value;
    }
    // sort values in descending order
    sorted_values.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut current_total: f64 = 0.0;
    // collect lows
    for present in sorted_values {
        current_total += present;
        if current_total >= (0.01 / 100.0 * sum) {
            println!("{}", 1000.0/present as f32);
            break;
        }
    }
}

fn add(val1: &str, val2: &str) {
    let val1 = val1.parse::<f64>().unwrap();
    let val2 = val2.parse::<f64>().unwrap();
    println!("{}", val1 + val2);
}

fn div(val1: &str, val2: &str) {
    let val1 = val1.parse::<f64>().unwrap();
    let val2 = val2.parse::<f64>().unwrap();
    println!("{}", val1 / val2);
}

fn divint(val1: &str, val2: &str) {
    let val1 = val1.parse::<i32>().unwrap();
    let val2 = val2.parse::<i32>().unwrap();
    println!("{}", val1 / val2);
}

fn help() {
    println!("{}", "
Usage:
calc parse <path> - calculate %0.01 lows of an OCAT CSV
calc add <val1> <val2> - adds two values
calc rnd <val> - rounds <val> to the nearest integer
calc div <val1> <val2> - integer division
calc divint <val1> <val2> - float division
calc help - prints this message");
}