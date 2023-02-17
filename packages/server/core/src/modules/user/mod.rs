mod query;
pub use query::UserQuery;

mod mutation;
pub use mutation::UserMutation;

mod inject_super_admin;
mod service;
pub use inject_super_admin::inject_super_admin;
