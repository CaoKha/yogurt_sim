#![allow(non_snake_case)]
use YogurtSim::run;

fn main() {
    pollster::block_on(run());
}
