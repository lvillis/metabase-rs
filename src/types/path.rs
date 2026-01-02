use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PathParam(String);

impl PathParam {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PathParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for PathParam {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for PathParam {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<i64> for PathParam {
    fn from(value: i64) -> Self {
        Self(value.to_string())
    }
}

impl From<u64> for PathParam {
    fn from(value: u64) -> Self {
        Self(value.to_string())
    }
}

impl From<i32> for PathParam {
    fn from(value: i32) -> Self {
        Self(value.to_string())
    }
}

impl From<u32> for PathParam {
    fn from(value: u32) -> Self {
        Self(value.to_string())
    }
}

impl From<usize> for PathParam {
    fn from(value: usize) -> Self {
        Self(value.to_string())
    }
}

impl From<crate::types::ids::UserId> for PathParam {
    fn from(value: crate::types::ids::UserId) -> Self {
        Self(value.to_string())
    }
}

impl From<crate::types::ids::DashboardId> for PathParam {
    fn from(value: crate::types::ids::DashboardId) -> Self {
        Self(value.to_string())
    }
}
