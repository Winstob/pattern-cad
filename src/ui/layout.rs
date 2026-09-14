#[derive(Default)]
pub struct Layout {
    nodes: Vec<LayoutNode>,
}

pub struct LayoutNode {
    subitem: LayoutNodeItem,
    size: LayoutNodeSize,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    pub fn new_row(&mut self) -> LayoutNodeId {
        let new_node_id = LayoutNodeId(self.nodes.len());
        self.nodes.push(LayoutNode {
            subitem: LayoutNodeItem::Row {
                children: Vec::new()
            },
            size: LayoutNodeSize::Flex(1.0),
        });
        new_node_id
    }

    pub fn new_column(&mut self) -> LayoutNodeId {
        let new_node_id = LayoutNodeId(self.nodes.len());
        self.nodes.push(LayoutNode {
            subitem: LayoutNodeItem::Column {
                children: Vec::new()
            },
            size: LayoutNodeSize::Flex(1.0),
        });
        new_node_id
    }

    pub fn new_leaf(&mut self) -> LayoutNodeId {
        let new_node_id = LayoutNodeId(self.nodes.len());
        self.nodes.push(LayoutNode {
            subitem: LayoutNodeItem::Leaf,
            size: LayoutNodeSize::Flex(1.0),
        });
        new_node_id
    }

    pub fn add_child(&mut self, parent_node_id: LayoutNodeId, child_node_id: LayoutNodeId) {
        let mut parent_node = self.node_mut(parent_node_id);
        match &mut parent_node.subitem {
            LayoutNodeItem::Row { children }
            | LayoutNodeItem::Column { children } => children.push(child_node_id),
            LayoutNodeItem::Leaf => panic!("add_child may not be called on Leaf type!"),
        }
    }

    pub fn resize(&mut self, node_id: LayoutNodeId, size: LayoutNodeSize) {
        self.node_mut(node_id).size = size;
    }

    pub fn organize(&self, width: f32, height: f32) -> Vec<Cell> {
        let mut result = Vec::with_capacity(self.nodes.len());
        //result.resize_with(self.nodes.len(), Cell::default);
        if self.nodes.len() > 0 {
            self.organize_node(&mut result, LayoutNodeId(0), width, height, 0.0, 0.0);
        }
        result
    }

    fn node(&self, node_id: LayoutNodeId) -> &LayoutNode {
        &self.nodes[node_id.0]
    }
    
    fn node_mut(&mut self, node_id: LayoutNodeId) -> &mut LayoutNode {
        &mut self.nodes[node_id.0]
    }

    fn organize_node(&self, result: &mut Vec<Cell>, node_id: LayoutNodeId, width: f32, height: f32, x: f32, y: f32) {

        /*
        result[node_id] = Cell {
            width,
            height,
            x,
            y,
        };
        */
        result.push(Cell {
            width,
            height,
            x,
            y,
        });

        match &self.node(node_id).subitem {
            LayoutNodeItem::Row { children } => {
                let distribution = self.distribute_space(&children, width);
                let mut curr_x = x;
                for (child_id, size) in children.iter().zip(distribution.iter()) {
                    self.organize_node(result, *child_id, *size, height, curr_x, y);
                    curr_x = curr_x + size;
                }
            },
            LayoutNodeItem::Column { children } => {
                let distribution = self.distribute_space(&children, height);
                let mut curr_y = y;
                for (child_id, size) in children.iter().zip(distribution.iter()) {
                    self.organize_node(result, *child_id, width, *size, x, curr_y);
                    curr_y = curr_y + size;
                }
            },
            LayoutNodeItem::Leaf {} => {
            },
        }
    }

    fn distribute_space(&self, node_ids: &Vec<LayoutNodeId>, total_space: f32) -> Vec<f32> {
        let mut total_fixed_space = 0.0;
        let mut total_flex_space = 0.0;
        for node_id in node_ids {
            let node = &self.node(*node_id);
            match node.size {
                LayoutNodeSize::Fixed(size) => total_fixed_space += size,
                LayoutNodeSize::Flex(size) => total_flex_space += size,
            }
        }
        let leftover_flex_space = total_space - total_fixed_space;

        let mut space_distributions = Vec::with_capacity(node_ids.len());
        for node_id in node_ids {
            let node = &self.node(*node_id);
            match node.size {
                LayoutNodeSize::Fixed(size) => space_distributions.push(size),
                LayoutNodeSize::Flex(mut size) => {
                    size = leftover_flex_space * (size/total_flex_space);
                    if size < 0.0 {
                        size = 0.0;
                    }
                    space_distributions.push(size);
                },
            }
        }

        space_distributions
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LayoutNodeId(usize);

pub enum LayoutNodeItem {
    Row {
        children: Vec<LayoutNodeId>,
    },
    Column {
        children: Vec<LayoutNodeId>,
    },
    Leaf,
}

pub enum LayoutNodeSize {
    Fixed(f32),
    Flex(f32),
}

#[derive(Default, Debug)]
pub struct Cell {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
}
