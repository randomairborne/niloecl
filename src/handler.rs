use std::future::Future;

use twilight_model::{
    application::interaction::Interaction, http::interaction::InteractionResponse,
};

use crate::IntoResponse;

/// Create an extractor which extracts something from a request- an interaction here.
///
/// This trait is implemented in an imperfect way, for simplicity. You cannot consume the interaction.
/// However, as there's no big expensive-to-clone things like Body in http apps, I considered that trade-off worth it.
/// If you really need to consume it... just clone it all. It's not that expensive. If you think it is, and
/// have numbers to back it up, make an issue.
pub trait FromRequest<S>: Sized {
    type Rejection: IntoResponse;
    fn from_request(
        req: &mut Interaction,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send;
}

#[diagnostic::on_unimplemented(note = "Function argument is not a valid extractor.")]
pub trait Handler<S, R, A> {
    fn call(self, req: Interaction, state: S) -> impl Future<Output = R> + Send;
}

#[rustfmt::skip]
macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!([]);
        $name!([T1]);
        $name!([T1, T2]);
        $name!([T1, T2, T3]);
        $name!([T1, T2, T3, T4]);
        $name!([T1, T2, T3, T4, T5]);
        $name!([T1, T2, T3, T4, T5, T6]);
        $name!([T1, T2, T3, T4, T5, T6, T7]);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8]);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9]);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10]);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11]);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12]);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13]);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14]);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15]);
    };
}

macro_rules! impl_handler {
([$($ty:ident),*]) => {
    #[allow(non_snake_case, unused_mut)]
    impl<F, Fut, S, R, M, $($ty,)*> Handler<S, InteractionResponse, (M, $($ty,)*)> for F
            where
            F: Fn($($ty,)*) -> Fut + Clone + Send + Sync + 'static,
            Fut: Future<Output = R> + Send,
            R: IntoResponse,
            S: Send + Sync + 'static,
            $( $ty: FromRequest<S> + Send, )*
        {
            #[allow(unused_variables)]
            async fn call(self, mut req: Interaction, state: S) -> InteractionResponse {
                #[allow(unused_variables)]
                let state = &state;
                $(
                    let $ty = match $ty::from_request(&mut req, state).await {
                        Ok(value) => value,
                        Err(rejection) => return rejection.into_response(),
                    };
                )*

                self($($ty,)*).await.into_response()
            }
        }
    };
}

macro_rules! impl_from_request {
    (
        [$($ty:ident),*]
    ) => {
        #[allow(non_snake_case, unused_mut, unused_variables)]
        impl<S, $($ty,)*> FromRequest<S> for ($($ty,)*)
        where
            $( $ty: FromRequest<S> + Send, )*
            S: Send + Sync,
        {
            type Rejection = InteractionResponse;

             async fn from_request(parts: &mut Interaction, state: &S) -> Result<Self, Self::Rejection> {
                $(
                    let $ty = $ty::from_request(parts, state)
                        .await
                        .map_err(|err| err.into_response())?;
                )*

                Ok(($($ty,)*))
            }
        }
    };
}

all_the_tuples!(impl_from_request);
all_the_tuples!(impl_handler);

#[allow(clippy::module_name_repetitions)]
pub trait HandlerFunction<S, R>
where
    Self: Fn(Interaction, S) -> Self::Future,
    R: IntoResponse,
{
    type Future: Future<Output = R> + Send;
}

impl<F, R, S, Fut> HandlerFunction<S, R> for F
where
    F: Fn(Interaction, S) -> Fut,
    R: IntoResponse,
    Fut: Future<Output = R> + Send,
{
    type Future = Fut;
}

pub fn make<S, H, R, A>(handler: H) -> impl HandlerFunction<S, R>
where
    H: Handler<S, R, A> + Copy,
    R: IntoResponse,
{
    move |req, state| handler.call(req, state)
}
