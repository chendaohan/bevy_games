use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins((
            function_plugin,
            StructurePlugin::new(0),
            StructurePlugin::new(1),
            StructurePlugin::new(2),
        ))
        .add_plugins(MyPluginGroup.set(OnePlugin{settings: false}).disable::<TwoPlugin>())
        // .add_plugins(MyPluginGroup.build().disable::<TwoPlugin>())
        .add_systems(PostStartup, |mut exit_writer: EventWriter<AppExit>| {
            exit_writer.send_default();
        })
        .run()
}

fn function_plugin(app: &mut App) {
    info!("function plugin");
    app.insert_resource(FunctionPluginResource(true))
        .add_systems(Startup, || info!("function plugin system"));
}

#[derive(Resource)]
struct FunctionPluginResource(bool);

struct StructurePlugin {
    settings: i32,
}

impl StructurePlugin {
    fn new(settings: i32) -> Self {
        Self { settings }
    }
}

impl Plugin for StructurePlugin {
    // 必须实现
    // 1
    fn build(&self, _app: &mut App) {
        info!("{} build", self.settings);
    }

    // 可选实现
    // 2
    fn ready(&self, _app: &App) -> bool {
        info!("{} ready", self.settings);
        true
    }

    // 3
    fn finish(&self, _app: &mut App) {
        info!("{} finish", self.settings);
    }

    // 4
    fn cleanup(&self, _app: &mut App) {
        info!("{} cleanup", self.settings);
    }

    // 是否可以多次实例化
    fn is_unique(&self) -> bool {
        false
    }

    // 插件名称
    fn name(&self) -> &str {
        "StructurePlugin"
    }
}


struct MyPluginGroup;

impl PluginGroup for MyPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(OnePlugin{settings: true})
            .add(TwoPlugin{settings: true})
    }
}

struct OnePlugin {
    settings: bool,
}

impl Plugin for OnePlugin {
    fn build(&self, app: &mut App) {
        info!("{} One Plugin", self.settings);
    }
}

struct TwoPlugin {
    settings: bool,
}

impl Plugin for TwoPlugin {
    fn build(&self, app: &mut App) {
        info!("{} Two Plugin", self.settings);
    }
}