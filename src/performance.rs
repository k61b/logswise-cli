use std::time::{Duration, Instant};

/// Simple performance timer for operations
#[allow(dead_code)] // Will be used in future performance improvements
pub struct Timer {
    start: Instant,
    operation: String,
}

#[allow(dead_code)] // Will be used in future performance improvements
impl Timer {
    pub fn new(operation: &str) -> Self {
        Self {
            start: Instant::now(),
            operation: operation.to_string(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn finish(self) -> Duration {
        let elapsed = self.elapsed();
        if elapsed > Duration::from_secs(5) {
            println!("⏱️  {} took {:.2}s", self.operation, elapsed.as_secs_f32());
        }
        elapsed
    }
}

/// Context for better error reporting
#[allow(dead_code)] // Will be used in future error handling improvements
pub struct OperationContext {
    pub operation: String,
    pub details: Vec<String>,
}

#[allow(dead_code)] // Will be used in future error handling improvements
impl OperationContext {
    pub fn new(operation: &str) -> Self {
        Self {
            operation: operation.to_string(),
            details: Vec::new(),
        }
    }

    pub fn add_detail(&mut self, detail: &str) {
        self.details.push(detail.to_string());
    }

    pub fn format_error(&self, error: &str) -> String {
        let mut result = format!("❌ Failed to {}: {}", self.operation, error);
        if !self.details.is_empty() {
            result.push_str("\n   Context:");
            for detail in &self.details {
                result.push_str(&format!("\n   • {}", detail));
            }
        }
        result
    }
}
