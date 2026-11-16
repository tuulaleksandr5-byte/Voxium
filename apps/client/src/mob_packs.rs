//! User-supplied CEM geometry converted offline into a native Bevy hierarchy.
use super::*;

#[derive(Deserialize)]
struct PackModel {
    kind: String,
    variant: String,
    texture: [u32; 2],
    height: f32,
    skin: String,
    emissive: Option<String>,
    parts: Vec<PackPart>,
}

#[derive(Deserialize)]
struct PackPart {
    name: String,
    parent: Option<usize>,
    translation: [f32; 3],
    rotation: [f32; 3],
    scale: [f32; 3],
    role: String,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

fn definitions() -> Result<Vec<PackModel>, String> {
    serde_json::from_str(crate::pack_assets::MODELS).map_err(|error| error.to_string())
}

fn semantic_role(name: &str) -> Role {
    if let Some(value) = name.strip_prefix("spider_")
        && let Some((side, index)) = value.rsplit_once('_')
        && let Ok(index) = index.parse::<usize>()
    {
        return Role::PackSpiderLeg(if side == "left" { 1.0 } else { -1.0 }, index);
    }
    match name {
        "head" => Role::Head,
        "body" => Role::Body,
        "leg_left" => Role::Leg(1.0),
        "leg_right" => Role::Leg(-1.0),
        "arm_left" => Role::Arm(1.0),
        "arm_right" => Role::Arm(-1.0),
        "wing_left" => Role::Wing(1.0),
        "wing_right" => Role::Wing(-1.0),
        "tail" => Role::Tail,
        _ => Role::Fixed,
    }
}

fn image(name: &str, images: &mut Assets<Image>) -> Result<Handle<Image>, String> {
    let bytes = crate::pack_assets::file(name).ok_or_else(|| format!("нет ресурса {name}"))?;
    let image = Image::from_buffer(
        bytes,
        bevy::image::ImageType::Extension("png"),
        bevy::image::CompressedImageFormats::NONE,
        true,
        ImageSampler::nearest(),
        RenderAssetUsages::default(),
    )
    .map_err(|error| format!("{name}: {error}"))?;
    Ok(images.add(image))
}

pub(super) fn load(
    meshes: &mut Assets<Mesh>,
    images: &mut Assets<Image>,
    materials: &mut Assets<StandardMaterial>,
) -> Result<HashMap<MobKind, Vec<MobModel>>, String> {
    let mut models: HashMap<MobKind, Vec<MobModel>> = HashMap::new();
    for definition in definitions()? {
        let kind = MobKind::ALL
            .into_iter()
            .find(|kind| kind.spec().id == definition.kind)
            .ok_or_else(|| format!("неизвестный моб {}", definition.kind))?;
        let texture = image(&definition.skin, images)?;
        let emissive_texture = definition
            .emissive
            .as_deref()
            .map(|name| image(name, images))
            .transpose()?;
        let material = |tint: Color, materials: &mut Assets<StandardMaterial>| {
            materials.add(StandardMaterial {
                base_color: tint,
                base_color_texture: Some(texture.clone()),
                alpha_mode: AlphaMode::Mask(0.1),
                cull_mode: None,
                double_sided: true,
                emissive_texture: emissive_texture.clone(),
                emissive: if emissive_texture.is_some() {
                    LinearRgba::rgb(2.0, 2.0, 2.0)
                } else {
                    LinearRgba::BLACK
                },
                perceptual_roughness: 0.95,
                reflectance: 0.08,
                ..default()
            })
        };
        let mut parts = Vec::new();
        for part in definition.parts {
            if part.parent.is_some_and(|parent| parent >= parts.len()) {
                return Err(format!("некорректный родитель части {}", part.name));
            }
            let mesh = (!part.indices.is_empty()).then(|| {
                meshes.add(
                    Mesh::new(
                        PrimitiveTopology::TriangleList,
                        RenderAssetUsages::default(),
                    )
                    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, part.positions)
                    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, part.normals)
                    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, part.uvs)
                    .with_inserted_indices(Indices::U32(part.indices)),
                )
            });
            parts.push(PartModel {
                role: semantic_role(&part.role),
                parent: part.parent,
                pivot: Vec3::from_array(part.translation),
                rotation: Quat::from_rotation_z(part.rotation[2].to_radians())
                    * Quat::from_rotation_y(part.rotation[1].to_radians())
                    * Quat::from_rotation_x(part.rotation[0].to_radians()),
                scale: Vec3::from_array(part.scale),
                mesh,
            });
        }
        debug!(kind = definition.kind, variant = definition.variant, texture = ?definition.texture, "Загружена пользовательская модель");
        models.entry(kind).or_default().push(MobModel {
            parts,
            material: material(Color::WHITE, materials),
            hurt: material(Color::srgb(1.0, 0.38, 0.34), materials),
            scale: kind.spec().size[1] / definition.height,
        });
    }
    Ok(models)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supplied_models_have_resolvable_skins_valid_meshes_and_acyclic_rigs() {
        let definitions = definitions().unwrap();
        for id in [
            "wolf",
            "skeleton",
            "ender_dragon",
            "wither",
            "cow",
            "sheep",
            "pig",
            "chicken",
            "rabbit",
            "mooshroom",
            "zombie",
            "spider",
            "creeper",
        ] {
            assert!(definitions.iter().any(|model| model.kind == id), "{id}");
        }
        for model in definitions {
            assert!(model.height.is_finite() && model.height > 0.0);
            let skin =
                image::load_from_memory(crate::pack_assets::file(&model.skin).unwrap()).unwrap();
            assert_eq!([skin.width(), skin.height()], model.texture);
            for (index, part) in model.parts.iter().enumerate() {
                assert!(
                    part.parent.is_none_or(|parent| parent < index),
                    "{} {}",
                    model.kind,
                    part.name
                );
                assert_eq!(part.positions.len(), part.normals.len());
                assert_eq!(part.positions.len(), part.uvs.len());
                assert!(
                    part.indices
                        .iter()
                        .all(|i| (*i as usize) < part.positions.len())
                );
                assert!(part.positions.iter().flatten().all(|n| n.is_finite()));
                assert!(
                    part.uvs.iter().flatten().all(|n| (0.0..=1.0).contains(n)),
                    "{} {}",
                    model.kind,
                    part.name
                );
            }
        }
    }
}
