use serde_json::json;
use ttrpg_types::{TtrpgItem, ItemType};
use crate::generated::journal::FoundryJournalPage;
use std::collections::HashMap;

impl From<FoundryJournalPage> for TtrpgItem {
    fn from(journal: FoundryJournalPage) -> Self {
        let mut system_data = HashMap::new();

        // Extract basic information
        let id = journal.id.to_string();
        let name = journal.name.to_string();
        let image = journal.image.map(|img| format!("{:?}", img));

        // Extract description from text content and store journal-specific data
        let description = if let Some(ref text) = journal.text {
            system_data.insert("text_format".to_string(), json!(text.format));
            if let Some(ref markdown) = text.markdown {
                system_data.insert("markdown".to_string(), json!(markdown));
            }
            text.content.clone()
        } else {
            None
        };

        // Store title if present
        if let Some(title) = journal.title {
            system_data.insert("title".to_string(), json!(title));
        }
        if let Some(video) = journal.video {
            system_data.insert("video".to_string(), json!(video));
        }
        system_data.insert("ownership".to_string(), json!(journal.ownership));
        system_data.insert("flags".to_string(), json!(journal.flags));

        TtrpgItem {
            id,
            name,
            description,
            item_type: ItemType::Equipment, // Treat as document
            quantity: 1,
            weight: None,
            cost: None,
            rarity: None,
            image,
            damage: None,
            armor_class: None,
            requires_attunement: Some(false),
            system_data,
        }
    }
}
