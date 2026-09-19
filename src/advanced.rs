//! Serializable Beta 2/3 authoring contracts. These are deliberately data-only:
//! the runtime and egui editor can consume the same files without hidden state.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CharacterSet {
    pub idle: DirectionFrames,
    pub walk: DirectionFrames,
    pub palette: Vec<[u8; 4]>,
    pub portrait: Option<String>,
}
impl Default for CharacterSet {
    fn default() -> Self {
        Self {
            idle: DirectionFrames {
                up: vec![0],
                down: vec![0],
                left: vec![0],
                right: vec![0],
            },
            walk: DirectionFrames {
                up: vec![0, 1],
                down: vec![0, 1],
                left: vec![2, 3],
                right: vec![2, 3],
            },
            palette: vec![[32, 28, 44, 255], [235, 243, 248, 255], [225, 70, 134, 255]],
            portrait: None,
        }
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DirectionFrames {
    pub up: Vec<usize>,
    pub down: Vec<usize>,
    pub left: Vec<usize>,
    pub right: Vec<usize>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub pixels: Vec<[u8; 4]>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Choice {
    pub label: String,
    pub next: usize,
    pub condition: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DialogueNode {
    pub speaker: String,
    pub text: String,
    pub choices: Vec<Choice>,
    pub hooks: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SceneHook {
    pub event: String,
    pub actions: Vec<String>,
    pub condition: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LogicNode {
    pub id: String,
    pub kind: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ParticleEmitter {
    pub name: String,
    pub rate: f32,
    pub lifetime: f32,
    pub color: [u8; 4],
    pub wind: [f32; 2],
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ShaderGraph {
    pub nodes: Vec<LogicNode>,
    pub source: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PluginManifest {
    pub id: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub entry: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct KeyboardBindings {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
    pub interact: String,
    pub play: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AccessibilityAudit {
    pub labels: Vec<String>,
    pub keyboard_complete: bool,
    pub contrast_checked: bool,
}

impl CharacterSet {
    pub fn frame(&self, direction: [i32; 2], moving: bool, tick: usize) -> Option<usize> {
        let set = if moving { &self.walk } else { &self.idle };
        let frames = if direction[1] < 0 {
            &set.up
        } else if direction[0] < 0 {
            &set.left
        } else if direction[0] > 0 {
            &set.right
        } else {
            &set.down
        };
        (!frames.is_empty()).then(|| frames[tick % frames.len()])
    }
}

pub fn composite_layers(layers: &[Layer]) -> Result<Vec<[u8; 4]>, String> {
    let mut output = vec![[0, 0, 0, 0]; 16 * 24];
    for layer in layers.iter().filter(|l| l.visible) {
        if layer.pixels.len() != output.len()
            || !layer.opacity.is_finite()
            || !(0.0..=1.0).contains(&layer.opacity)
        {
            return Err("Invalid sprite layer".into());
        }
        for (dst, src) in output.iter_mut().zip(&layer.pixels) {
            let alpha = src[3] as f32 / 255.0 * layer.opacity;
            let inverse = 1.0 - alpha;
            for c in 0..3 {
                dst[c] = (src[c] as f32 * alpha + dst[c] as f32 * inverse).round() as u8;
            }
            dst[3] = ((alpha + dst[3] as f32 / 255.0 * inverse) * 255.0).round() as u8;
        }
    }
    Ok(output)
}

pub fn validate_dialogue(nodes: &[DialogueNode]) -> Result<(), String> {
    for (i, node) in nodes.iter().enumerate() {
        for choice in &node.choices {
            if choice.next >= nodes.len() {
                return Err(format!("Dialogue node {i} points outside graph"));
            }
            if choice.label.trim().is_empty() {
                return Err("Dialogue choice label is empty".into());
            }
        }
    }
    Ok(())
}
pub fn validate_plugin(plugin: &PluginManifest) -> Result<(), String> {
    if plugin.id.is_empty() || plugin.version.is_empty() || plugin.entry.is_empty() {
        return Err("Plugin manifest needs id, version, and entry".into());
    }
    if plugin
        .capabilities
        .iter()
        .any(|c| c == "filesystem" || c == "network")
    {
        return Err("Privileged plugin capabilities require explicit host approval".into());
    }
    Ok(())
}
pub fn lua_export(nodes: &[LogicNode]) -> String {
    let mut out =
        String::from("-- Generated by OMASPRITE; review before running\nreturn function(event)\n");
    for node in nodes {
        out.push_str(&format!("  -- node {} ({})\n", node.id, node.kind));
    }
    out.push_str("end\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dialogue_choices_validate_and_reject_bad_edges() {
        let nodes = vec![DialogueNode {
            speaker: "Mira".into(),
            text: "Go?".into(),
            choices: vec![Choice {
                label: "Yes".into(),
                next: 0,
                condition: Some("beacon == false".into()),
            }],
            hooks: vec!["open_gate".into()],
        }];
        assert!(validate_dialogue(&nodes).is_ok());
        let mut bad = nodes;
        bad[0].choices[0].next = 9;
        assert!(validate_dialogue(&bad).is_err());
    }
    #[test]
    fn plugin_validation_blocks_privileged_capabilities() {
        let plugin = PluginManifest {
            id: "demo".into(),
            version: "1".into(),
            capabilities: vec!["network".into()],
            entry: "main".into(),
        };
        assert!(validate_plugin(&plugin).is_err());
    }
    #[test]
    fn lua_export_is_deterministic_and_reviewable() {
        let lua = lua_export(&[LogicNode {
            id: "start".into(),
            kind: "event".into(),
            inputs: vec![],
            outputs: vec![],
        }]);
        assert!(lua.contains("Generated by OMASPRITE"));
        assert!(lua.contains("start"));
    }
    #[test]
    fn directional_frames_and_layers_are_deterministic() {
        let set = CharacterSet {
            walk: DirectionFrames {
                right: vec![2, 3],
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(set.frame([1, 0], true, 3), Some(3));
        let layers = vec![
            Layer {
                name: "base".into(),
                visible: true,
                opacity: 1.0,
                pixels: vec![[10, 20, 30, 255]; 384],
            },
            Layer {
                name: "tint".into(),
                visible: true,
                opacity: 0.5,
                pixels: vec![[110, 20, 30, 255]; 384],
            },
        ];
        assert_eq!(composite_layers(&layers).unwrap()[0], [60, 20, 30, 255]);
    }
}
