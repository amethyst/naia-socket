pub trait GaiaMessage {
    fn new() -> Result<Self> where Self: Sized;

    fn pack(&self);

    fn unpack(&self);
}