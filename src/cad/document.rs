use super::feature::Feature;
use super::geometry::point::*;
use super::model::CadModel;

pub struct CadDocument {
    model: CadModel,

    history: Vec<Feature>,
    timeline_position: usize,

    next_point_id: usize,
}

impl CadDocument {
    pub fn new() -> Self {
        CadDocument {
            model: CadModel::new(),
            history: Vec::new(),
            timeline_position: 0,
            next_point_id: 0,
        }
    }

    fn allocate_point_id(&mut self) -> PointId {
        let point_id = PointId(self.next_point_id.into());
        self.next_point_id += 1;
        point_id
    }

    pub fn add_point(&mut self, def: PointDefinition) {
        println!("Add point: {:?}", def);
        let point_id = self.allocate_point_id();

        let feature = Feature::AddPoint {
            id: point_id,
            definition: def,
        };

        self.history.insert(self.timeline_position, feature);
        self.regenerate();
    }

    fn regenerate(&mut self) {
        //TODO: regenerate model from feature history
    }
}
