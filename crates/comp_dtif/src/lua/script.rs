use super::{
    code::run_code,
    comp_lib::load_comp_global,
    freeze::{Freeze, Frozen},
};
use bevy_ecs::world::World;
use piccolo::{Context, Executor, Lua, UserData};

/// A frozen reference to the ECS [`World`].
///
// This type can be converted into lua userdata for accessing the world from lua.
pub type WorldRef = Frozen<Freeze![&'freeze mut World]>;

impl WorldRef {
    /// Convert this [`WorldRef`] into a Lua userdata.
    pub fn into_userdata(self, ctx: Context<'_>) -> UserData<'_> {
        UserData::new_static(&ctx, self)
    }

    pub fn load_global<'gc>(&self, ctx: Context<'gc>) {
        ctx.globals()
            .set(ctx, "world", self.clone().into_userdata(ctx))
            .unwrap();
    }
}

pub struct LuaScript {
    pub source: String,
    // pub args: LuaScriptArgs,
}

impl LuaScript {
    pub fn run(&self, world: &mut World) {
        let mut lua = Lua::full();

        let executor = lua.enter(|ctx| ctx.stash(Executor::new(ctx)));

        Frozen::<Freeze![&'freeze mut World]>::in_scope(world, |world| {
            lua.enter(|ctx| {
                load_comp_global(ctx, world);
                //    WorldRef(world).load_global(ctx);
            });

            return match run_code(&mut lua, &executor, &self.source) {
                Ok(_) => log::info!("Lua code executed successfully"),
                Err(err) => {
                    log::error!("Failed to execute Lua code by exception: {:?}", err);
                }
            };
        });
    }
}

pub enum LuaScriptArgs {
    Number(NumberArg),
}

pub struct NumberArg {
    default: f32,
}

#[cfg(test)]
mod tests {
    use bevy_app::{App, Update};
    use bevy_ecs::event::EventReader;
    use dyn_comp_bundles::events::{CoreInputEvent, InputEvent, UpdateCompositionSizeInputEvent};

    use super::*;

    fn init() {
        let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn e2e() {
        init();

        let mut app = App::new();
        CoreInputEvent::register_events(&mut app);
        app.add_systems(Update, event_listener_system);

        let code = r#"
            comp.log.warn("This is a warning")
            comp.log.info("This is an info message")
            comp.log.error("This is an error message")
            local sum = comp.sum(1, 2, 3)
            comp.log.info("Sum of 1, 2, 3 is " .. sum)

            local my_event = '{"type":"UpdateCompositionSize","size":[50, 100]}'
            comp.send_event(my_event)
        "#;

        let script = LuaScript {
            source: code.to_string(),
        };

        script.run(&mut app.world);

        app.update();
    }

    fn event_listener_system(mut event_reader: EventReader<UpdateCompositionSizeInputEvent>) {
        for event in event_reader.read() {
            log::info!("Event: {:?}", event);
        }
    }
}
