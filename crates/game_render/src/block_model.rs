//! 3D-модели блоков из ресурс-паков: подмножество формата JSON-моделей Minecraft/OptiFine.
//!
//! ```json
//! { "textures": { "top": "crafting_table_top", "side": "crafting_table_side" },
//!   "elements": [ { "from": [0,0,0], "to": [16,14,16],
//!       "faces": { "up": {"texture": "#top"}, "north": {"texture": "#side", "uv": [0,2,16,16]} } } ] }
//! ```
//! Координаты в 1/16 блока (0…16). Поддержаны кубоиды, `uv`, переменные `#имя`.
//! Вращение элементов и `display` пока игнорируются/отклоняются явно.
use crate::MATERIAL_COUNT;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ModelFace {
    /// 0=X, 1=Y, 2=Z.
    pub axis: usize,
    pub positive: bool,
    /// Слой атласа текстур.
    pub layer: u8,
    /// `[u0, v0, u1, v1]` в пикселях 0…16, v вниз.
    pub uv: [f32; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModelElement {
    pub from: [f32; 3],
    pub to: [f32; 3],
    pub faces: Vec<ModelFace>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BlockModel {
    pub elements: Vec<ModelElement>,
}

/// Модели по ID блока. Блок с моделью не считается непрозрачным кубом.
#[derive(Clone, Debug, Default)]
pub struct BlockModels(pub HashMap<u8, BlockModel>);

impl BlockModels {
    #[must_use]
    pub fn get(&self, block: u8) -> Option<&BlockModel> {
        self.0.get(&block)
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawModel {
    #[serde(default)]
    textures: HashMap<String, String>,
    elements: Vec<RawElement>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawElement {
    from: [f32; 3],
    to: [f32; 3],
    faces: HashMap<String, RawFace>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFace {
    texture: String,
    #[serde(default)]
    uv: Option<[f32; 4]>,
}

const MAX_ELEMENTS: usize = 256;

impl BlockModel {
    /// `layer_of` переводит имя текстуры (`crafting_table_top`) в слой атласа.
    pub fn from_json(
        source: &str,
        layer_of: impl Fn(&str) -> Option<usize>,
    ) -> Result<Self, String> {
        let raw: RawModel = serde_json::from_str(source).map_err(|e| e.to_string())?;
        if raw.elements.is_empty() || raw.elements.len() > MAX_ELEMENTS {
            return Err(format!("нужно от 1 до {MAX_ELEMENTS} элементов"));
        }
        let mut elements = Vec::new();
        for element in raw.elements {
            let in_range =
                |v: &[f32; 3]| v.iter().all(|c| c.is_finite() && (0.0..=16.0).contains(c));
            if !in_range(&element.from)
                || !in_range(&element.to)
                || (0..3).any(|a| element.from[a] > element.to[a])
            {
                return Err("from/to должны лежать в 0…16 и from ≤ to".into());
            }
            let mut faces = Vec::new();
            let mut names: Vec<_> = element.faces.keys().cloned().collect();
            names.sort();
            for name in names {
                let face = &element.faces[&name];
                let (axis, positive) = match name.as_str() {
                    "east" => (0, true),
                    "west" => (0, false),
                    "up" => (1, true),
                    "down" => (1, false),
                    "south" => (2, true),
                    "north" => (2, false),
                    other => return Err(format!("неизвестная грань «{other}»")),
                };
                let texture = match face.texture.strip_prefix('#') {
                    Some(variable) => raw
                        .textures
                        .get(variable)
                        .ok_or_else(|| format!("не задана переменная «#{variable}»"))?
                        .as_str(),
                    None => face.texture.as_str(),
                };
                let layer = layer_of(texture)
                    .filter(|layer| *layer < MATERIAL_COUNT)
                    .ok_or_else(|| format!("неизвестная текстура «{texture}»"))?;
                let (u_axis, v_axis) = face_axes(axis);
                // Как у обычных граней: u растёт вдоль u_axis, v направлен вниз.
                let uv = face.uv.unwrap_or([
                    element.from[u_axis],
                    16.0 - element.to[v_axis],
                    element.to[u_axis],
                    16.0 - element.from[v_axis],
                ]);
                if uv
                    .iter()
                    .any(|c| !c.is_finite() || !(0.0..=16.0).contains(c))
                {
                    return Err("uv должны лежать в 0…16".into());
                }
                faces.push(ModelFace {
                    axis,
                    positive,
                    layer: layer as u8,
                    uv,
                });
            }
            elements.push(ModelElement {
                from: element.from,
                to: element.to,
                faces,
            });
        }
        Ok(Self { elements })
    }
}

/// Те же оси, что у жадного меша: (u, v) для граней по X, Y, Z.
pub(crate) fn face_axes(axis: usize) -> (usize, usize) {
    match axis {
        0 => (2, 1),
        1 => (0, 2),
        _ => (0, 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn layers(name: &str) -> Option<usize> {
        match name {
            "a" => Some(3),
            "b" => Some(7),
            _ => None,
        }
    }
    #[test]
    fn parses_variables_default_uv_and_rejects_bad_input() {
        let model = BlockModel::from_json(
            r##"{"textures":{"t":"a"},"elements":[{"from":[0,0,0],"to":[16,8,16],
               "faces":{"up":{"texture":"#t"},"north":{"texture":"b","uv":[0,8,16,16]}}}]}"##,
            layers,
        )
        .unwrap();
        let faces = &model.elements[0].faces;
        assert_eq!(faces.len(), 2);
        let up = faces.iter().find(|f| f.axis == 1).unwrap();
        assert_eq!(
            (up.layer, up.positive, up.uv),
            (3, true, [0.0, 0.0, 16.0, 16.0])
        );
        let north = faces.iter().find(|f| f.axis == 2).unwrap();
        assert_eq!(
            (north.layer, north.positive, north.uv),
            (7, false, [0.0, 8.0, 16.0, 16.0])
        );
        for bad in [
            r#"{"elements":[]}"#,
            r#"{"elements":[{"from":[0,0,0],"to":[17,1,1],"faces":{}}]}"#,
            r#"{"elements":[{"from":[4,0,0],"to":[1,1,1],"faces":{}}]}"#,
            r#"{"elements":[{"from":[0,0,0],"to":[1,1,1],"faces":{"up":{"texture":"zzz"}}}]}"#,
            r##"{"elements":[{"from":[0,0,0],"to":[1,1,1],"faces":{"up":{"texture":"#x"}}}]}"##,
            r#"{"elements":[{"from":[0,0,0],"to":[1,1,1],"faces":{"left":{"texture":"a"}}}]}"#,
            r#"{"elements":[{"from":[0,0,0],"to":[1,1,1],"rotation":{},"faces":{}}]}"#,
        ] {
            assert!(BlockModel::from_json(bad, layers).is_err(), "{bad}");
        }
    }
}
