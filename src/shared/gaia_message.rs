
use std::any::Any;
use crate::Result;

pub trait GaiaMessage {

    fn as_any(&self) -> &dyn Any;

    fn pack(&self);

    fn unpack(&self);
}