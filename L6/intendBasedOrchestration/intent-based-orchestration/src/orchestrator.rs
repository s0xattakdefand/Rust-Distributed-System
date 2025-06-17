use crate::actions::*;

use crate::intent::Intent;

pub fn orchestrate(intent: &Intent) -> Vec<String> {
    match intent.goal.as_str() {
        "deploy_web_app" => deploy_web_app(intent.parameters.as_ref()),
        "scale_service" => scale_service(intent.parameters.as_ref()),
        _ => unknown_intent(&intent.goal),
    }
}
