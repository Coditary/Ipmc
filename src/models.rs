#[derive(Debug)]
pub struct Goal {
    pub name: String,
    pub id: String,
    pub metrics: Vec<Metric>,
    pub actors: Vec<Actor>,
}

#[derive(Debug)]
pub struct Metric {
    pub name: String,
    pub target: String,
    pub desc: String,
}

#[derive(Debug)]
pub struct Actor {
    pub name: String,
    pub id: Option<String>, // Neu: Optionale ID
    pub prio: i64,
    pub impacts: Vec<Impact>,
}

#[derive(Debug)]
pub struct Impact {
    pub name: String,
    pub id: Option<String>, // Neu: Optionale ID
    pub targets: String,
    pub prio: i64,
    pub sp: i64,
    pub deliverables: Vec<Deliverable>,
}

#[derive(Debug)]
pub struct Deliverable {
    pub name: String,
    pub id: String,
    pub prio: i64,
    pub sp: i64,
}
