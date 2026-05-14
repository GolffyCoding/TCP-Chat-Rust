pub mod broadcaster;

pub use broadcaster::MessageBus;
pub use broadcaster::MessageBroadcaster;

pub trait MessageSender: Send + Sync {
    fn send(&self, msg: String) -> anyhow::Result<()>;
}