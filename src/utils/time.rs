use std::time::{Duration, SystemTime,UNIX_EPOCH};

// current time since epoch
fn time_since_epoch() -> Duration{
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch
}

// get some hours timestamp for jwt expiry
pub fn timestamp_secs_from_now( secs: u64) ->  u64 {
    let since_the_epoch = time_since_epoch();
    let hoursfromnow = since_the_epoch + Duration::from_secs(secs);
    // let eighthoursfromnow = since_the_epoch + Duration::from_secs(10);
    hoursfromnow.as_secs()
}


// secs since the epoch
pub fn current_secs_since_epoch() -> u64{
    time_since_epoch().as_secs()
}