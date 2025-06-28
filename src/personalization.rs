use crate::utils::load_profile;
use dirs::home_dir;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;

/// Enhanced user context for more personalized suggestions
#[derive(Debug, Clone)]
pub struct UserContext {
    pub basic_profile: Value,
    pub preferences: UserPreferences,
    pub activity_patterns: ActivityPatterns,
    pub learning_style: LearningStyle,
    pub current_projects: Vec<ProjectContext>,
    pub goals: Vec<Goal>,
    pub interaction_history: InteractionHistory,
}

#[derive(Debug, Clone)]
pub struct UserPreferences {
    pub suggestion_types: Vec<String>, // learning, productivity, collaboration, wellness
    pub communication_style: String,   // concise, detailed, casual, formal
    pub frequency: String,             // daily, weekly, as-needed
    pub time_of_day: Vec<String>,      // morning, afternoon, evening
    pub focus_areas: Vec<String>,      // technical skills, soft skills, career growth
}

#[derive(Debug, Clone)]
pub struct ActivityPatterns {
    pub most_active_times: Vec<String>,
    pub note_taking_frequency: String,
    pub common_topics: Vec<String>,
    pub collaboration_frequency: String,
    pub learning_pace: String, // fast, moderate, slow
}

#[derive(Debug, Clone)]
pub struct LearningStyle {
    pub preferred_format: String, // hands-on, reading, videos, peer-learning
    pub complexity_preference: String, // beginner, intermediate, advanced
    pub feedback_preference: String, // immediate, periodic, milestone-based
}

#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub name: String,
    pub tech_stack: Vec<String>,
    pub current_challenges: Vec<String>,
    pub team_size: u32,
    pub deadline_pressure: String, // low, medium, high
}

#[derive(Debug, Clone)]
pub struct Goal {
    pub description: String,
    pub category: String, // career, skill, project, personal
    pub timeline: String, // short-term, medium-term, long-term
    pub progress: f32,    // 0.0 to 1.0
}

#[derive(Debug, Clone)]
pub struct InteractionHistory {
    pub suggestion_acceptance_rate: f32,
    pub most_engaged_categories: Vec<String>,
    pub recent_topics: Vec<String>,
    pub feedback_patterns: HashMap<String, f32>, // topic -> satisfaction score
}

#[derive(Debug, Clone)]
pub struct QueryIntent {
    pub intent_type: String,
    pub context_type: String,
    pub urgency_level: String,
}

impl UserContext {
    /// Load existing user context or create a basic one from profile
    pub fn load_or_create() -> Result<Self, String> {
        let profile = load_profile()?;

        // Try to load enhanced context
        if let Ok(enhanced_context) = Self::load_enhanced_context() {
            return Ok(enhanced_context);
        }

        // Create basic context from existing profile
        Ok(Self {
            basic_profile: profile.clone(),
            preferences: UserPreferences::from_profile(&profile),
            activity_patterns: ActivityPatterns::default(),
            learning_style: LearningStyle::from_profile(&profile),
            current_projects: vec![],
            goals: vec![],
            interaction_history: InteractionHistory::default(),
        })
    }

    /// Load enhanced context from file
    fn load_enhanced_context() -> Result<Self, String> {
        let mut path = home_dir().ok_or("Could not determine home directory")?;
        path.push(".logswise/enhanced_context.json");

        let data = fs::read_to_string(&path).map_err(|_| "Enhanced context not found")?;

        let context_data: Value =
            serde_json::from_str(&data).map_err(|_| "Failed to parse enhanced context")?;

        // Parse the JSON back into UserContext
        // This is simplified - in a real implementation you'd want proper deserialization
        let basic_profile = load_profile()?;

        Ok(Self {
            basic_profile,
            preferences: UserPreferences::from_json(&context_data["preferences"]),
            activity_patterns: ActivityPatterns::from_json(&context_data["activity_patterns"]),
            learning_style: LearningStyle::from_json(&context_data["learning_style"]),
            current_projects: ProjectContext::vec_from_json(&context_data["current_projects"]),
            goals: Goal::vec_from_json(&context_data["goals"]),
            interaction_history: InteractionHistory::from_json(
                &context_data["interaction_history"],
            ),
        })
    }

    /// Save enhanced context to file
    pub fn save(&self) -> Result<(), String> {
        let mut path = home_dir().ok_or("Could not determine home directory")?;
        path.push(".logswise");
        fs::create_dir_all(&path).map_err(|e| format!("Failed to create directory: {e}"))?;
        path.push("enhanced_context.json");

        let context_data = json!({
            "preferences": self.preferences.to_json(),
            "activity_patterns": self.activity_patterns.to_json(),
            "learning_style": self.learning_style.to_json(),
            "current_projects": self.current_projects.iter().map(|p| p.to_json()).collect::<Vec<_>>(),
            "goals": self.goals.iter().map(|g| g.to_json()).collect::<Vec<_>>(),
            "interaction_history": self.interaction_history.to_json(),
        });

        fs::write(&path, serde_json::to_string_pretty(&context_data).unwrap())
            .map_err(|e| format!("Failed to save enhanced context: {e}"))?;

        Ok(())
    }

    /// Generate rich prompt context for LLM using advanced contextualization
    pub fn generate_llm_context(&self, query: &str, relevant_notes: &[String]) -> String {
        let mut context = String::new();

        // Advanced system prompt with chain-of-thought reasoning
        context.push_str("=== SYSTEM INSTRUCTIONS ===\n");
        context.push_str(
            "You are an AI assistant specializing in personalized professional development.\n",
        );
        context.push_str(
            "Your role: Provide contextual, actionable advice based on verified user data.\n\n",
        );

        context.push_str("REASONING FRAMEWORK:\n");
        context.push_str("1. ANALYZE: Understand the intent behind the query\n");
        context.push_str("2. CONTEXTUALIZE: Map query to relevant user projects/goals\n");
        context.push_str("3. SYNTHESIZE: Generate specific, actionable suggestions\n");
        context.push_str("4. VALIDATE: Ensure all advice is grounded in verified data\n\n");

        context.push_str("CRITICAL CONSTRAINTS:\n");
        context.push_str("- NEVER invent relationships, conversations, or commitments\n");
        context.push_str("- Focus on SITUATION and TOPIC, not on people mentioned\n");
        context.push_str(
            "- Use conversation context as INPUT TYPE (meeting prep, progress update, etc.)\n",
        );
        context
            .push_str("- Ground ALL suggestions in documented projects, challenges, and goals\n");
        context.push_str("- Provide MEASURABLE and TIME-BOUND recommendations when possible\n\n");

        // Enhanced user profile with contextual signals
        context.push_str(&format!(
            "=== USER CONTEXT PROFILE ===\n\
            🎯 PROFESSIONAL IDENTITY:\n\
            Role: {} | Experience Level: {} | Domain: {}\n\
            Organization: {} ({} employees) | Work Style: {}\n\
            Tech Stack: {} | Career Stage: Aiming to {}\n\n\
            💡 COGNITIVE PREFERENCES:\n\
            Communication: {} | Learning: {} (complexity: {})\n\
            Decision Making: {} feedback loops | Active Times: {}\n\
            Learning Velocity: {} | Focus Areas: {}\n\n",
            self.basic_profile["profession"]
                .as_str()
                .unwrap_or("Professional"),
            self.basic_profile["yearsExperience"]
                .as_str()
                .unwrap_or("Not specified"),
            self.basic_profile["preferredLanguage"]
                .as_str()
                .unwrap_or("Multi-tech"),
            self.basic_profile["companyName"]
                .as_str()
                .unwrap_or("Current Organization"),
            self.basic_profile["companySize"]
                .as_str()
                .unwrap_or("Unknown size"),
            self.basic_profile["workMode"]
                .as_str()
                .unwrap_or("Flexible"),
            self.basic_profile["preferredLanguage"]
                .as_str()
                .unwrap_or("Various"),
            self.goals
                .first()
                .map(|g| g.description.as_str())
                .unwrap_or("grow professionally"),
            self.preferences.communication_style,
            self.learning_style.preferred_format,
            self.learning_style.complexity_preference,
            self.learning_style.feedback_preference,
            self.activity_patterns.most_active_times.join(", "),
            self.activity_patterns.learning_pace,
            self.preferences.focus_areas.join(" + ")
        ));

        // Strategic project portfolio analysis
        if !self.current_projects.is_empty() {
            context.push_str("=== ACTIVE PROJECT PORTFOLIO ===\n");
            for (i, project) in self.current_projects.iter().enumerate() {
                let priority = match project.deadline_pressure.as_str() {
                    "high" => "🔥 HIGH PRIORITY",
                    "medium" => "⚡ MODERATE PRIORITY",
                    _ => "📋 STANDARD PRIORITY",
                };

                let team_context = match project.team_size {
                    1 => "Solo Project",
                    2..=5 => "Small Team",
                    6..=15 => "Medium Team",
                    _ => "Large Team",
                };

                context.push_str(&format!(
                    "🚀 PROJECT {}: {} [{}]\n\
                    ├─ Tech Stack: {}\n\
                    ├─ Team Context: {} ({} people)\n\
                    ├─ Key Challenges: {}\n\
                    └─ Strategic Focus: {}\n\n",
                    i + 1,
                    project.name,
                    priority,
                    project.tech_stack.join(" + "),
                    team_context,
                    project.team_size,
                    project.current_challenges.join(" | "),
                    if project.deadline_pressure == "high" {
                        "Delivery & Quality"
                    } else {
                        "Innovation & Growth"
                    }
                ));
            }
        } else {
            context.push_str("=== PROJECT PORTFOLIO ===\n");
            context.push_str("📝 No specific projects documented. Opportunity to capture current work context.\n\n");
        }

        // Goal-oriented development roadmap
        if !self.goals.is_empty() {
            context.push_str("=== DEVELOPMENT ROADMAP ===\n");
            for goal in &self.goals {
                let progress_bar = "▓".repeat((goal.progress * 10.0) as usize)
                    + &"░".repeat(10 - (goal.progress * 10.0) as usize);
                let timeline_icon = match goal.timeline.as_str() {
                    "short-term" => "⚡",
                    "medium-term" => "🎯",
                    _ => "🌟",
                };

                context.push_str(&format!(
                    "{} {} | {} | [{}] {}%\n\
                    └─ Domain: {} | Target: {}\n",
                    timeline_icon,
                    goal.description.trim(),
                    goal.timeline.replace('-', " ").to_uppercase(),
                    progress_bar,
                    (goal.progress * 100.0) as u32,
                    goal.category.replace('_', " ").to_uppercase(),
                    goal.timeline
                ));
            }
            context.push('\n');
        }

        // Interaction patterns (excluding recent topics to prevent hallucination)
        context.push_str(&format!(
            "=== INTERACTION PATTERNS ===\n\
            Suggestion Acceptance Rate: {:.1}%\n\
            Most Engaged Categories: {}\n\
            Activity Level: {} suggestion interactions\n\n",
            self.interaction_history.suggestion_acceptance_rate * 100.0,
            self.interaction_history.most_engaged_categories.join(", "),
            self.interaction_history.recent_topics.len()
        ));

        // Relevant notes
        if !relevant_notes.is_empty() {
            context.push_str("=== RELEVANT NOTES ===\n");
            for (i, note) in relevant_notes.iter().enumerate() {
                context.push_str(&format!("{}. {}\n", i + 1, note));
            }
            context.push('\n');
        }

        // Advanced query contextualization
        let query_intent = self.analyze_query_intent(query);
        context.push_str("=== QUERY ANALYSIS ===\n");
        context.push_str(&format!("📋 Request: \"{query}\"\n"));
        context.push_str(&format!("🎯 Intent: {}\n", query_intent.intent_type));
        context.push_str(&format!("📊 Context: {}\n", query_intent.context_type));
        context.push_str(&format!("⚡ Priority: {}\n\n", query_intent.urgency_level));

        // Relevant notes with contextual ranking
        if !relevant_notes.is_empty() {
            context.push_str("=== CONTEXTUAL KNOWLEDGE BASE ===\n");
            for (i, note) in relevant_notes.iter().enumerate() {
                let relevance = if i < 2 {
                    "🎯 HIGH"
                } else if i < 4 {
                    "📋 MEDIUM"
                } else {
                    "📝 LOW"
                };
                context.push_str(&format!("{}. [{}] {}\n", i + 1, relevance, note));
            }
            context.push('\n');
        }

        context.push_str("=== RESPONSE FRAMEWORK ===\n");
        context.push_str("Structure your response using this advanced framework:\n\n");

        match query_intent.intent_type.as_str() {
            "meeting_preparation" => {
                context.push_str("📋 MEETING PREP FRAMEWORK:\n");
                context.push_str("1. 🎯 AGENDA PREPARATION: Key topics to address\n");
                context.push_str("2. 📊 PROGRESS SUMMARY: Quantified achievements\n");
                context.push_str("3. 🚧 CHALLENGE ANALYSIS: Issues and proposed solutions\n");
                context.push_str("4. 🎪 NEXT STEPS: Specific, measurable actions\n");
            }
            "progress_reporting" => {
                context.push_str("📈 PROGRESS REPORT FRAMEWORK:\n");
                context.push_str("1. ✅ ACCOMPLISHMENTS: What was delivered\n");
                context.push_str("2. 📊 METRICS: Quantifiable progress indicators\n");
                context.push_str("3. 🔄 PROCESS IMPROVEMENTS: How efficiency was enhanced\n");
                context.push_str("4. 🎯 UPCOMING MILESTONES: What's next with timelines\n");
            }
            "problem_solving" => {
                context.push_str("🔧 PROBLEM-SOLVING FRAMEWORK:\n");
                context.push_str("1. 🔍 ROOT CAUSE: Core issue identification\n");
                context.push_str("2. 💡 SOLUTION OPTIONS: Alternative approaches\n");
                context.push_str("3. ⚖️ TRADE-OFF ANALYSIS: Pros/cons of each option\n");
                context.push_str("4. 🎯 RECOMMENDATION: Best path forward with rationale\n");
            }
            _ => {
                context.push_str("🎯 GENERAL FRAMEWORK:\n");
                context.push_str("1. 📊 SITUATION ANALYSIS: Current state assessment\n");
                context.push_str("2. 💡 STRATEGIC OPTIONS: Available approaches\n");
                context.push_str("3. 🎯 ACTIONABLE RECOMMENDATIONS: Specific next steps\n");
                context.push_str("4. 📈 SUCCESS METRICS: How to measure progress\n");
            }
        }

        context.push_str("\n💼 PROFESSIONAL EXCELLENCE STANDARDS:\n");
        context.push_str("- Provide SPECIFIC, time-bound recommendations\n");
        context.push_str("- Include SUCCESS METRICS for each suggestion\n");
        context.push_str("- Consider STAKEHOLDER IMPACT and team dynamics\n");
        context.push_str("- Balance SHORT-TERM wins with LONG-TERM strategy\n");
        context.push_str("- Suggest FOLLOW-UP mechanisms for accountability\n\n");

        context
    }

    /// Update interaction history based on user feedback
    pub fn update_interaction_history(
        &mut self,
        category: &str,
        accepted: bool,
        satisfaction: f32,
    ) {
        // Update acceptance rate
        let current_rate = self.interaction_history.suggestion_acceptance_rate;
        self.interaction_history.suggestion_acceptance_rate =
            (current_rate * 0.9) + (if accepted { 0.1 } else { 0.0 });

        // Update feedback patterns
        self.interaction_history
            .feedback_patterns
            .entry(category.to_string())
            .and_modify(|score| *score = (*score * 0.8) + (satisfaction * 0.2))
            .or_insert(satisfaction);

        // Update engaged categories
        if satisfaction > 0.7
            && !self
                .interaction_history
                .most_engaged_categories
                .contains(&category.to_string())
        {
            self.interaction_history
                .most_engaged_categories
                .push(category.to_string());
        }
    }

    /// Add a new goal
    pub fn add_goal(&mut self, description: String, category: String, timeline: String) {
        self.goals.push(Goal {
            description,
            category,
            timeline,
            progress: 0.0,
        });
    }

    /// Add current project context
    pub fn add_project(&mut self, project: ProjectContext) {
        self.current_projects.push(project);
    }

    /// Advanced query intent analysis for contextualized responses
    fn analyze_query_intent(&self, query: &str) -> QueryIntent {
        let query_lower = query.to_lowercase();

        let intent_type = if query_lower.contains("meeting")
            || query_lower.contains("one-on-one")
            || query_lower.contains("discuss")
            || query_lower.contains("prepare")
        {
            "meeting_preparation"
        } else if query_lower.contains("progress")
            || query_lower.contains("update")
            || query_lower.contains("accomplished")
            || query_lower.contains("completed")
        {
            "progress_reporting"
        } else if query_lower.contains("problem")
            || query_lower.contains("issue")
            || query_lower.contains("challenge")
            || query_lower.contains("stuck")
        {
            "problem_solving"
        } else if query_lower.contains("learn")
            || query_lower.contains("skill")
            || query_lower.contains("improve")
            || query_lower.contains("develop")
        {
            "skill_development"
        } else if query_lower.contains("career")
            || query_lower.contains("promotion")
            || query_lower.contains("senior")
            || query_lower.contains("advancement")
        {
            "career_growth"
        } else {
            "general_advice"
        };

        let context_type = if query_lower.contains("team") || query_lower.contains("colleague") {
            "team_collaboration"
        } else if query_lower.contains("project")
            || query_lower.contains("vena")
            || query_lower.contains("logswise")
        {
            "project_specific"
        } else if query_lower.contains("technical")
            || query_lower.contains("code")
            || query_lower.contains("development")
        {
            "technical_focus"
        } else {
            "general_professional"
        };

        let urgency_level = if query_lower.contains("tomorrow")
            || query_lower.contains("urgent")
            || query_lower.contains("asap")
            || query_lower.contains("immediately")
        {
            "high"
        } else if query_lower.contains("soon") || query_lower.contains("this week") {
            "medium"
        } else {
            "normal"
        };

        QueryIntent {
            intent_type: intent_type.to_string(),
            context_type: context_type.to_string(),
            urgency_level: urgency_level.to_string(),
        }
    }
}

impl UserPreferences {
    fn from_profile(profile: &Value) -> Self {
        // Infer preferences from basic profile
        let profession = profile["profession"].as_str().unwrap_or("").to_lowercase();
        let _experience = profile["yearsExperience"].as_str().unwrap_or("");

        let communication_style = if profession.contains("manager") || profession.contains("lead") {
            "detailed".to_string()
        } else if profession.contains("developer") {
            "concise".to_string()
        } else {
            "balanced".to_string()
        };

        let focus_areas = if profession.contains("developer") {
            vec!["technical skills".to_string(), "code quality".to_string()]
        } else if profession.contains("manager") {
            vec!["leadership".to_string(), "team collaboration".to_string()]
        } else {
            vec!["professional growth".to_string()]
        };

        Self {
            suggestion_types: vec!["learning".to_string(), "productivity".to_string()],
            communication_style,
            frequency: "weekly".to_string(),
            time_of_day: vec!["morning".to_string()],
            focus_areas,
        }
    }

    fn from_json(json: &Value) -> Self {
        Self {
            suggestion_types: json["suggestion_types"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            communication_style: json["communication_style"]
                .as_str()
                .unwrap_or("balanced")
                .to_string(),
            frequency: json["frequency"].as_str().unwrap_or("weekly").to_string(),
            time_of_day: json["time_of_day"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            focus_areas: json["focus_areas"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    fn to_json(&self) -> Value {
        json!({
            "suggestion_types": self.suggestion_types,
            "communication_style": self.communication_style,
            "frequency": self.frequency,
            "time_of_day": self.time_of_day,
            "focus_areas": self.focus_areas
        })
    }
}

impl Default for ActivityPatterns {
    fn default() -> Self {
        Self {
            most_active_times: vec!["morning".to_string()],
            note_taking_frequency: "moderate".to_string(),
            common_topics: vec![],
            collaboration_frequency: "moderate".to_string(),
            learning_pace: "moderate".to_string(),
        }
    }
}

impl ActivityPatterns {
    fn from_json(json: &Value) -> Self {
        Self {
            most_active_times: json["most_active_times"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            note_taking_frequency: json["note_taking_frequency"]
                .as_str()
                .unwrap_or("moderate")
                .to_string(),
            common_topics: json["common_topics"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            collaboration_frequency: json["collaboration_frequency"]
                .as_str()
                .unwrap_or("moderate")
                .to_string(),
            learning_pace: json["learning_pace"]
                .as_str()
                .unwrap_or("moderate")
                .to_string(),
        }
    }

    fn to_json(&self) -> Value {
        json!({
            "most_active_times": self.most_active_times,
            "note_taking_frequency": self.note_taking_frequency,
            "common_topics": self.common_topics,
            "collaboration_frequency": self.collaboration_frequency,
            "learning_pace": self.learning_pace
        })
    }
}

impl LearningStyle {
    fn from_profile(profile: &Value) -> Self {
        let profession = profile["profession"].as_str().unwrap_or("").to_lowercase();
        let experience = profile["yearsExperience"].as_str().unwrap_or("");

        let preferred_format = if profession.contains("developer") {
            "hands-on".to_string()
        } else {
            "reading".to_string()
        };

        let complexity_preference = if experience.contains("Senior") || experience.contains("Lead")
        {
            "advanced".to_string()
        } else if experience.contains("Junior") {
            "beginner".to_string()
        } else {
            "intermediate".to_string()
        };

        Self {
            preferred_format,
            complexity_preference,
            feedback_preference: "periodic".to_string(),
        }
    }

    fn from_json(json: &Value) -> Self {
        Self {
            preferred_format: json["preferred_format"]
                .as_str()
                .unwrap_or("hands-on")
                .to_string(),
            complexity_preference: json["complexity_preference"]
                .as_str()
                .unwrap_or("intermediate")
                .to_string(),
            feedback_preference: json["feedback_preference"]
                .as_str()
                .unwrap_or("periodic")
                .to_string(),
        }
    }

    fn to_json(&self) -> Value {
        json!({
            "preferred_format": self.preferred_format,
            "complexity_preference": self.complexity_preference,
            "feedback_preference": self.feedback_preference
        })
    }
}

impl ProjectContext {
    fn vec_from_json(json: &Value) -> Vec<Self> {
        json.as_array()
            .map(|arr| arr.iter().filter_map(Self::from_json).collect())
            .unwrap_or_default()
    }

    fn from_json(json: &Value) -> Option<Self> {
        Some(Self {
            name: json["name"].as_str()?.to_string(),
            tech_stack: json["tech_stack"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            current_challenges: json["current_challenges"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            team_size: json["team_size"].as_u64().unwrap_or(1) as u32,
            deadline_pressure: json["deadline_pressure"]
                .as_str()
                .unwrap_or("medium")
                .to_string(),
        })
    }

    fn to_json(&self) -> Value {
        json!({
            "name": self.name,
            "tech_stack": self.tech_stack,
            "current_challenges": self.current_challenges,
            "team_size": self.team_size,
            "deadline_pressure": self.deadline_pressure
        })
    }
}

impl Goal {
    fn vec_from_json(json: &Value) -> Vec<Self> {
        json.as_array()
            .map(|arr| arr.iter().filter_map(Self::from_json).collect())
            .unwrap_or_default()
    }

    fn from_json(json: &Value) -> Option<Self> {
        Some(Self {
            description: json["description"].as_str()?.to_string(),
            category: json["category"].as_str().unwrap_or("personal").to_string(),
            timeline: json["timeline"]
                .as_str()
                .unwrap_or("medium-term")
                .to_string(),
            progress: json["progress"].as_f64().unwrap_or(0.0) as f32,
        })
    }

    fn to_json(&self) -> Value {
        json!({
            "description": self.description,
            "category": self.category,
            "timeline": self.timeline,
            "progress": self.progress
        })
    }
}

impl Default for InteractionHistory {
    fn default() -> Self {
        Self {
            suggestion_acceptance_rate: 0.0,
            most_engaged_categories: vec![],
            recent_topics: vec![],
            feedback_patterns: HashMap::new(),
        }
    }
}

impl InteractionHistory {
    fn from_json(json: &Value) -> Self {
        Self {
            suggestion_acceptance_rate: json["suggestion_acceptance_rate"].as_f64().unwrap_or(0.0)
                as f32,
            most_engaged_categories: json["most_engaged_categories"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            recent_topics: json["recent_topics"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            feedback_patterns: json["feedback_patterns"]
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f as f32)))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    fn to_json(&self) -> Value {
        json!({
            "suggestion_acceptance_rate": self.suggestion_acceptance_rate,
            "most_engaged_categories": self.most_engaged_categories,
            "recent_topics": self.recent_topics,
            "feedback_patterns": self.feedback_patterns
        })
    }
}
