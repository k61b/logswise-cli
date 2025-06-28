use crate::personalization::{ProjectContext, UserContext};
use colored::*;
use dialoguer::{Confirm, FuzzySelect, Input, MultiSelect, Select};

/// Enhanced setup process to collect detailed user preferences and context
pub fn run_enhanced_setup() -> Result<(), String> {
    println!("{}", "🚀 Enhanced Personalization Setup".cyan().bold());
    println!("This will help us provide much more personalized suggestions!\n");

    let mut user_context = UserContext::load_or_create()?;

    // Communication and Learning Preferences
    setup_communication_preferences(&mut user_context)?;

    // Learning Style Assessment
    setup_learning_style(&mut user_context)?;

    // Current Projects and Tech Stack
    setup_current_projects(&mut user_context)?;

    // Goals and Aspirations
    setup_goals(&mut user_context)?;

    // Activity Patterns and Preferences
    setup_activity_patterns(&mut user_context)?;

    // Save enhanced context
    user_context.save()?;

    println!(
        "{}",
        "✨ Enhanced setup complete! Your suggestions will now be much more personalized."
            .green()
            .bold()
    );

    Ok(())
}

fn setup_communication_preferences(context: &mut UserContext) -> Result<(), String> {
    println!("{}", "📝 Communication Preferences".yellow().bold());

    let style_options = vec![
        "Concise (bullet points, quick wins)",
        "Detailed (comprehensive explanations)",
        "Casual (friendly, informal tone)",
        "Professional (formal, structured)",
        "Balanced (mix of above)",
    ];

    let style_idx = Select::new()
        .with_prompt("How do you prefer to receive suggestions?")
        .items(&style_options)
        .default(4)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    context.preferences.communication_style = match style_idx {
        0 => "concise",
        1 => "detailed",
        2 => "casual",
        3 => "professional",
        _ => "balanced",
    }
    .to_string();

    let frequency_options = vec![
        "Daily",
        "Few times a week",
        "Weekly",
        "Bi-weekly",
        "As-needed",
    ];
    let freq_idx = Select::new()
        .with_prompt("How often would you like personalized suggestions?")
        .items(&frequency_options)
        .default(2)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    context.preferences.frequency = match freq_idx {
        0 => "daily",
        1 => "few_times_week",
        2 => "weekly",
        3 => "bi_weekly",
        _ => "as_needed",
    }
    .to_string();

    let focus_options = vec![
        "Technical Skills",
        "Soft Skills",
        "Leadership",
        "Career Growth",
        "Work-Life Balance",
        "Team Collaboration",
        "Innovation & Creativity",
        "Productivity & Efficiency",
    ];

    let focus_indices = MultiSelect::new()
        .with_prompt("What areas would you like suggestions to focus on? (Select multiple)")
        .items(&focus_options)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    context.preferences.focus_areas = focus_indices
        .iter()
        .map(|&i| focus_options[i].to_lowercase().replace(' ', "_"))
        .collect();

    println!();
    Ok(())
}

fn setup_learning_style(context: &mut UserContext) -> Result<(), String> {
    println!("{}", "🎓 Learning Style Assessment".yellow().bold());

    let format_options = vec![
        "Hands-on practice (learning by doing)",
        "Reading documentation and articles",
        "Video tutorials and demos",
        "Peer learning and discussions",
        "Structured courses",
        "Experimentation and exploration",
    ];

    let format_idx = Select::new()
        .with_prompt("How do you prefer to learn new things?")
        .items(&format_options)
        .default(0)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    context.learning_style.preferred_format = match format_idx {
        0 => "hands_on",
        1 => "reading",
        2 => "videos",
        3 => "peer_learning",
        4 => "structured",
        _ => "experimentation",
    }
    .to_string();

    let complexity_options = vec![
        "Beginner-friendly (step-by-step guidance)",
        "Intermediate (some background assumed)",
        "Advanced (deep technical details)",
        "Mixed (adapt based on topic)",
    ];

    let complexity_idx = Select::new()
        .with_prompt("What complexity level do you prefer for suggestions?")
        .items(&complexity_options)
        .default(3)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    context.learning_style.complexity_preference = match complexity_idx {
        0 => "beginner",
        1 => "intermediate",
        2 => "advanced",
        _ => "adaptive",
    }
    .to_string();

    let feedback_options = vec![
        "Immediate (quick wins and fast feedback)",
        "Periodic (weekly/monthly check-ins)",
        "Milestone-based (after completing goals)",
    ];

    let feedback_idx = Select::new()
        .with_prompt("How do you prefer to track progress?")
        .items(&feedback_options)
        .default(1)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    context.learning_style.feedback_preference = match feedback_idx {
        0 => "immediate",
        1 => "periodic",
        _ => "milestone",
    }
    .to_string();

    println!();
    Ok(())
}

fn setup_current_projects(context: &mut UserContext) -> Result<(), String> {
    println!("{}", "🛠️  Current Projects & Tech Stack".yellow().bold());

    let add_projects = Confirm::new()
        .with_prompt("Would you like to add information about your current projects?")
        .default(true)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    if !add_projects {
        println!();
        return Ok(());
    }

    let num_projects = Select::new()
        .with_prompt("How many current projects would you like to add?")
        .items(&["1", "2", "3", "4", "5+"])
        .default(0)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    let project_count = std::cmp::min(num_projects + 1, 5);

    for i in 0..project_count {
        println!("{}", format!("Project {} Details:", i + 1).cyan());

        let name: String = Input::new()
            .with_prompt("Project name")
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let tech_stack_options = vec![
            "Rust",
            "Python",
            "JavaScript/TypeScript",
            "Go",
            "Java",
            "C#",
            "C/C++",
            "React",
            "Vue",
            "Angular",
            "Node.js",
            "Docker",
            "Kubernetes",
            "AWS",
            "GCP",
            "Azure",
            "PostgreSQL",
            "MongoDB",
            "Redis",
            "Other",
        ];

        let tech_indices = MultiSelect::new()
            .with_prompt("Tech stack (select all that apply)")
            .items(&tech_stack_options)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        let tech_stack: Vec<String> = tech_indices
            .iter()
            .map(|&i| tech_stack_options[i].to_string())
            .collect();

        let team_size_options = vec!["1 (solo)", "2-3", "4-6", "7-10", "11-20", "20+"];
        let team_idx = Select::new()
            .with_prompt("Team size")
            .items(&team_size_options)
            .default(1)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        let team_size = match team_idx {
            0 => 1,
            1 => 3,
            2 => 5,
            3 => 8,
            4 => 15,
            _ => 25,
        };

        let pressure_options = vec!["Low", "Medium", "High", "Crunch mode"];
        let pressure_idx = Select::new()
            .with_prompt("Current deadline pressure")
            .items(&pressure_options)
            .default(1)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        let deadline_pressure = pressure_options[pressure_idx].to_lowercase();

        let challenges: String = Input::new()
            .with_prompt("Current challenges (comma-separated, or press Enter to skip)")
            .allow_empty(true)
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let current_challenges: Vec<String> = if challenges.trim().is_empty() {
            vec![]
        } else {
            challenges
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        };

        context.add_project(ProjectContext {
            name,
            tech_stack,
            current_challenges,
            team_size,
            deadline_pressure,
        });

        println!();
    }

    Ok(())
}

fn setup_goals(context: &mut UserContext) -> Result<(), String> {
    println!("{}", "🎯 Goals & Aspirations".yellow().bold());

    let add_goals = Confirm::new()
        .with_prompt("Would you like to set some professional goals for personalized suggestions?")
        .default(true)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    if !add_goals {
        println!();
        return Ok(());
    }

    let goal_categories = vec![
        "Technical Skills",
        "Career Advancement",
        "Leadership",
        "Work-Life Balance",
        "Team Building",
        "Innovation",
        "Personal Development",
        "Industry Knowledge",
    ];

    for i in 0..3 {
        let add_another = if i == 0 {
            true
        } else {
            Confirm::new()
                .with_prompt("Add another goal?")
                .default(true)
                .interact()
                .map_err(|e| format!("Input error: {e}"))?
        };

        if !add_another {
            break;
        }

        println!("{}", format!("Goal {} Details:", i + 1).cyan());

        let description: String = Input::new()
            .with_prompt("Describe your goal")
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let category_idx = FuzzySelect::new()
            .with_prompt("Goal category")
            .items(&goal_categories)
            .default(0)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        let timeline_options = vec![
            "Short-term (1-3 months)",
            "Medium-term (3-12 months)",
            "Long-term (1+ years)",
        ];
        let timeline_idx = Select::new()
            .with_prompt("Timeline")
            .items(&timeline_options)
            .default(1)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        let timeline = match timeline_idx {
            0 => "short_term",
            1 => "medium_term",
            _ => "long_term",
        }
        .to_string();

        context.add_goal(
            description,
            goal_categories[category_idx]
                .to_lowercase()
                .replace(' ', "_"),
            timeline,
        );

        println!();
    }

    Ok(())
}

fn setup_activity_patterns(context: &mut UserContext) -> Result<(), String> {
    println!("{}", "⏰ Activity Patterns & Preferences".yellow().bold());

    let active_times = vec![
        "Early morning (6-9 AM)",
        "Morning (9-12 PM)",
        "Afternoon (12-3 PM)",
        "Late afternoon (3-6 PM)",
        "Evening (6-9 PM)",
        "Night (9+ PM)",
    ];

    let time_indices = MultiSelect::new()
        .with_prompt("When are you most productive? (Select multiple)")
        .items(&active_times)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    context.activity_patterns.most_active_times = time_indices
        .iter()
        .map(|&i| {
            match i {
                0 => "early_morning",
                1 => "morning",
                2 => "afternoon",
                3 => "late_afternoon",
                4 => "evening",
                _ => "night",
            }
            .to_string()
        })
        .collect();

    let pace_options = vec![
        "Fast (I like rapid iterations and quick wins)",
        "Moderate (steady progress with regular milestones)",
        "Slow (I prefer deep, thorough understanding)",
    ];

    let pace_idx = Select::new()
        .with_prompt("What's your preferred learning/work pace?")
        .items(&pace_options)
        .default(1)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    context.activity_patterns.learning_pace = match pace_idx {
        0 => "fast",
        1 => "moderate",
        _ => "slow",
    }
    .to_string();

    let collaboration_options = vec![
        "High (I love working with others daily)",
        "Medium (regular team interactions)",
        "Low (I prefer independent work)",
        "Varies by project",
    ];

    let collab_idx = Select::new()
        .with_prompt("How much do you enjoy collaboration?")
        .items(&collaboration_options)
        .default(1)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    context.activity_patterns.collaboration_frequency = match collab_idx {
        0 => "high",
        1 => "medium",
        2 => "low",
        _ => "varies",
    }
    .to_string();

    println!();
    Ok(())
}

/// Quick update function for existing users
pub fn update_personalization() -> Result<(), String> {
    println!("{}", "🔄 Update Personalization Settings".cyan().bold());

    let mut context = UserContext::load_or_create()?;

    let update_options = vec![
        "Communication preferences",
        "Learning style",
        "Current projects",
        "Goals",
        "Activity patterns",
        "All settings",
    ];

    let update_idx = Select::new()
        .with_prompt("What would you like to update?")
        .items(&update_options)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    match update_idx {
        0 => setup_communication_preferences(&mut context)?,
        1 => setup_learning_style(&mut context)?,
        2 => {
            context.current_projects.clear();
            setup_current_projects(&mut context)?;
        }
        3 => {
            context.goals.clear();
            setup_goals(&mut context)?;
        }
        4 => setup_activity_patterns(&mut context)?,
        5 => {
            setup_communication_preferences(&mut context)?;
            setup_learning_style(&mut context)?;
            context.current_projects.clear();
            setup_current_projects(&mut context)?;
            context.goals.clear();
            setup_goals(&mut context)?;
            setup_activity_patterns(&mut context)?;
        }
        _ => return Err("Invalid selection".to_string()),
    }

    context.save()?;
    println!("{}", "✅ Personalization settings updated!".green());

    Ok(())
}
