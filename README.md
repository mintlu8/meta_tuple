# meta_tuple

A statically typed opaque tuple that can contain any type.

## Use Cases

This crate is generally used for complicated input and output abstractions.
The incompleteness of rust's current trait system makes this crate somewhat tricky
to use, so we compiled a few standard usage patterns:

Chaining builder pattern:

```rust
pub trait CardComponent {
    fn play(&self, input: impl MetaTuple) -> impl MetaTuple;

    fn join(self, other: impl CardComponent) -> impl CardComponent where Self: Sized{
        Join(self, other)
    }
}

impl CardComponent for Attack {
    fn play(&self, input: impl MetaTuple) -> impl MetaTuple {
        let attacker = input.get::<Attacker>().unwrap();
        let defender = input.get::<Defender>().unwrap();
        let damage_dealt = self.calculate_damage(attacker, defender);
        input.join(DamageDealt(damage_dealt))
    }
}

pub struct Join<A, B>(A, B);

impl<A: CardComponent, B: CardComponent> CardComponent for Join<A, B> {
    fn play(&self, input: impl MetaTuple) -> impl MetaTuple{
        let out = self.0.play(input);
        self.1.play(out)
    }
}
```

Reference input pattern:

```rust
pub trait CardComponent {
    type Out: MetaTuple;
    fn play<T: MetaTuple>(&self, input: &T) -> Self::Out;

    fn join(self, other: impl CardComponent) -> impl CardComponent where Self: Sized{
        Join(self, other)
    }
}

impl<A: CardComponent, B: CardComponent> CardComponent for Join<A, B> {
    type Out = meta_tuple_type!(#A::Out, #B::Out);
    fn play<T: MetaTuple>(&self, input: &T) -> Self::Out {
        let out = self.0.play(input);
        let out_2 = self.1.play(&input.join_tuple(&out));
        meta_tuple!(#out, #out_2)
    }
}
```

Note: for RPITIT to be used here precise capturing must be stabilized first.

## License

License under either of

Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Contributions are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
