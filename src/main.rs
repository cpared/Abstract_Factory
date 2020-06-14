mod buttonFactory;
use buttonFactory::bFactory::ButtonFactory;

fn main() {
    let factory = ButtonFactory{};
    let redButton = factory.redButton();
    let blueButton = factory.blueButton();
    redButton.click();
    blueButton.click();
}
