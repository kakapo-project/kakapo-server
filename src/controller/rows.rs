
use objekt::Clone;

use super::types::DataPoint;

#[derive(Clone)]
pub struct Row(Vec<DataPoint>);


pub trait RowContainer: Clone {

}
clone_trait_object!(RowContainer);


#[derive(Clone)]
struct RowContainerImpl {

}
impl RowContainerImpl {
    pub fn new() -> Self {
        RowContainerImpl {}
    }
}

pub trait RowInsertion: Clone {

}
clone_trait_object!(RowInsertion);


pub trait RowDeletion: Clone {

}
clone_trait_object!(RowDeletion);


pub trait RowUpdate: Clone {

}
clone_trait_object!(RowUpdate);
