use match_derive::Matcher;

#[derive(Matcher)]
enum AppState {
    #[matcher(builder_name = login)]
    Login(LoginState),
    Main(MainState),
}

struct LoginState;
struct MainState;

fn main() {}
