use nannou::prelude::*;

struct Model {}

pub fn run() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw=app.draw();
    draw.background().color(PLUM);

    for i in 0..10{
        let angle=i as f32 * 0.1 * TAU;
        draw.ellipse().x_y(100.0*angle.cos(),100.0*angle.sin()).color(STEELBLUE);
    }
    draw.to_frame(app,&frame).unwrap();
}
