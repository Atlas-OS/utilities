fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_ref() {
            "parse" => parse(args[2].as_ref()),
            "add" => println!("{}", args[2].parse::<f64>().unwrap() + args[3].parse::<f64>().unwrap()),
            "rnd" => println!("{}", args[2].parse::<f64>().unwrap().round()),
            "div" => println!("{}", args[2].parse::<f64>().unwrap() / args[3].parse::<f64>().unwrap()),
            "divint" => println!("{}", args[2].parse::<i32>().unwrap() / args[3].parse::<i32>().unwrap()),
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