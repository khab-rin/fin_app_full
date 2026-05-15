use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::primitives::frozen::implements::*;
use crate::primitives::composite::implements::RasBicAcc;

pub type InnKppAccMap = HashMap<(Inn, Kpp), HashSet<RasBicAcc>>;
pub type InnKppAccVec = Vec<((Inn, Kpp), HashSet<RasBicAcc>)>;


pub type InsertData = (
    Vec<String>, 
    Vec<String>, 
    Vec<String>, 
    Vec<String>, 
    Vec<Value>
    );