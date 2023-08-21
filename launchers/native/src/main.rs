fn main() {
    tracing_subscriber::fmt::init();

    pollster::block_on(boids_core::game());
}
