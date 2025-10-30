use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Environment {
    #[default]
    Development,
    Testing,
    Staging,
    Production,
}

impl Environment {
    pub fn all() -> Vec<Environment> {
        vec![
            Environment::Development,
            Environment::Testing,
            Environment::Staging,
            Environment::Production,
        ]
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Development => write!(f, "开发环境"),
            Environment::Testing => write!(f, "测试环境"),
            Environment::Staging => write!(f, "预发布环境"),
            Environment::Production => write!(f, "生产环境"),
        }
    }
}

// Combined enum for pick_list that includes both environments and management option
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentOption {
    Environment(Environment),
    ManageEnvironments,
}

impl EnvironmentOption {
    pub fn all() -> Vec<EnvironmentOption> {
        let mut options: Vec<EnvironmentOption> = Environment::all()
            .into_iter()
            .map(EnvironmentOption::Environment)
            .collect();
        options.push(EnvironmentOption::ManageEnvironments);
        options
    }
}

impl fmt::Display for EnvironmentOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvironmentOption::Environment(env) => write!(f, "{}", env),
            EnvironmentOption::ManageEnvironments => write!(f, "⚙ 管理环境"),
        }
    }
}
