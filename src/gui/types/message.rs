/// The messages that are going to mutate the app state
#[derive(Debug, Clone)]
pub enum Message {
    // Send the load show command
    ShowLoadPressed,
    // Send the play command
    ShowStartPressed,
    // Send the stop command
    ShowStopPressed,
}
