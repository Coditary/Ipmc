use crate::models::*;
use anyhow::{Context, Result};
use kdl::KdlDocument;

pub fn parse_kdl(input: &str) -> Result<Vec<Goal>> {
    let doc: KdlDocument = input.parse().context("Fehler beim Parsen der KDL-Datei")?;
    let mut goals = Vec::new();

    for node in doc.nodes() {
        if node.name().value() == "goal" {
            let name = node.get(0).unwrap().value().as_string().unwrap().to_string();
            let id = node.get("id").unwrap().value().as_string().unwrap().to_string();
            
            let mut metrics = Vec::new();
            let mut actors = Vec::new();

            if let Some(children) = node.children() {
                for child in children.nodes() {
                    match child.name().value() {
                        "metric" => {
                            metrics.push(Metric {
                                name: child.get(0).unwrap().value().as_string().unwrap().to_string(),
                                target: child.get("target").unwrap().value().as_string().unwrap().to_string(),
                                desc: child.get("desc").unwrap().value().as_string().unwrap().to_string(),
                            });
                        }
                        "actor" => {
                            let actor_name = child.get(0).unwrap().value().as_string().unwrap().to_string();
                            // HIER NEU: Optionale ID auslesen
                            let actor_id = child.get("id").map(|n| n.value().as_string().unwrap().to_string());
                            let prio = child.get("prio").unwrap().value().as_i64().unwrap_or(0);
                            let mut impacts = Vec::new();

                            if let Some(actor_children) = child.children() {
                                for impact_node in actor_children.nodes() {
                                    if impact_node.name().value() == "impact" {
                                        let impact_name = impact_node.get(0).unwrap().value().as_string().unwrap().to_string();
                                        // HIER NEU: Optionale ID auslesen
                                        let impact_id = impact_node.get("id").map(|n| n.value().as_string().unwrap().to_string());
                                        let targets = impact_node.get("targets").unwrap().value().as_string().unwrap().to_string();
                                        let iprio = impact_node.get("prio").unwrap().value().as_i64().unwrap_or(0);
                                        let isp = impact_node.get("sp").unwrap().value().as_i64().unwrap_or(0);
                                        
                                        let mut deliverables = Vec::new();
                                        if let Some(impact_children) = impact_node.children() {
                                            for del_node in impact_children.nodes() {
                                                if del_node.name().value() == "deliverable" {
                                                    deliverables.push(Deliverable {
                                                        name: del_node.get(0).unwrap().value().as_string().unwrap().to_string(),
                                                        id: del_node.get("id").unwrap().value().as_string().unwrap().to_string(),
                                                        prio: del_node.get("prio").unwrap().value().as_i64().unwrap_or(0),
                                                        sp: del_node.get("sp").unwrap().value().as_i64().unwrap_or(0),
                                                    });
                                                }
                                            }
                                        }
                                        impacts.push(Impact { name: impact_name, id: impact_id, targets, prio: iprio, sp: isp, deliverables });
                                    }
                                }
                            }
                            actors.push(Actor { name: actor_name, id: actor_id, prio, impacts });
                        }
                        _ => {}
                    }
                }
            }
            goals.push(Goal { name, id, metrics, actors });
        }
    }
    Ok(goals)
}
