fn osman_maxfps_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    if !game.native_window() {
        return;
    }
    let workers = horizon_threads::jobs::pool().threads();
    let cap = crate::game::osman_maxfps_state::update(
        workers,
        game.native_playing(),
        elapsed.as_secs_f32() * 1000.0,
    );
    horizon_threads::osman_maxfps_pool::set_cap(cap);
}

