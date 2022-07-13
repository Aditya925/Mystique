use serde_json;
use serde_json::Value;
use config::Config;
use files;
use system;

pub fn decode(input: &str, config: &Config) -> String {
    let data: Value = serde_json::from_str(input).unwrap();

    if data["action"] == "requestFilelist" {
        let mut path_end = data["path"].to_string();
        path_end.remove(0);
        path_end.pop();

        files::get_file_respond(&path_end, config)
    } else if data["action"] == "requestNewFolder" {
        let mut path_end = data["path"].to_string();
        path_end.remove(0);
        path_end.pop();

        files::get_new_folder_respond(&path_end, &config)
    } else if data["action"] == "requestUptime" {
        system::get_uptime_respond(&config.start_time)
    } else {
        json!({
            "action": "sendError",
            "message": format!("Unknown action from client: {}", data["action"]) 
        }).to_string()
    }
}