use lazy_static::lazy_static;

use crate::data::cut::Cut;

lazy_static! {
  pub static ref TABLE_1: Cut = Cut::read_str(include_str!("table1.cut"));
  pub static ref TABLE_2: Cut = Cut::read_str(include_str!("table2.cut"));
  pub static ref TABLE_3: Cut = Cut::read_str(include_str!("table3.cut"));
  pub static ref TABLE_4: Cut = Cut::read_str(include_str!("table4.cut"));
  pub static ref TABLE_4A: Cut = Cut::read_str(include_str!("table4a.cut"));
  pub static ref TABLE_5: Cut = Cut::read_str(include_str!("table5.cut"));
  pub static ref TABLE_6: Cut = Cut::read_str(include_str!("table6.cut"));
  pub static ref TABLE_9: Cut = Cut::read_str(include_str!("table9.cut"));
  pub static ref TABLE_10: Cut = Cut::read_str(include_str!("table10.cut"));
  pub static ref TABLE_11: Cut = Cut::read_str(include_str!("table11.cut"));
  pub static ref TABLE_12: Cut = Cut::read_str(include_str!("table12.cut"));
  pub static ref TABLE_13: Cut = Cut::read_str(include_str!("table13.cut"));
  pub static ref TABLE_14: Cut = Cut::read_str(include_str!("table14.cut"));
  pub static ref TABLE_15: Cut = Cut::read_str(include_str!("table15.cut"));
}
