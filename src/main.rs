use julia::run;

fn main() {
    pollster::block_on(run());
}
