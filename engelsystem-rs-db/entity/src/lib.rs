mod entities;

pub mod intern {
    pub use crate::entities::*;

    pub use angel_type::Entity as AngelType;
    pub use permission::Entity as Permission;
    pub use role::Entity as Role;
    pub use session::Entity as Session;
    pub use shift::Entity as Shift;
    pub use user::Entity as User;
    pub use user_shift::Entity as UserShift;
}

pub mod public {
    use crate::entities::*;

    pub use user::ActiveModel as ActiveUser;
    pub use user::Model as User;
    pub use user::View as UserView;

    pub use permission::ActiveModel as ActivePermission;
    pub use permission::Model as PermissionModel;

    pub use role::ActiveModel as ActiveRole;
    pub use role::Model as Role;

    pub use session::ActiveModel as ActiveSession;
    pub use session::Model as Session;

    pub use shift::ActiveModel as ActiveShift;
    pub use shift::Model as Shift;

    pub use user_shift::ActiveModel as ActiveUserShift;
    pub use user_shift::Model as UserShift;

    pub use angel_type::ActiveModel as ActiveAngelType;
    pub use angel_type::Model as AngelType;
}
