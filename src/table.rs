
/**
 * テーブルモデル
 * Created by Atsushi Miyake on 2017/05/09.
 * Copyright © 2017年 Atsushi Miyake. All rights reserved.
 **/

use std::sync::Mutex;

pub struct Table {
    pub forks: Vec<Mutex<()>>
}