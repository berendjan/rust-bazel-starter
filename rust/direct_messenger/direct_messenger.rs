pub trait Sender<Message, DirectMessenger, Response = ()> {
    fn send(
        message: &Message,
        router: &DirectMessenger,
    ) -> impl std::future::Future<Output = Response> + Send;
}

pub trait MessengerRoute<H, Message, Response = ()> {
    fn route(&self, message: &Message) -> impl std::future::Future<Output = Response> + Send;
}

#[macro_export]
macro_rules! direct_messenger {
    (
        $( derive: [ $( $derive:ident ),+ $(,)? ], )?
        handlers: [ $( $handler_ident:ident: $handler_ty:ty ),+ $(,)? ]
        routes: [ $( $source:ty, $message:ty$(, $response:ty)?: [ $( $(->)? $receiver:ident ),+ ] ),+ $(,)? ]
    ) => {

        $( #[derive( $( $derive ),+ )] )?
        pub struct DirectMessenger {
            $(
                pub $handler_ident: $handler_ty,
            )+
        }

        pub mod trait_impls {
            use direct_messenger::Sender;
            use super::*;

            $(
                impl Sender<$message, DirectMessenger$(, $response)?> for $source {
                    #[inline]
                    async fn send(message: &$message, router: &DirectMessenger)$( -> $response)? {
                        direct_messenger::MessengerRoute::<Self, $message$(, $response)?>::route(router, message).await
                    }
                }
            )+

            $(
                impl direct_messenger::MessengerRoute<$source, $message$(, $response)?> for DirectMessenger {
                    #[inline]
                    async fn route(&self, message: &$message)$( -> $response)? {
                        $(
                            let _out = self.$receiver.handle(message, self).await;
                        )+
                        _out
                    }
                }
            )+
        }
    }
}
