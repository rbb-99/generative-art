use nannou::prelude::*;

pub fn run() {
    nannou::app(model).event(event).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw=app.draw();
    draw.background().color(PLUM);

    let time=app.elapsed_frames() as f32 /60.0;

    for i in 0..10{
        let angle=i as f32 * 0.1 * TAU+time;
        draw.ellipse().x_y(200.0*angle.cos(),200.0*angle.sin()).color(STEELBLUE);
    }
    draw.to_frame(app,&frame).unwrap();
}
