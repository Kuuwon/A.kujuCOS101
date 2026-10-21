// Week 8 - Project II: The EY Interview
// EY Nigeria is scouting for the developer with the MOST years of
// programming experience. This program is used live during the interview.
//
// Compound data type chosen: a VECTOR OF TUPLES, Vec<(String, u32)>
//   - a tuple (name, years) bundles two different types for one candidate
//   - a vector lets the list grow while the interview is running, because
//     we do not know in advance how many people will be interviewed
//   (an array could not grow, and a tuple alone has a fixed length)

use std::io;

// ---------------------------------------------------------------
// Input helpers
// ---------------------------------------------------------------

// Shows a prompt, reads one line and returns it without the newline.
fn read_text(prompt: &str) -> String {
    println!("{}", prompt);
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read input");
    return text.trim().to_string();
}

// True only when the text is not empty and every character is a digit.
// Checking first means parse() below can never fail on bad typing.
fn is_whole_number(text: &str) -> bool {
    if text.len() == 0 || text.len() > 6 {
        return false;
    }
    for ch in text.chars() {
        if !(ch >= '0' && ch <= '9') {
            return false;
        }
    }
    return true;
}

// Keeps asking until the user types a whole number from low to high.
fn read_number(prompt: &str, low: u32, high: u32) -> u32 {
    loop {
        let text = read_text(prompt);

        if !is_whole_number(&text) {
            println!("  Please type a whole number between {} and {}.\n", low, high);
            continue;
        }

        let number: u32 = text.parse().expect("Input not an integer");
        if number >= low && number <= high {
            return number;
        }
        println!("  Please type a whole number between {} and {}.\n", low, high);
    }
}

// Keeps asking until the user types a name that is not blank.
fn read_name(prompt: &str) -> String {
    loop {
        let name = read_text(prompt);
        if name.len() > 0 {
            return name;
        }
        println!("  The name cannot be empty.\n");
    }
}

// ---------------------------------------------------------------
// Display helpers
// ---------------------------------------------------------------

// Adds spaces on the right so columns line up.
fn pad(text: &str, width: usize) -> String {
    let mut padded = String::from(text);
    while padded.len() < width {
        padded.push(' ');
    }
    return padded;
}

// "1 year" but "5 years" and "0 years".
fn years_label(years: u32) -> String {
    if years == 1 {
        return String::from("1 year");
    }
    return format!("{} years", years);
}

// ---------------------------------------------------------------
// The search
// ---------------------------------------------------------------

// Walks the candidates and keeps the highest years seen so far.
// The slice (&[...]) lets the function look at the vector without owning it.
fn find_highest(candidates: &[(String, u32)]) -> u32 {
    let mut highest = candidates[0].1;
    for i in 1..candidates.len() {
        if candidates[i].1 > highest {
            highest = candidates[i].1;
        }
    }
    return highest;
}

// Collects the names of everyone who has exactly `years` of experience.
// (Two people can tie for first place, so the answer is a vector.)
fn names_with_years(candidates: &[(String, u32)], years: u32) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for i in 0..candidates.len() {
        // t.0 is the name, t.1 is the years (dot-number access)
        if candidates[i].1 == years {
            names.push(candidates[i].0.to_string());
        }
    }
    return names;
}

// Prints every candidate in a neat table, marking the most experienced.
fn print_summary(candidates: &[(String, u32)], highest: u32) {
    println!("\n---------------------------------------------------");
    println!("  {}{}{}", pad("No.", 6), pad("Candidate", 28), "Experience");
    println!("---------------------------------------------------");
    for i in 0..candidates.len() {
        let number = (i + 1).to_string();
        let mut marker = String::new();
        if candidates[i].1 == highest {
            marker.push_str("  <-- highest");
        }
        println!(
            "  {}{}{}{}",
            pad(&number, 6),
            pad(&candidates[i].0, 28),
            years_label(candidates[i].1),
            marker
        );
    }
    println!("---------------------------------------------------");
}

fn main() {
    println!("===================================================");
    println!("        EY NIGERIA - DEVELOPER INTERVIEW");
    println!("   Finding the most experienced programmer");
    println!("===================================================\n");

    // How many people are on the interview list?
    let total = read_number("How many candidates are being interviewed? (1-50)", 1, 50);

    // The vector starts empty and grows by one tuple per candidate
    let mut candidates: Vec<(String, u32)> = Vec::new();

    for count in 0..total {
        println!("\n--- Candidate {} of {} ---", count + 1, total);
        let name = read_name("Full name:");
        let years = read_number("Years of programming experience (0-60):", 0, 60);

        candidates.push((name, years));
    }

    // Find the winner(s)
    let highest = find_highest(&candidates);
    let winners = names_with_years(&candidates, highest);

    print_summary(&candidates, highest);

    // One winner, or a tie?
    if winners.len() == 1 {
        println!("\nMOST EXPERIENCED CANDIDATE: {}", winners[0]);
        println!("Years of programming experience: {}", highest);
    } else {
        println!(
            "\nIT'S A TIE! {} candidates share the highest experience ({}):",
            winners.len(),
            years_label(highest)
        );
        for i in 0..winners.len() {
            println!("  {}. {}", i + 1, winners[i]);
        }
    }
    println!("\nInterview complete. Thank you!");
}
