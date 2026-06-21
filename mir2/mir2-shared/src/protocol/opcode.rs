#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpCode {
    // 登录
    LoginRequest,
    LoginResponse,
    // 选服 / 选角色
    SelectServer,
    SelectCharacter,
    CreateCharacter,
    DeleteCharacter,
    // 游戏
    Move,
    Chat,
    Attack,
    UseSkill,
    // GM
    GmCommand,
    // 其他
    Ping,
    Unknown(u16),
}
