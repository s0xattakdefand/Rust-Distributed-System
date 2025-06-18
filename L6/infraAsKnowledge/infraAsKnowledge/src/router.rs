use crate::knowledge::KnowledgeBase;

const ALL_REGIONS: &[&str] = &["asia", "us", "europe"];

pub fn choose_region(kb: &KnowledgeBase) -> String {
    // Try to pick a region not avoided
    for region in ALL_REGIONS {
        if !kb.is_region_avoided(region) {
            return region.to_string();
        }
    }
    // Fallback: pick the "least bad" (expired incident)
    "none-available".to_string()
}
