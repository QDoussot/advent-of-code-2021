use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Element: PartialEq + Eq + Hash {}

pub trait Application {
    type In: Element;
    type Out: Element;
    fn start(&self) -> HashSet<Self::In>;

    fn image(&self, e: &Self::In) -> Self::Out;
}

pub trait ApplicationExt: Application {
    fn antecedent(&self, o: &<Self as Application>::Out) -> Vec<<Self as Application>::In> {
        self.start().into_iter().filter(|x| self.image(x) == *o).collect()
    }

    fn ensemble_injectif(&self) -> HashSet<<Self as Application>::In> {
        self.start()
            .into_iter()
            .filter(|x| self.antecedent(&self.image(x)).len() == 1)
            .collect()
    }

    fn as_map(&self) -> HashMap<Self::In, Self::Out> {
        //
        self.start()
            .into_iter()
            .map(|x| {
                let y = self.image(&x);
                (x, y)
            })
            .collect()
    }
}

impl<T: Application> ApplicationExt for T {}
