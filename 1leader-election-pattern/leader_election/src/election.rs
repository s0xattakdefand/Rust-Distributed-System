#[derive(Clone, Debug)]
pub struct LeaderElection {
    pub node_id: u32,
    pub leader_id: Option<u32>,
}

impl LeaderElection {
    pub fn new(node_id: u32) -> Self {
        Self {
            node_id,
            leader_id: None,
        }
    }

    pub async fn start(&mut self) {
        loop {
            if self.should_become_leader() {
                println!("[{}] Becoming leader!", self.node_id);
                self.leader_id = Some(self.node_id);
            } else {
                println!("[{}] Not the best candidate right now...", self.node_id);
            }

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    fn should_become_leader(&self) -> bool {
        let simulated_ids = vec![101, 102, 103];
        let lowest = *simulated_ids.iter().min().unwrap();
        self.node_id == lowest
    }

    pub fn get_leader(&self) -> Option<u32> {
        self.leader_id
    }
}
