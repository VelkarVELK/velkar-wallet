use crate::Notification;

pub type ChannelConnection = velkar_notify::connection::ChannelConnection<Notification>;
pub use velkar_notify::connection::ChannelType;
