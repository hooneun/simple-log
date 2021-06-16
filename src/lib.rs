#[derive(Debug, PartialEq)]
enum Level {
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Informational,
    Debug,
}

#[derive(Debug, PartialEq)]
enum Kind {
    File,
    //    Telegram,
    //    Slack,
    //    Email
}

#[derive(Debug)]
pub struct Config {
    level: Level,
    kind: Kind,
}

pub struct Logger {}

impl Logger {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_setting() {
        let config = Config {
            level: Level::Emergency,
            kind: Kind::File,
        };

        assert_eq!(Level::Emergency, config.level);
        assert_eq!(Kind::File, config.kind);
    }
}
