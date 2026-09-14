pub struct Layout {
    subitem: LayoutNodeItem,
    size: LayoutNodeSize,
}

impl Layout {
    pub fn organize(&self, width: f32, height: f32) -> Vec<Cell> {
        self.organize_node(width, height, 0.0, 0.0)
    }

    fn organize_node(&self, width: f32, height: f32, x: f32, y: f32) -> Vec<Cell> {
        let mut cells = vec![Cell {
            width,
            height,
            x,
            y,
        }];
        match &self.subitem {
            LayoutNodeItem::Row { children } => {
                let distribution = self.distribute_space(&children, width);
                let mut curr_x = x;
                for (child, size) in children.iter().zip(distribution.iter()) {
                    cells.append(&mut child.organize_node(*size, height, curr_x, y));
                    curr_x = curr_x + size;
                }
            },
            LayoutNodeItem::Column { children } => {
                let distribution = self.distribute_space(&children, height);
                let mut curr_y = y;
                for (child, size) in children.iter().zip(distribution.iter()) {
                    cells.append(&mut child.organize_node(width, *size, x, curr_y));
                    curr_y = curr_y + size;
                }
            },
            LayoutNodeItem::Leaf {} => {},
        }
        cells
    }

    fn distribute_space(&self, nodes: &Vec<Layout>, total_space: f32) -> Vec<f32> {
        let mut total_fixed_space = 0.0;
        let mut total_flex_space = 0.0;
        for node in nodes {
            match node.size {
                LayoutNodeSize::Fixed(size) => total_fixed_space += size,
                LayoutNodeSize::Flex(size) => total_flex_space += size,
            }
        }
        let leftover_flex_space = total_space - total_fixed_space;

        let mut space_distributions = Vec::with_capacity(nodes.len());
        for node in nodes {
            match node.size {
                LayoutNodeSize::Fixed(size) => space_distributions.push(size),
                LayoutNodeSize::Flex(mut size) => {
                    size = leftover_flex_space / (size/total_flex_space);
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

pub enum LayoutNodeItem {
    Row {
        children: Vec<Layout>,
    },
    Column {
        children: Vec<Layout>,
    },
    Leaf {
    },
}

pub enum LayoutNodeSize {
    Fixed(f32),
    Flex(f32),
}

pub struct Cell {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
}
