use chrono::Weekday;

pub fn week_str_to_week_day(week_str: &str) -> Weekday {
    match week_str {
        "Mon" => Weekday::Mon,
        "Tue" => Weekday::Tue,
        "Wed" => Weekday::Wed,
        "Thu" => Weekday::Thu,
        "Fri" => Weekday::Fri,
        "Sat" => Weekday::Sat,
        "Sun" => Weekday::Sun,
        _ => panic!("weekDay can set Mon,Tue,Wed,Thu,Fri,Sat,Sun in conf."),
    }
}
