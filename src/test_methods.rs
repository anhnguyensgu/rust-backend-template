use crate::adapters::repository::in_memory::InMemoryUserRepo;
use crate::adapters::http::{AppState, router};
use crate::application::user_service::UserService;

pub fn test_methods() {
    let repo = InMemoryUserRepo::new();
    let user_service = UserService::new(repo);
    let state = AppState {
        user_service: std::sync::Arc::new(user_service),
    };
    
    let router = router(state);
    
    // Let's see what methods are available
    // This won't compile but will show us what the compiler thinks is available
    let _ = router.into_make_service();
}