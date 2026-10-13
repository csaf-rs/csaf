use crate::csaf::traits::util::not_present_20::NotPresentInCsaf20;
use crate::csaf_traits::{ActionTrait, EntityTrait, WithDate};
use crate::schema::csaf2_1::schema::{Action as Action21, Entity as Entity21, Involvement as Involvement21};
use crate::validation::CsafError;

pub trait Csaf21InvolvementTrait {
    type ActionType: ActionTrait;
    
    type EntityType: EntityTrait;

    fn get_action(&self) -> impl Iterator<Item=&Self::ActionType> + '_;
    
    fn get_entity(&self) -> impl Iterator<Item=&Self::EntityType> + '_;
    
    
}

impl Csaf21InvolvementTrait for NotPresentInCsaf20 {
    type ActionType = NotPresentInCsaf20;
    type EntityType = NotPresentInCsaf20;

    fn get_action(&self) -> impl Iterator<Item=&Self::ActionType> + '_ {
        self.into_any_iter()
    }

    fn get_entity(&self) -> impl Iterator<Item=&Self::EntityType> + '_ {
        self.into_any_iter()
    }
}

impl Csaf21InvolvementTrait for Involvement21 {
    type ActionType = Action21;
    type EntityType = Entity21;

    fn get_action(&self) -> impl Iterator<Item=&Self::ActionType> + '_ {
        self.actions.iter()
    }

    fn get_entity(&self) -> impl Iterator<Item=&Self::EntityType> + '_ {
        self.entities.iter()
    }
    
    
}