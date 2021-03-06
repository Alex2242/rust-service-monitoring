use crate::probes::Probes;

#[derive(Debug, Clone)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Default for Severity {
    fn default() -> Self {Severity::Info}
}

impl Severity {
    pub fn to_str(&self) -> String {
        match self {
            Severity::Info => String::from("Info"),
            Severity::Warning => String::from("Warning"),
            Severity::Error => String::from("Error"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Message {
    pub date: String,
    pub service: String,
    pub probe: Probes,
    pub severity: Severity,
    pub header: String,
    pub body: String,
}

impl Message {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn to_str(&self) -> String {
        format!(
            "[{}] {} {}/{}: {}\n{}",
            self.date,
            self.severity.to_str(),
            self.probe.to_str(),
            self.service,
            self.header,
            self.body
        )
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        let self_probe = &self.probe;
        let self_severity = &self.severity;
        let _other_probe = &other.probe;
        let _other_severity = &other.severity;

        self.date == other.date &&
        self.service == other.service &&
        matches!(self_probe, _other_probe) &&
        matches!(self_severity, _other_severity) &&
        self.header == other.header
    }
}

#[cfg(test)]
mod test {

    use super::Message;
    use super::Severity;
    use crate::probes::Probes;

    #[test]
    fn basic_tests() {
        let s1 = Severity::Error;
        let s2 = Severity::Info;
        let s3 = Severity::Warning;

        assert!(matches!(s1, Severity::Error));
        assert!(matches!(s2, Severity::Info));
        assert!(matches!(s3, Severity::Warning));

        let m = Message {
            date: String::from("1970-01-01T00:00:00Z"),
            body: String::from("test"),
            header: String::from("testheader"),
            probe: Probes::Ping,
            severity: Severity::Info,
            service: String::from("testService"),
        };

        let expected_str = String::from("[1970-01-01T00:00:00Z] Info ping/testService: testheader\ntest");

        assert_eq!(m.to_str(), expected_str);

        let m = Message {
            date: String::from("1970-01-01T00:00:00Z"),
            body: String::from("test"),
            header: String::from("testheader"),
            probe: Probes::Ping,
            severity: Severity::Error,
            service: String::from("testService"),
        };

        let expected_str = String::from("[1970-01-01T00:00:00Z] Error ping/testService: testheader\ntest");

        assert_eq!(m.to_str(), expected_str);

        let m = Message {
            date: String::from("1970-01-01T00:00:00Z"),
            body: String::from("test"),
            header: String::from("testheader"),
            probe: Probes::Ping,
            severity: Severity::Error,
            service: String::from("testService"),
        };

        let expected_str = String::from("[1970-01-01T00:00:00Z] Error ping/testService: testheader\ntest");

        assert_eq!(m.to_str(), expected_str);
    }

    #[test]
    fn test_equality() {
        let m = Message {
            date: String::from("1970-01-01T00:00:00Z"),
            body: String::from("test"),
            header: String::from("testheader"),
            probe: Probes::Ping,
            severity: Severity::Error,
            service: String::from("testService"),
        };

        assert!(m == m);
        assert_eq!(m, m);
    }
}
