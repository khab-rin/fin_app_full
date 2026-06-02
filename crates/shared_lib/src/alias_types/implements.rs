use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::primitives::frozen::implements::*;
use crate::primitives::composite::implements::RasBicAcc;

pub type InnKppAccMap = HashMap<(CompInn, Kpp), HashSet<RasBicAcc>>;
pub type InnKppAccVec = Vec<((CompInn, Kpp), HashSet<RasBicAcc>)>;


pub type InsertData = (
    Vec<String>, 
    Vec<String>, 
    Vec<String>, 
    Vec<String>, 
    Vec<Value>
    );