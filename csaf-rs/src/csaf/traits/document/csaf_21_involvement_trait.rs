use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf_traits::{ActionTrait, WithDate};
use crate::schema::csaf2_1::schema::{Action as Action21, Involvement as Involvement21 };
use crate::validation::CsafError;

pub trait Csaf21InvolvementTrait {
    type ActionType: ActionTrait;

    fn get_action(&self) -> impl Iterator<Item=&Self::ActionType> + '_;
}

impl Csaf21InvolvementTrait for NotPresentInCsaf20 {
    type ActionType = NotPresentInCsaf20;

    fn get_action(&self) -> impl Iterator<Item=&Self::ActionType> + '_ {
        self.into_any_iter()
    }
}

impl Csaf21InvolvementTrait for Involvement21 {
    type ActionType = Action21;

    fn get_action(&self) -> impl Iterator<Item=&Self::ActionType> + '_ {
        self.actions.iter()
    }
}