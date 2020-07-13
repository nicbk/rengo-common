use {
    crate::logic::{
        Board,
        Stone,
        Position,
        InvalidMove,
        Move,
    },
    serde::{Serialize, Deserialize},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub username: String,
    pub score: u32,
    pub stone: Stone,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Room {
    // TODO: Use LinkedHashMap
    pub players: Vec<(String, Player)>,
    pub current_player: String,
    pub self_player: String,
    pub board: Board,
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    // Username, Room
    Login(String, String),
    // Room, Capacity, Board width, Board height
    RoomCreate(String, u8, u8, u8),
    // Some means a stone was requested to be placed
    // None means that the player passed
    Place(Option<Position<u8>>),
    Chat(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    LoginResponse(Result<Room, LoginError>),
    RoomCreateResponse(Result<Option<Room>, RoomCreateError>),
    PlaceResponse(Result<Move<u8>, InvalidMove>),
    PlayerAdd(Player),
    PlayerRemove(String),
    NextTurn(String),
    Chat(String),
    AlreadyLoggedIn,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatError {
    MessageTooLong,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LoginError {
    RoomDoesNotExist(String),
    RoomFull,
    UsernameTaken,
    UsernameTooLong,
    RoomNameTooLong,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RoomCreateError {
    RoomNameTooLong,
    RoomNameTaken,
}
