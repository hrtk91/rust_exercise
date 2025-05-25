use std::marker::PhantomData;

struct Pipeline<A, B, F>(F, PhantomData<(A, B)>)
where
    F: Fn(A) -> B;

impl<A, B, F> Pipeline<A, B, F>
where
    F: Fn(A) -> B,
{
    fn then<C>(self, f: impl Fn(B) -> C) -> Pipeline<A, C, impl Fn(A) -> C> {
        Pipeline(move |a| f((self.0)(a)), PhantomData)
    }

    fn then_if<C>(
        self,
        cond: impl Fn(&B) -> bool,
        f: impl Fn(B) -> C,
    ) -> Pipeline<A, Option<C>, impl Fn(A) -> Option<C>> {
        Pipeline(
            move |a| {
                let b = (self.0)(a);
                if cond(&b) {
                    Some(f(b))
                } else {
                    None
                }
            },
            PhantomData,
        )
    }

    fn run(self, input: A) -> B {
        (self.0)(input)
    }
}

fn pipe<A, B>(f: impl Fn(A) -> B) -> Pipeline<A, B, impl Fn(A) -> B> {
    Pipeline(f, PhantomData)
}

fn main() {
    pipe(|x| format!("{} world!", x))
        .then(|y| println!("{}", y))
        .run("Hello");

    let result = pipe(|a| a * 2).then_if(|b| *b < 5, |b| b * 2).run(4);
    println!("{:?}", result)
}
