use futures;
use futures::compat::Future01CompatExt;
use futures::future::{TryFutureExt, FutureExt};

use futures_01;
use futures_01::Future;

macro_rules! await_compat {
    ($e:expr) => {
        $e.compat().await
    }
}


// You could do this a lot better with a proc macro
macro_rules! async_compat_hack {
    (fn $fn_name:ident ( $( $fn_arg_name:ident : $fn_arg_type: ty ),*  ) 
        -> Result < $item_ty:ty , $err_ty:ty >  { 
            $($fn_body:tt)*
        }   ) => {
        
            async_compat fn $fn_name ( $( $fn_arg_name : $fn_arg_type ),*  ) 
                -> impl futures_01::Future < Item=$item_ty , Error = $err_ty >  { 
                    async fn foo_async_hack($( $fn_arg_name : $fn_arg_type ),*) -> Result < $item_ty , $err_ty > {
                        $($fn_body)*
                    }

                    foo_async_hack( $( $fn_arg_name),* ) .boxed().compat()
                 }   
    }
}

fn previous_future() -> impl futures_01::Future<Item=i32, Error=Box<dyn std::error::Error + Send + Sync>> {
    futures_01::future::result(Ok(3))
}

async_compat_hack!{
    async_compat fn hacked_future() -> Result <i32, Box<dyn std::error::Error + Send + Sync>> {
        let x = await_compat!(previous_future())?;
        Ok(x+1)
    }
}

fn post_future() -> impl futures_01::Future<Item=i32, Error=Box<dyn std::error::Error + Send + Sync>> {
    hacked_future().map(|x| x+1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(post_future().wait().unwrap(), 5)
    }
}

//fn foo_01() -> impl futures_01::Future<Item=i32, Error=Box<dyn std::error::Error + Send + Sync>> {
//     async fn foo() -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
//         let x_result: Result<i32, Box<dyn std::error::Error + Send + Sync>> = Ok(3);
//         let x_fut = futures_01::future::result(x_result);
//         let x = await_compat!(x_fut)?;
//         Ok(x+1)
//     }

//     foo().boxed().compat()
// }
