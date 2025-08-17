use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Intent {
    FollowUpQuestion,
    Definition,
    ActionItem,
    DraftReply,
    GeneralAssistance,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub transcript_window: String,
    pub speaker_context: Option<String>,
    pub screen_context: Option<String>,
    pub meeting_metadata: Option<MeetingMetadata>,
}

#[derive(Debug, Clone)]
pub struct MeetingMetadata {
    pub participants: Vec<String>,
    pub meeting_type: String,
    pub duration_minutes: u32,
}

#[derive(Debug)]
pub struct RoutingDecision {
    pub intent: Intent,
    pub priority: u8, // 1-10, 10 = highest
    pub context: Context,
    pub suggested_prompt: String,
}

pub struct StateIntentRouter {
    classification_rules: Vec<ClassificationRule>,
}

struct ClassificationRule {
    pattern: String,
    intent: Intent,
    priority: u8,
}

impl StateIntentRouter {
    pub fn new() -> Self {
        let rules = vec![
            ClassificationRule {
                pattern: r"(?i)(what|how|why|when|where)".to_string(),
                intent: Intent::FollowUpQuestion,
                priority: 7,
            },
            ClassificationRule {
                pattern: r"(?i)(define|what is|explain)".to_string(),
                intent: Intent::Definition,
                priority: 8,
            },
            ClassificationRule {
                pattern: r"(?i)(will|should|need to|action|todo)".to_string(),
                intent: Intent::ActionItem,
                priority: 9,
            },
        ];
        
        Self {
            classification_rules: rules,
        }
    }
    
    pub fn classify_intent(&self, transcript: &str) -> Intent {
        for rule in &self.classification_rules {
            if regex::Regex::new(&rule.pattern)
                .unwrap()
                .is_match(transcript) {
                return rule.intent.clone();
            }
        }
        Intent::GeneralAssistance
    }
    
    pub fn build_context(&self, transcript: &str, screen_text: Option<String>) -> Context {
        Context {
            transcript_window: transcript.to_string(),
            speaker_context: None, // TODO: Speaker identification
            screen_context: screen_text,
            meeting_metadata: None, // TODO: Extract from meeting state
        }
    }
    
    pub fn route_request(&self, transcript: &str, screen_context: Option<String>) -> Result<RoutingDecision> {
        let intent = self.classify_intent(transcript);
        let context = self.build_context(transcript, screen_context);
        
        let priority = match intent {
            Intent::ActionItem => 9,
            Intent::Definition => 8,
            Intent::FollowUpQuestion => 7,
            Intent::DraftReply => 6,
            Intent::GeneralAssistance => 5,
        };
        
        let suggested_prompt = self.build_prompt(&intent, &context);
        
        Ok(RoutingDecision {
            intent,
            priority,
            context,
            suggested_prompt,
        })
    }
    
    fn build_prompt(&self, intent: &Intent, context: &Context) -> String {
        let base_context = format!("Recent conversation: {}", context.transcript_window);
        
        match intent {
            Intent::FollowUpQuestion => {
                format!("{}\n\nGenerate a relevant follow-up question to deepen understanding.", base_context)
            },
            Intent::Definition => {
                format!("{}\n\nProvide a concise definition or explanation of the key concept mentioned.", base_context)
            },
            Intent::ActionItem => {
                format!("{}\n\nExtract and format any action items or next steps mentioned.", base_context)
            },
            Intent::DraftReply => {
                format!("{}\n\nSuggest a professional response or reply.", base_context)
            },
            Intent::GeneralAssistance => {
                format!("{}\n\nProvide helpful context or suggestions.", base_context)
            },
        }
    }
}
