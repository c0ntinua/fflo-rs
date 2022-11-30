use crate::settings::*;

pub struct Layer {
    pub rows : u32,
    pub cols : u32,
    pub cells : Vec<f64>,
}

pub fn random_layer(settings : &Settings) -> Layer {
	let mut cells = vec![0.0;(settings.abstract_rows*settings.abstract_cols) as usize];
	for x in cells.iter_mut() {
		*x = 1.0 - 2.0*rand::random::<f64>();
	}
    Layer {
        rows : settings.abstract_rows,
        cols : settings.abstract_cols,
        cells,
    }
}
pub fn random_layers(settings : &Settings) -> Vec<Layer> {
	let mut layers = vec![];
    for l in 0..settings.num_layers {
        layers.push(random_layer(settings));
    }
    layers
}

impl Layer {
    pub fn get(&self , row_index : i64, col_index : i64) -> f64 {
        let mut row =  row_index;
        let mut col =  col_index; 
        while row < 0 { row += self.rows as i64};
        while row > (self.rows as i64) - 1 { row -= self.rows as i64 };
        while col < 0 { col += (self.cols as i64)};
        while col > (self.cols as i64) - 1 { col -= self.cols as i64};
        // debug_assert!(0 <= row && row < rows);
        // debug_assert!(0 <= col && col < self.abstract_cols);
        self.cells[((row as u32)*self.cols+(col as u32)) as usize]
    }
}


