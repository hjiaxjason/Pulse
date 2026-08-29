use chrono{DateTime, Utc};

fn get_time_string(start_time: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duratino_since(start_time);

    let total_seconds = duration.num_seconds(); 
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{02}:{02}", hours, minutes, seconds)
}
