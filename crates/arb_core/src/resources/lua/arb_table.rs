#![cfg(feature = "lua_scripts")]

use bevy_ecs::world::World;
use dyn_arb_bundles::events::{CoreInputEvent, InputEvent};
use dyn_arb_lua::{
    freeze::{Freeze, Frozen},
    helper::callback,
    serde::from_value,
};
use piccolo::{Context, Table, Value};

/// A frozen reference to the ECS [`World`].
pub type FrozenWorld = Frozen<Freeze![&'freeze mut World]>;

pub fn load_arb_table_global<'gc>(ctx: Context<'gc>, frozen_world: FrozenWorld) {
    let arb_table = create_arb_table(ctx, frozen_world);
    ctx.set_global("arb", arb_table).unwrap();
}

fn create_arb_table<'gc>(ctx: Context<'gc>, frozen_world: FrozenWorld) -> Table<'gc> {
    let arb_table = Table::new(&ctx);

    let movable_frozen_world = frozen_world.clone();
    let send_event_callback = callback("sendEvent", &ctx, move |_, v: Value| {
        match from_value::<CoreInputEvent>(v) {
            Ok(event) => {
                movable_frozen_world.with_mut(|mut world| {
                    event.send_into_world(&mut world);
                });
                Some(Value::Nil)
            }
            Err(err) => {
                log::error!(
                    "[send_event_callback] Failed to parse value '{}' as event by exception: {}",
                    v,
                    err
                );
                None
            }
        }
    });

    let movable_frozen_world = frozen_world.clone();
    let send_events_callback = callback("sendEvents", &ctx, move |_, v: Value| match v {
        Value::Table(events_table) => {
            for (_, event_value) in events_table.iter() {
                match from_value::<CoreInputEvent>(event_value) {
                    Ok(event) => {
                        movable_frozen_world.with_mut(|mut world| {
                            event.send_into_world(&mut world);
                        });
                    }
                    Err(err) => {
                        log::error!(
                                "[send_events_callback] Failed to parse value '{}' as event by exception: {}",
                                event_value,
                                err
                            );
                    }
                }
            }
            Some(Value::Nil)
        }
        _ => {
            log::error!("[send_events_callback] Expected a table of events");
            None
        }
    });

    arb_table
        .set(ctx, "sendEvent", send_event_callback)
        .unwrap();
    arb_table
        .set(ctx, "sendEvents", send_events_callback)
        .unwrap();

    return arb_table;
}
