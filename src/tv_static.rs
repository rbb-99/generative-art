use nannou::noise::*;
use nannou::prelude::*;

pub fn run() {
    nannou::app(model).event(event).run();
}

struct Thing {
    positions: Vec<Vec2>,
}

impl Thing {
    pub fn new(p: Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(p);
        Thing { positions }
    }
}

struct Model {
    things: Vec<Thing>,
    noise: Perlin,
}

const N_THINGS: usize = 5000;

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    let mut things = Vec::new();

    for _i in 0..N_THINGS {
        let thing = Thing::new(vec2(
            (random::<f32>() - 0.5) * 800.0,
            (random::<f32>() - 0.5) * 800.0,
        ));
        things.push(thing);
    }

    let noise = Perlin::new();
    Model { things, noise }
}

fn event(_app: &App, model: &mut Model, _event: Event) {
    let sn = random::<f64>() * 0.1;
    for thing in model.things.iter_mut() {
        thing.positions.clear();
        thing.positions.push(vec2(
            (random::<f32>() - 0.5) * 800.0,
            (random::<f32>() - 0.5) * 800.0,
        ));

        let last = thing.positions[0];
        let new = last
            + vec2(
                model
                    .noise
                    .get([sn * last.x as f64, sn * last.y as f64, 0.0]) as f32,
                model
                    .noise
                    .get([sn * last.x as f64, sn * last.y as f64, 1.0]) as f32,
            );
        thing.positions.insert(0, new);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect()
        .w_h(800.0, 800.0)
        .color(srgba(0.0, 0.0, 0.0, 0.1));

    for thing in model.things.iter() {
        draw.polyline()
            .points(thing.positions.iter().cloned())
            .color(WHITE);
    }
    draw.to_frame(app, &frame).unwrap();
}
