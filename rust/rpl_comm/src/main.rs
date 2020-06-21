extern crate rpl_comm;

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}
/*
fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
*/

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules(){}
        }
    }
}

use a::series::of;


enum TrafficLight{
    Red,Yellow,Green,
}

use TrafficLight::*;

fn main(){
    rpl_comm::client::connect();
    rpl_comm::network::connect();
    rpl_comm::network::server::connect();

    of::nested_modules();
    let r = Red;
    let y = Yellow;
    let g = Green;
}