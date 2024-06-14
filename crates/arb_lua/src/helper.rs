use gc_arena::Mutation;
use piccolo::{Callback, CallbackReturn, Context, FromMultiValue, IntoMultiValue, IntoValue};

pub fn callback<'gc, F, A, R>(name: &'static str, mc: &Mutation<'gc>, f: F) -> Callback<'gc>
where
    F: Fn(Context<'gc>, A) -> Option<R> + 'static,
    A: FromMultiValue<'gc>,
    R: IntoMultiValue<'gc>,
{
    Callback::from_fn(mc, move |ctx, _, mut stack| {
        if let Some(res) = f(ctx, stack.consume(ctx)?) {
            stack.replace(ctx, res);
            Ok(CallbackReturn::Return)
        } else {
            Err(format!("Bad argument to {name}").into_value(ctx).into())
        }
    })
}
