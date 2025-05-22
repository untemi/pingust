use crate::Result;
use crate::error::*;
use crate::msg::Msg;
use notify_rust::Notification;
use std::time::Duration;

pub trait Notif {
    fn notify(&self) -> Result<()>;
}

impl Notif for Msg<'_> {
    fn notify(&self) -> Result<()> {
        Notification::new()
            .summary("Pingust")
            .appname("Pingust")
            .body(self.message().as_ref())
            .urgency(self.as_urgency())
            .timeout(Duration::from_secs(2))
            .show()
            .map_err(AnyError::new)?;

        Ok(())
    }
}
