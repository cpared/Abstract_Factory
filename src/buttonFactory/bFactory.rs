mod redButtonClass;
mod blueButtonClass;
use redButtonClass::RedButton;
use blueButtonClass::BlueButton;

pub struct ButtonFactory;

impl ButtonFactory{
    pub fn redButton(&self) -> RedButton{
        return RedButton;
    }

    pub fn blueButton(&self) -> BlueButton{
        return BlueButton;
    }
}
