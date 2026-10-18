/// The greeting printed by the application.
pub const GREETING: &str = "Hello World from Claude Code!";

fn main() {
    println!("{GREETING}");
}

#[cfg(test)]
mod tests {
    use super::GREETING;

    #[test]
    fn greeting_is_exact() {
        assert_eq!(GREETING, "Hello World from Claude Code!");
    }
}
