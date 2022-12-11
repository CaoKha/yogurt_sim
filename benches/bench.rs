#![feature(test)]

extern crate test;
extern crate yogurt_sim;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = yogurt_sim::Universe::new(64, 64);

    b.iter(|| {
        universe.tick();
    });
}
