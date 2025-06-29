# meta_tuple

A statically typed opaque tuple that can contain any type.

## Use Case

This crate is generally used for complicated input and output abstractions, for example

```rust
pub trait CardComponent {
    fn play(&self, input: &impl MetaTuple) -> impl MetaTuple;
}

impl CardComponent for Attack {
    fn play(&self, input: &impl MetaTuple) -> impl MetaTuple {
        let attacker = input.get::<Attacker>();
        let defender = input.get::<Defender>();
        let damage_dealt = self.calculate_damage(attacker, defender);
        input.join(DamageDealt(damage_dealt))
    }
}

pub trait CardComponent {
    pub fn play(&self, input: &impl MetaTuple) -> impl MetaTuple;
}
```

## License

License under either of

Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Contributions are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
