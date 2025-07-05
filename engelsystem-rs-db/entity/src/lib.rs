mod entities;

pub mod intern {
    pub use crate::entities::*;

    pub use user::Entity as User;
    pub use permission::Entity as Permission;
    pub use role::Entity as Role;
    pub use session::Entity as Session;
    pub use shift::Entity as Shift;
    pub use user_shift::Entity as UserShift;
    pub use angel_type::Entity as AngelType;
}

pub mod public {
    use crate::entities::*;

    pub use user::Model       as User;
    pub use user::ActiveModel as ActiveUser;
    pub use user::View        as UserView;

    pub use permission::Model       as PermissionModel;
    pub use permission::ActiveModel as ActivePermission;

    pub use role::Model       as Role;
    pub use role::ActiveModel as ActiveRole;

    pub use session::Model       as Session;
    pub use session::ActiveModel as ActiveSession;

    pub use shift::Model       as Shift;
    pub use shift::ActiveModel as ActiveShift;

    pub use user_shift::Model       as UserShift;
    pub use user_shift::ActiveModel as ActiveUserShift;

    pub use angel_type::Model       as AngelType;
    pub use angel_type::ActiveModel as ActiveAngelType;
}
