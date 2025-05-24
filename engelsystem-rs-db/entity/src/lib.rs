mod entities;

pub use entities::*;

pub use user::Entity      as User;
pub use user::ActiveModel as ActiveUser;

pub use permission::Entity      as Permission;
pub use permission::ActiveModel as ActivePermission;

pub use role::Entity      as Role;
pub use role::ActiveModel as ActiveRole;
