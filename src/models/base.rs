use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    agents: i32,
    ships: i32,
    systems: i32,
    waypoints: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MostCredits {
    agent_symbol: String,
    credits: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MostSubmittedCharts {
    agent_symbol: String,
    chart_count: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Leaderboard {
    most_credits: Vec<MostCredits>,
    most_submitted_charts: Vec<MostSubmittedCharts>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Base {
    status: String,
    stats: Stats,
    leaderboards: Leaderboard
}
