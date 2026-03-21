extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CommandMapping {
    pub intent_keywords: Vec<String>,
    pub command_template: String,
    pub requires_approval: bool,
    pub risk_level: u8,
}

pub struct ParsedIntent {
    pub raw_input: String,
    pub action: String,
    pub target: Option<String>,
    pub parameters: Vec<String>,
}

pub struct CommandProposal {
    pub description: String,
    pub shell_command: String,
    pub risk_level: u8,
    pub requires_approval: bool,
    pub approved: bool,
}

pub struct NlpBridge {
    pub mappings: Vec<CommandMapping>,
    pub history: Vec<CommandProposal>,
}

impl NlpBridge {
    pub fn new() -> Self {
        let mut bridge = Self {
            mappings: Vec::new(),
            history: Vec::new(),
        };
        bridge.add_mapping(&["commit", "push", "save changes"], "git add -A && git commit -m '{}' && git push", true, 3);
        bridge.add_mapping(&["list", "show", "files", "directory"], "ls -la {}", false, 1);
        bridge.add_mapping(&["delete", "remove", "trash"], "trash {}", true, 7);
        bridge.add_mapping(&["search", "find", "look for"], "grep -r '{}' .", false, 1);
        bridge.add_mapping(&["install", "add package"], "cargo add {}", true, 5);
        bridge.add_mapping(&["build", "compile"], "cargo build --release", true, 3);
        bridge.add_mapping(&["run", "start", "launch"], "./{}", true, 5);
        bridge.add_mapping(&["open", "edit"], "nano {}", false, 1);
        bridge
    }

    pub fn add_mapping(&mut self, keywords: &[&str], template: &str, approval: bool, risk: u8) {
        self.mappings.push(CommandMapping {
            intent_keywords: keywords.iter().map(|k| String::from(*k)).collect(),
            command_template: String::from(template),
            requires_approval: approval,
            risk_level: risk,
        });
    }

    pub fn parse_intent(&self, input: &str) -> ParsedIntent {
        let words: Vec<&str> = input.split(32 as char).collect();
        let action = if !words.is_empty() { String::from(words[0]) } else { String::from("unknown") };
        let target = if words.len() > 1 { Some(String::from(words[1])) } else { None };
        let params = words.iter().skip(2).map(|w| String::from(*w)).collect();
        ParsedIntent {
            raw_input: String::from(input),
            action,
            target,
            parameters: params,
        }
    }

    pub fn translate(&mut self, input: &str) -> Option<CommandProposal> {
        let input_lower = input.to_lowercase();
        for mapping in &self.mappings {
            if mapping.intent_keywords.iter().any(|k| input_lower.contains(k.as_str())) {
                let cmd = mapping.command_template.clone();
                let proposal = CommandProposal {
                    description: String::from(input),
                    shell_command: cmd,
                    risk_level: mapping.risk_level,
                    requires_approval: mapping.requires_approval,
                    approved: false,
                };
                self.history.push(CommandProposal {
                    description: proposal.description.clone(),
                    shell_command: proposal.shell_command.clone(),
                    risk_level: proposal.risk_level,
                    requires_approval: proposal.requires_approval,
                    approved: false,
                };
                return Some(proposal);
            }
        }
        None
    }

    pub fn approve_last(&mut self) -> Option<&CommandProposal> {
        if let Some(last) = self.history.last_mut() {
            last.approved = true;
            Some(last)
        } else {
            None
        }
    }
)}
