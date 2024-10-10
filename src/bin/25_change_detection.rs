use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*,
    time::common_conditions::once_after_real_delay,
};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))),
            LogPlugin::default(),
        ))
        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(2)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (change_my_component, change_my_resource)
                .run_if(once_after_real_delay(Duration::from_secs(2))),
        )
        .add_systems(
            Update,
            (
                added_info,
                changed_info,
                print_components_info,
                print_mutable_components_info,
                print_resource_info,
            ),
        )
        .add_systems(
            Update,
            app_exit.run_if(once_after_real_delay(Duration::from_secs(5))),
        )
        .run()
}

#[derive(Component)]
struct MyComponent(i32);

#[derive(Resource)]
struct MyResource(i32);

fn setup(mut commands: Commands) {
    commands.spawn(MyComponent(0));

    commands.insert_resource(MyResource(0));
}

fn change_my_component(mut component: Query<&mut MyComponent>) {
    let Ok(mut component) = component.get_single_mut() else {
        return;
    };
    component.0 += 1;
    // unsafe { *(component.0 as *mut i32) += 1; }
}

fn change_my_resource(mut resource: ResMut<MyResource>) {
    resource.0 += 1;
}

fn added_info(components: Query<&MyComponent, Added<MyComponent>>) {
    for component in &components {
        info!("Added MyComponent: {}", component.0);
    }
}

fn changed_info(components: Query<&MyComponent, Changed<MyComponent>>) {
    for component in &components {
        info!("Changed MyComponent: {}", component.0);
    }
}

fn print_components_info(components: Query<Ref<MyComponent>>) {
    for component in &components {
        info!("print components info: {}", component.0);

        if component.is_added() {
            info!("print components info added: {}", component.0);
        }

        if component.is_changed() {
            info!("print components info changed: {}", component.0);
        }
    }
}

// mut components: Query<Mut<MyComponent>>
fn print_mutable_components_info(mut components: Query<&mut MyComponent>) {
    for mut component in &mut components {
        info!("print mutable components info: {}", component.0);
        if component.is_added() {
            info!("print mutable components info added: {}", component.0);
        }

        if component.is_changed() {
            info!("print mutable components info changed: {}", component.0);
        }
    }
}

fn print_resource_info(resource: Res<MyResource>) {
    info!("print resource info: {}", resource.0);

    if resource.is_added() {
        info!("print resource info added: {}", resource.0);
    }

    if resource.is_changed() {
        info!("print resource info changed: {}", resource.0);
    }
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    exit_writer.send_default();
}
