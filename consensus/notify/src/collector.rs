use crate::notification::Notification;
use velkar_notify::{collector::CollectorFrom, converter::ConverterFrom};

pub type ConsensusConverter = ConverterFrom<Notification, Notification>;
pub type ConsensusCollector = CollectorFrom<ConsensusConverter>;
