enum GameState {
    Stopped,
    Live,
    Ended,
}

trait GameLogic: Sync {
    // Statistics
    fn get_total_attempts(&self) -> usize;
    fn get_successful_attempts(&self) -> usize;
    fn get_total_game_time_seconds(&self) -> usize;
    fn get_score(&self) -> usize;

    // Game variables
    fn get_word_length() -> usize;
    fn get_definition(&self) -> String;
    fn get_game_state(&self) -> GameState;
    fn get_health(&self) -> usize;
    fn get_round_time_left_seconds(&self) -> usize;

    // Signalling
    fn input_word(&mut self, word: String) -> bool;
    fn stop_timer(&mut self);
    fn start_timer(&mut self);

    fn skip_word(&mut self);
    fn reset(&mut self);
}