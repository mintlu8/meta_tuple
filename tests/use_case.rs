use std::any::Any;

use meta_tuple::{MetaAny, MetaItem, MetaTuple, impl_meta_any, meta_tuple, meta_tuple_type};

pub struct Attack;
pub struct Attacker;
pub struct Defender;

#[derive(MetaItem)]
pub struct DamageDealt(usize);

impl Attack {
    pub fn calculate_damage(&self, _: &Attacker, _: &Defender) -> usize {
        40
    }
}

pub trait CardComponent {
    fn play(&self, input: impl MetaTuple) -> impl MetaTuple;

    fn join(self, other: impl CardComponent) -> impl CardComponent
    where
        Self: Sized,
    {
        Join(self, other)
    }
}

pub struct Join<A, B>(A, B);

impl<A: CardComponent, B: CardComponent> CardComponent for Join<A, B> {
    fn play(&self, input: impl MetaTuple) -> impl MetaTuple {
        let out = self.0.play(input);
        self.1.play(out)
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

pub trait CardComponent2 {
    type Out: MetaTuple;
    fn play<T: MetaTuple>(&self, input: &T) -> Self::Out;

    fn join(self, other: impl CardComponent2) -> impl CardComponent2
    where
        Self: Sized,
    {
        Join(self, other)
    }
}

impl<A: CardComponent2, B: CardComponent2> CardComponent2 for Join<A, B> {
    type Out = meta_tuple_type!(#A::Out, #B::Out);
    fn play<T: MetaTuple>(&self, input: &T) -> Self::Out {
        let out = self.0.play(input);
        let out_2 = self.1.play(&input.join_tuple(&out));
        meta_tuple!(#out, #out_2)
    }
}

pub trait Metadata: MetaAny + Any {}

impl<T> Metadata for T where T: MetaAny + Any {}

impl_meta_any!(Metadata);

#[test]
fn metadata() {
    let boxed: Box<dyn Metadata> = Box::new(());
    assert_eq!(boxed.get::<f32>(), None)
}
