

pub fn main(){
    let mut cacher = Cacher::new(|num| {
        num + 2
    });
    println!("{}", cacher.value(32));
}

struct Cacher<T> {
    handler: T,
    value: Option<i32>
}

impl<T: Fn(i32)->i32 > Cacher<T>
{
    fn new(handler: T) -> Cacher<T> {
        Cacher{
            handler,
            value: None
        }
    }

    fn value(&mut self, arg: i32) ->i32 {
        match self.value {
            Some(v) => v,
            None =>{
                let v = (self.handler)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}