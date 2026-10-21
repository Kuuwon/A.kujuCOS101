// Week 8 - Project I: The APS Level Checker
// A Public Service APS level checker for the Federal Government of Nigeria.
//
// How it works:
//   The whole APS table is stored in ONE vector of tuples:
//       (career track, job title, APS level)
//   The user picks a track, a job title and the level the staff member
//   CLAIMS to hold. The program looks the title up in the vector and
//   reports whether the claim is VALID or INVALID.

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
fn read_choice(prompt: &str, low: usize, high: usize) -> usize {
    loop {
        let text = read_text(prompt);

        if !is_whole_number(&text) {
            println!("  Please type a number between {} and {}.\n", low, high);
            continue;
        }

        let choice: usize = text.parse().expect("Input not an integer");
        if choice >= low && choice <= high {
            return choice;
        }
        println!("  Please type a number between {} and {}.\n", low, high);
    }
}

// ---------------------------------------------------------------
// Data
// ---------------------------------------------------------------

// The APS table, one record per position: (track, job title, level).
// 'static is used because the text is hardcoded in the program.
fn build_table() -> Vec<(&'static str, &'static str, &'static str)> {
    let table = vec![
        // Office Admin
        ("Office Admin", "Intern", "APS 1-2"),
        ("Office Admin", "Administrator", "APS 3-5"),
        ("Office Admin", "Senior Administrator", "APS 5-8"),
        ("Office Admin", "Office Manager", "EL1 8-10"),
        ("Office Admin", "Director", "EL2 10-13"),
        ("Office Admin", "CEO", "SES"),
        // Academic (there is no APS 1-2 position)
        ("Academic", "Research Assistant", "APS 3-5"),
        ("Academic", "PhD Candidate", "APS 5-8"),
        ("Academic", "Post-Doc Researcher", "EL1 8-10"),
        ("Academic", "Senior Lecturer", "EL2 10-13"),
        ("Academic", "Dean", "SES"),
        // Lawyer
        ("Lawyer", "Paralegal", "APS 1-2"),
        ("Lawyer", "Junior Associate", "APS 3-5"),
        ("Lawyer", "Associate", "APS 5-8"),
        ("Lawyer", "Senior Associate 1-2", "EL1 8-10"),
        ("Lawyer", "Senior Associate 3-4", "EL2 10-13"),
        ("Lawyer", "Partner", "SES"),
        // Teacher
        ("Teacher", "Placement", "APS 1-2"),
        ("Teacher", "Classroom Teacher", "APS 3-5"),
        ("Teacher", "Senior Teacher", "APS 5-8"),
        ("Teacher", "Leading Teacher", "EL1 8-10"),
        ("Teacher", "Deputy Principal", "EL2 10-13"),
        ("Teacher", "Principal", "SES"),
    ];
    return table;
}

// ---------------------------------------------------------------
// Table look-ups (each one walks the vector of tuples)
// ---------------------------------------------------------------

// Every job title that belongs to one career track, in level order.
fn titles_in_track(
    table: &[(&'static str, &'static str, &'static str)],
    track: &str,
) -> Vec<&'static str> {
    let mut titles: Vec<&'static str> = Vec::new();
    for i in 0..table.len() {
        // Destructure the tuple into three named variables
        let (row_track, row_title, _row_level) = table[i];
        if row_track == track {
            titles.push(row_title);
        }
    }
    return titles;
}

// The official APS level of a job title within a track.
fn level_of(
    table: &[(&'static str, &'static str, &'static str)],
    track: &str,
    title: &str,
) -> &'static str {
    for i in 0..table.len() {
        let (row_track, row_title, row_level) = table[i];
        if row_track == track && row_title == title {
            return row_level;
        }
    }
    return "Unknown";
}

// The job title a track holds at a given level ("none" if it has no such post).
fn title_at_level(
    table: &[(&'static str, &'static str, &'static str)],
    track: &str,
    level: &str,
) -> &'static str {
    for i in 0..table.len() {
        let (row_track, row_title, row_level) = table[i];
        if row_track == track && row_level == level {
            return row_title;
        }
    }
    return "none";
}

// ---------------------------------------------------------------
// Display helpers
// ---------------------------------------------------------------

// Prints a numbered list: 1. first item, 2. second item, ...
fn print_menu(items: &[&str]) {
    for i in 0..items.len() {
        println!("  {}. {}", i + 1, items[i]);
    }
}

// Adds spaces on the right so columns line up.
fn pad(text: &str, width: usize) -> String {
    let mut padded = String::from(text);
    while padded.len() < width {
        padded.push(' ');
    }
    return padded;
}

// Prints the whole APS table, one career track at a time.
fn show_table(table: &[(&'static str, &'static str, &'static str)], tracks: &[&str]) {
    println!("\n==================== APS TABLE ====================");
    for t in 0..tracks.len() {
        println!("\n{}", tracks[t]);
        println!("  {}{}", pad("Level", 12), "Job title");
        for i in 0..table.len() {
            let (row_track, row_title, row_level) = table[i];
            if row_track == tracks[t] {
                println!("  {}{}", pad(row_level, 12), row_title);
            }
        }
    }
    println!("\n===================================================");
}

// ---------------------------------------------------------------
// The checker
// ---------------------------------------------------------------

fn validate_staff(
    table: &[(&'static str, &'static str, &'static str)],
    tracks: &[&str],
    levels: &[&str],
) {
    // Step 1: career track
    println!("\nSelect the career track:");
    print_menu(tracks);
    let track_choice = read_choice("Track number:", 1, tracks.len());
    let track = tracks[track_choice - 1];

    // Step 2: job title (only the titles that belong to that track)
    let titles = titles_in_track(table, track);
    println!("\nSelect the job title:");
    print_menu(&titles);
    let title_choice = read_choice("Job title number:", 1, titles.len());
    let title = titles[title_choice - 1];

    // Step 3: the level being claimed
    println!("\nSelect the APS level the staff member claims to hold:");
    print_menu(levels);
    let level_choice = read_choice("Level number:", 1, levels.len());
    let claimed = levels[level_choice - 1];

    // Step 4: compare the claim with the table
    let official = level_of(table, track, title);

    println!("\n---------------------------------------------------");
    println!("Career track   : {}", track);
    println!("Job title      : {}", title);
    println!("Claimed level  : {}", claimed);
    println!("Official level : {}", official);
    println!("---------------------------------------------------");

    if claimed == official {
        println!("RESULT: VALID. {} ({}) holds {}.", title, track, official);
    } else {
        println!(
            "RESULT: INVALID. {} ({}) holds {}, not {}.",
            title, track, official, claimed
        );

        // Help the officer: what does this track actually hold at the claimed level?
        let real_title = title_at_level(table, track, claimed);
        if real_title == "none" {
            println!("The {} track has no position at {}.", track, claimed);
        } else {
            println!("At {} the {} track holds: {}.", claimed, track, real_title);
        }
    }
    println!("---------------------------------------------------");
}

fn main() {
    // The data: one vector of tuples, plus two small vectors for the menus
    let table = build_table();
    let tracks = vec!["Office Admin", "Academic", "Lawyer", "Teacher"];
    let levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    println!("===================================================");
    println!("   FEDERAL GOVERNMENT OF NIGERIA - APS LEVEL CHECKER");
    println!("===================================================");

    // Main menu: repeats until the user chooses to quit
    loop {
        println!("\nMAIN MENU");
        println!("  1. Validate a staff level");
        println!("  2. View the full APS table");
        println!("  3. Quit");

        let option = read_choice("Choose an option (1-3):", 1, 3);

        if option == 1 {
            validate_staff(&table, &tracks, &levels);
        } else if option == 2 {
            show_table(&table, &tracks);
        } else {
            println!("\nThank you for using the APS Level Checker. Goodbye!");
            break;
        }
    }
}
