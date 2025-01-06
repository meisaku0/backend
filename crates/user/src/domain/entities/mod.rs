pub mod avatar;
pub mod email;
pub mod password;
pub mod session;
pub mod user;

pub use {
    avatar as AvatarEntity, email as EmailEntity, password as PasswordEntity, session as SessionEntity,
    user as UserEntity,
};
