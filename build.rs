const COMMANDS: &[&str] = &[
    "get_fd",
    "parse_network_config",
    "stop_network_instance",
    "collect_network_infos",
    "start_network_instance",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
