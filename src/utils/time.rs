use std::time::{Duration, SystemTime,UNIX_EPOCH};

// get 8 hours timestamp for jwt expiry
pub fn get_timestamp_8_hours_from_now() ->  u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    // let eighthoursfromnow = since_the_epoch + Duration::from_secs(28800);
    let eighthoursfromnow = since_the_epoch + Duration::from_secs(10);
    eighthoursfromnow.as_secs()
}
