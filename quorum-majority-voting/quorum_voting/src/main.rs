use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Vote {
    voter_id: String,
    choice: String,
}

type RoundVotes = HashMap<String, Vec<Vote>>;
type SharedVotes = Arc<Mutex<RoundVotes>>;

#[tokio::main]
async fn main() {
    let votes: SharedVotes = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/vote/:round", post(cast_vote))
        .route("/result/:round", get(get_result))
        .with_state(votes);

    let addr: SocketAddr = "127.0.0.1:4000".parse().unwrap();
    println!("🗳️ Quorum Voting API running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn cast_vote(
    Path(round): Path<String>,
    State(votes): State<SharedVotes>,
    Json(vote): Json<Vote>,
) -> Json<String> {
    let mut data = votes.lock().unwrap();
    let round_votes = data.entry(round.clone()).or_insert_with(Vec::new);

    // Prevent duplicate voter in same round
    if round_votes.iter().any(|v| v.voter_id == vote.voter_id) {
        return Json(format!("❌ Voter '{}' has already voted in round '{}'", vote.voter_id, round));
    }

    round_votes.push(vote.clone());
    Json(format!("✅ Vote by '{}' for '{}' in round '{}'", vote.voter_id, vote.choice, round))
}

async fn get_result(Path(round): Path<String>, State(votes): State<SharedVotes>) -> Json<String> {
    let data = votes.lock().unwrap();
    if let Some(round_votes) = data.get(&round) {
        let mut tally: HashMap<String, usize> = HashMap::new();
        for vote in round_votes {
            *tally.entry(vote.choice.clone()).or_insert(0) += 1;
        }

        if tally.is_empty() {
            return Json(format!("🤷 No votes found for round '{}'", round));
        }

        let (winner, count) = tally.into_iter().max_by_key(|(_, c)| *c).unwrap();
        Json(format!("🏆 Winner for round '{}': {} ({} votes)", round, winner, count))
    } else {
        Json(format!("🚫 No votes yet for round '{}'", round))
    }
}
