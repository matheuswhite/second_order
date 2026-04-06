use aule::prelude::*;
use ndarray::Array2;

fn main() {
    let r1 = 3.3e3;
    let r2 = 4.7e3;
    let c1 = 10e-6;
    let c2 = 100e-6;

    let a = Array2::from_shape_vec(
        (2, 2),
        vec![
            -(1.0 / (c1 * r1) + 1.0 / (c1 * r2)),
            1.0 / (c1 * r2),
            1.0 / (c2 * r2),
            -(1.0 / (c2 * r2)),
        ],
    )
    .unwrap();
    let b = vec![1.0 / (c1 * r1), 0.0];

    let time = Time::new(1e-3, 4.0);
    let mut input = Step::new(1.0);
    let mut v2_model = SS::<Euler, _>::new(a.clone(), b.clone(), vec![0.0, 1.0], 0.0);
    let mut v1_model = SS::<Euler, _>::new(a.clone(), b.clone(), vec![1.0, 0.0], 0.0);
    let mut graphs =
        Plotter::new("C2 Voltage".to_string(), ["u(t)", "Vc1(t)", "Vc2(t)"]).with_light_theme();

    for dt in time {
        let u = dt * input.as_block();
        let c2_v = v2_model.output(u);
        let c1_v = v1_model.output(u);

        let _ = [u, c1_v, c2_v].pack() * graphs.as_block();
    }

    graphs.display();

    let _ = graphs.save("second_order.png");

    graphs.join();
}
