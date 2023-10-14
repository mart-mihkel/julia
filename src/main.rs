fn main() {
    env_logger::init();
    pollster::block_on(julia::run());
}
