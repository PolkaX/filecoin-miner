use api::HeadChangeType;
use plum_tipset::Tipset;
pub type Result<T> = std::result::Result<T, EventsError>;

#[derive(thiserror::Error, Debug)]
pub enum EventsError {
    #[error("unexpected initial head notification length: {0}")]
    UnexpectedInitial(usize),
    #[error("expected first head notification type to be 'current', was '{0:?}'")]
    UnexpectedInitialType(HeadChangeType),
    #[error("expected new tipset height to be at least {0}, was {1}")]
    HigherThenBest(u64, u64),
    #[error("revert tipset didn't match cache head|best:{0:?}|revert:{1:?}")]
    RevertError(Tipset, Tipset),
    #[error("requested tipset not in cache|best:{0}|req:{1}")]
    NotInCache(u64, u64),
    #[error("fail to get tail from cache")]
    GetTailFailed,
    #[error("overflow cache capacity|try times:{0}|cap:{0}")]
    OverflowCacheCapacity(usize, usize),
    #[error("other error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
