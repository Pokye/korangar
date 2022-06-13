use derive_new::new;
use num::{ Zero, NumCast };
use num::traits::NumOps;
use std::cmp::PartialOrd;

use interface::traits::{ Window, PrototypeWindow };
use interface::types::InterfaceSettings;
use interface::{ WindowCache, ElementCell, Size };
use interface::elements::*;
use interface::FramedWindow;

#[derive(new)]
pub struct NumberWindow<T> {
    name: String,
    inner_pointer: *const T,
    minimum_value: T,
    maximum_value: T,
}

impl<T: Zero + NumOps + NumCast + Copy + PartialOrd + 'static> PrototypeWindow for NumberWindow<T> {

    fn to_window(&self, window_cache: &WindowCache, interface_settings: &InterfaceSettings, avalible_space: Size) -> Box<dyn Window + 'static> {

        let elements: Vec<ElementCell> = vec![
            cell!(Headline::new(self.name.clone(), Headline::DEFAULT_SIZE)),
            cell!(Headline::new("value".to_string(), Headline::DEFAULT_SIZE)),
            cell!(Slider::new(unsafe { &(*self.inner_pointer) as *const T }, self.minimum_value, self.maximum_value)),
        ];

        Box::new(FramedWindow::new(window_cache, interface_settings, avalible_space, "number".to_string(), None, elements, constraint!(200.0 > 250.0 < 300.0, ?)))
    }
}
