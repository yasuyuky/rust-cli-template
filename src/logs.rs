pub fn init_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_max_level(tracing::Level::INFO)
        .with_level(false)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
