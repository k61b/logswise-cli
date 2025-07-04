#[derive(Debug, Clone)]
pub enum SetupMode {
    Express,
    Template,
    Custom,
    Import,
}

#[derive(Debug, Clone)]
pub enum SetupAction {
    UpdateExisting,
    CreateNew,
    UseTemplate,
}

pub struct SetupOptions;

impl SetupOptions {
    pub fn role_options() -> Vec<&'static str> {
        vec![
            "Software Developer",
            "Data Scientist",
            "DevOps Engineer",
            "Product Manager",
            "Other",
        ]
    }

    pub fn tech_options() -> Vec<&'static str> {
        vec![
            "JavaScript/TypeScript",
            "Python",
            "Rust",
            "Go",
            "Java",
            "Other",
        ]
    }

    pub fn llm_options() -> Vec<&'static str> {
        vec![
            "llama3.2 (recommended for general use)",
            "codellama (code-focused)",
            "mistral (lightweight)",
            "custom (I'll specify)",
        ]
    }

    pub fn profession_options() -> Vec<&'static str> {
        vec![
            "Software Developer",
            "Product Manager",
            "Designer",
            "Data Scientist",
            "QA Engineer",
            "DevOps Engineer",
            "Sales Engineer",
            "Technical Writer",
            "Other",
        ]
    }

    pub fn job_title_options() -> Vec<&'static str> {
        vec![
            "Intern", "Junior", "Mid", "Senior", "Lead", "Manager", "Director", "VP", "C-level",
            "Other",
        ]
    }

    pub fn language_options() -> Vec<&'static str> {
        vec![
            "Rust",
            "Python",
            "JavaScript/TypeScript",
            "Go",
            "Java",
            "C#",
            "C/C++",
            "Ruby",
            "Swift",
            "Kotlin",
            "Other",
        ]
    }

    pub fn company_size_options() -> Vec<&'static str> {
        vec![
            "1-10",
            "10-100",
            "100-500",
            "500-1000",
            "1000-5000",
            "5000+",
        ]
    }

    pub fn work_mode_options() -> Vec<&'static str> {
        vec!["Remote", "On-site", "Hybrid"]
    }

    pub fn years_experience_options() -> Vec<&'static str> {
        vec![
            "<1 year",
            "1-3 years",
            "3-5 years",
            "5-10 years",
            "10+ years",
        ]
    }

    pub fn get_smart_defaults(profession: &str) -> (String, String, String, String) {
        match profession {
            "Software Developer" => (
                "Mid".to_string(),
                "10-100".to_string(),
                "3-5 years".to_string(),
                "Remote".to_string(),
            ),
            "Data Scientist" => (
                "Senior".to_string(),
                "100-500".to_string(),
                "3-5 years".to_string(),
                "Hybrid".to_string(),
            ),
            "DevOps Engineer" => (
                "Senior".to_string(),
                "100-500".to_string(),
                "5-10 years".to_string(),
                "Remote".to_string(),
            ),
            "Product Manager" => (
                "Senior".to_string(),
                "100-500".to_string(),
                "5-10 years".to_string(),
                "Hybrid".to_string(),
            ),
            _ => (
                "Mid".to_string(),
                "10-100".to_string(),
                "3-5 years".to_string(),
                "Remote".to_string(),
            ),
        }
    }
}
