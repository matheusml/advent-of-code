mod input;
mod report;
fn main() {
    let reports = input::read_input();

    let mut safe_reports = 0;

    for report in reports {
        if report::is_safe(report) {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {}", safe_reports);
}
