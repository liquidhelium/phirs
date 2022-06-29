mod offical;
pub use offical::OfficalLoader;
use std::fs::File;
use crate::phi_types::Chart;
pub trait Loader {
    fn load_chart(file: &File) -> Chart;
    // fn save_chart(chart: Chart) -> Result<bool>; // TODO
}