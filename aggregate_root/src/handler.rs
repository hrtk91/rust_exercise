pub mod user;

// 後方互換性のため、handlersモジュールとしてuserモジュールを再エクスポート
pub mod handlers {
    pub use super::user::*;
}