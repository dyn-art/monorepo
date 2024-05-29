use super::{code::run_code, comp_lib::load_comp_global};
use piccolo::{Executor, Lua};

pub struct LuaScript {
    pub source: String,
    // pub args: LuaScriptArgs,
}

impl LuaScript {
    pub fn run(&self) {
        let mut lua = Lua::full();

        let executor = lua.enter(|ctx| ctx.stash(Executor::new(ctx)));

        lua.enter(|ctx| {
            load_comp_global(ctx);
        });

        return match run_code(&mut lua, &executor, &self.source) {
            Ok(_) => log::info!("Lua code executed successfully"),
            Err(e) => {
                log::error!("{:?}", e);
            }
        };
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
    use super::*;

    fn init() {
        let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .is_test(true)
            .try_init();
    }

    #[test]
    fn e2e() {
        init();

        let code = r#"
            comp.log.warn("This is a warning")
            comp.log.info("This is an info message")
            comp.log.error("This is an error message")
            local sum = comp.sum(1, 2, 3)
            comp.log.info("Sum of 1, 2, 3 is " .. sum)
        "#;

        let script = LuaScript {
            source: code.to_string(),
        };

        script.run();
    }
}
