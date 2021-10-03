use nannou::noise::*;
use nannou::prelude::*;

pub fn run() {
    nannou::app(model).event(event).run();
}

struct Thing {
    position: Vec2,
}

impl Thing {
    pub fn new(p: Vec2) -> Self {
        Thing { position: p }
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
    let sn = 0.01;
    for thing in model.things.iter_mut() {
        thing.position += vec2(
            model.noise.get([
                sn * thing.position.x as f64,
                sn * thing.position.y as f64,
                0.0,
            ]) as f32,
            model.noise.get([
                sn * thing.position.x as f64,
                sn * thing.position.y as f64,
                1.0,
            ]) as f32,
        );
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
        draw.ellipse().xy(thing.position).radius(2.0).color(WHITE);
    }
    draw.to_frame(app, &frame).unwrap();
}
