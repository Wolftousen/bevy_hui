use bevy::prelude::*;
use bevy_hui::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin {
                default_sampler: bevy::image::ImageSamplerDescriptor::nearest(),
            }),
            HuiPlugin,
        ))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, check_data)
        .run();
}

fn process_data_test(_: &String, entity: &mut EntityCommands) {
    entity.insert(Test(0.0));
}

#[derive(Component, Deref, DerefMut)]
pub struct Test(pub f32);

fn setup_scene(mut cmd: Commands, server: Res<AssetServer>, mut html_comps: HtmlComponents, mut data_bindings: HtmlDataBindings) {
    cmd.spawn(Camera2d);
    
    html_comps.register("data", server.load("demo/data.html"));
    data_bindings.register("data_test", process_data_test);

    cmd.spawn(HtmlNode(server.load("demo/data.html")));
}

fn check_data(query: Query<&Test>) {
    for test in query.iter() {
        println!("Test: {}", test.0);
    }
}