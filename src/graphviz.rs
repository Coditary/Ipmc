use crate::models::{Goal, Actor, Impact, Deliverable};

// ==========================================
// 🎨 THEME & LAYOUT KONFIGURATION
// ==========================================

const CELL_PAD: &str = "10"; 
const FONT_MAIN: &str = "Helvetica, Arial, sans-serif";

const C_GOAL_BG: &str = "#8b5cf6";
const C_GOAL_BORDER: &str = "#4c1d95";
const C_ACTOR_BG: &str = "#3b82f6";
const C_ACTOR_BORDER: &str = "#1e3a8a";
const C_IMPACT_BG: &str = "#10b981";
const C_IMPACT_BORDER: &str = "#064e3b";
const C_DELIV_BG: &str = "#f59e0b";
const C_DELIV_BORDER: &str = "#78350f";

const C_BG_LIGHT: &str = "#ffffff";
const C_BG_FOOTER: &str = "#f8fafc";
const C_LEGEND_BORDER: &str = "#cbd5e1";
const C_TEXT_MUTED: &str = "#475569";
const C_TEXT_LIGHT: &str = "#94a3b8";

// ==========================================
// HILFSFUNKTIONEN
// ==========================================

fn escape_html(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace(">=", "&ge;")
        .replace("<=", "&le;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

fn generate_footer(stats: &str, id_opt: Option<&String>) -> String {
    let id_str = match id_opt {
        Some(id) => format!(r#"<font color="{C_TEXT_LIGHT}">#{id}</font>"#),
        None => String::from(""),
    };

    format!(r#"
        <table border="0" cellborder="0" cellspacing="0" cellpadding="0">
            <tr>
                <td align="left"><font point-size="10" color="{C_TEXT_MUTED}">{stats}</font></td>
                <td align="right"><font point-size="9">{id_str}</font></td>
            </tr>
        </table>
    "#)
}

// ==========================================
// NODE-GENERATOREN
// ==========================================

fn generate_goal_node(goal: &Goal) -> String {
    let mut metrics_html = String::new();
    for m in &goal.metrics {
        metrics_html.push_str(&format!(r#"
            <tr><td align="left" width="200"><font point-size="11"><b>{name}</b>: {target}</font></td></tr>
            <tr><td align="left" width="200"><font point-size="9" color="{C_TEXT_MUTED}"><i>{desc}</i></font></td></tr>
            <tr><td><font point-size="4">&nbsp;</font></td></tr>
        "#, name = escape_html(&m.name), target = escape_html(&m.target), desc = escape_html(&m.desc)));
    }

    format!(r#"
    "{goal_id}" [label=<
        <table border="0" cellborder="1" cellspacing="0" cellpadding="8" color="{C_GOAL_BORDER}">
            <tr><td bgcolor="{C_GOAL_BG}"><font color="white"><b>{name}</b></font></td></tr>
            <tr><td bgcolor="{C_BG_LIGHT}">
                <table border="0" cellborder="0" cellspacing="0" cellpadding="2">
                    {metrics_html}
                </table>
            </td></tr>
            <tr><td bgcolor="{C_BG_FOOTER}" align="right"><font point-size="9" color="{C_TEXT_LIGHT}">#{esc_id}</font></td></tr>
        </table>
    >];
    "#, goal_id = goal.id, name = escape_html(&goal.name), esc_id = escape_html(&goal.id), metrics_html = metrics_html)
}

fn generate_actor_node(actor: &Actor, node_id: &str) -> String {
    let stats = format!("Prio: {} &nbsp;&nbsp;&nbsp;", actor.prio);
    
    format!(r#"
    "{node_id}" [label=<
        <table border="0" cellborder="1" cellspacing="0" cellpadding="{CELL_PAD}" color="{C_ACTOR_BORDER}">
            <tr><td bgcolor="{C_ACTOR_BG}" align="center"><font color="white"><b>{name}</b></font></td></tr>
            <tr><td align="left" bgcolor="{C_BG_FOOTER}">{footer}</td></tr>
        </table>
    >];
    "#, name = escape_html(&actor.name), footer = generate_footer(&stats, actor.id.as_ref()))
}

fn generate_impact_node(impact: &Impact, node_id: &str) -> String {
    let stats = format!("Prio: {} &nbsp;&bull;&nbsp; SP: {}", impact.prio, impact.sp);
    
    format!(r#"
    "{node_id}" [label=<
        <table border="0" cellborder="1" cellspacing="0" cellpadding="8" color="{C_IMPACT_BORDER}">
            <tr><td bgcolor="{C_IMPACT_BG}"><font color="white"><b>{name}</b></font></td></tr>
            <tr><td bgcolor="{C_BG_LIGHT}">
                <table border="0" cellborder="0" cellspacing="0" cellpadding="2">
                    <tr><td align="left" width="200"><font point-size="10" color="{C_TEXT_MUTED}">Target:</font> <b>{target}</b></td></tr>
                </table>
            </td></tr>
            <tr><td bgcolor="{C_BG_FOOTER}" align="left">{footer}</td></tr>
        </table>
    >];
    "#, name = escape_html(&impact.name), target = escape_html(&impact.targets), footer = generate_footer(&stats, impact.id.as_ref()))
}

fn generate_deliverable_node(del: &Deliverable) -> String {
    let stats = format!("Prio: {} &nbsp;&bull;&nbsp; SP: {}", del.prio, del.sp);
    format!(r#"
    "{del_id}" [label=<
        <table border="0" cellborder="1" cellspacing="0" cellpadding="{CELL_PAD}" color="{C_DELIV_BORDER}">
            <tr><td bgcolor="{C_DELIV_BG}" align="center"><font color="white"><b>{name}</b></font></td></tr>
            <tr><td align="left" bgcolor="{C_BG_FOOTER}">{footer}</td></tr>
        </table>
    >];
    "#, del_id = del.id, name = escape_html(&del.name), footer = generate_footer(&stats, Some(&del.id)))
}

pub fn generate_dot(goal: &Goal) -> String {
    let mut dot = format!(
        "digraph ImpactMap {{\n rankdir=LR;\n nodesep=0.5;\n ranksep=0.8;\n splines=spline;\n \
         node [shape=none, fontname=\"{FONT_MAIN}\", margin=0];\n \
         edge [color=\"{C_TEXT_LIGHT}\", penwidth=1.5, arrowsize=0.8];\n\n\
         \"Legend\" [label=<<table border=\"0\" cellborder=\"1\" cellspacing=\"0\" cellpadding=\"6\" color=\"{C_LEGEND_BORDER}\">\
         <tr><td bgcolor=\"{C_BG_FOOTER}\"><b>Legende:</b></td><td bgcolor=\"{C_GOAL_BG}\"><font color=\"white\">Goal</font></td>\
         <td bgcolor=\"{C_ACTOR_BG}\"><font color=\"white\">Actor</font></td><td bgcolor=\"{C_IMPACT_BG}\"><font color=\"white\">Impact</font></td>\
         <td bgcolor=\"{C_DELIV_BG}\"><font color=\"white\">Deliverable</font></td></tr></table>>];\n\
         \"Legend\" -> \"{gid}\" [style=invis]; {{ rank=same; \"Legend\"; \"{gid}\" }}\n\n",
        gid = goal.id
    );

    dot.push_str(&generate_goal_node(goal));

    for (a_idx, actor) in goal.actors.iter().enumerate() {
        let actor_node_id = format!("{}_A{}", goal.id, a_idx);
        dot.push_str(&generate_actor_node(actor, &actor_node_id));
        dot.push_str(&format!(" \"{}\" -> \"{}\";\n", goal.id, actor_node_id));

        for (i_idx, impact) in actor.impacts.iter().enumerate() {
            let impact_node_id = format!("{}_I{}", actor_node_id, i_idx);
            dot.push_str(&generate_impact_node(impact, &impact_node_id));
            dot.push_str(&format!(" \"{}\" -> \"{}\";\n", actor_node_id, impact_node_id));

            for del in &impact.deliverables {
                dot.push_str(&generate_deliverable_node(del));
                dot.push_str(&format!(" \"{}\" -> \"{}\";\n", impact_node_id, del.id));
            }
        }
    }
    dot.push_str("}\n");
    dot
}
