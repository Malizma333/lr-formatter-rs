use crate::track::{
    group_builder_error::{GroupBuilderError, IntoGroupResult},
    group_feature_access::GroupFeatureAccess,
    properties::layer::{
        layer_base::{Layer, LayerBuilder, LayerBuilderError},
        layer_folder::{LayerFolder, LayerFolderBuilder, LayerFolderBuilderError},
    },
};
use derive_more::Display;
use getset::Getters;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Display, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LayerFeature {
    Name,
    Visible,
    Editable,
    Folders,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct LayerGroup {
    features: HashSet<LayerFeature>,
    layers: Vec<Layer>,
    layer_folders: Option<Vec<LayerFolder>>,
}

#[derive(Default)]
pub struct LayerGroupBuilder {
    features: HashSet<LayerFeature>,
    layers: Vec<LayerBuilder>,
    layer_folders: Option<Vec<LayerFolderBuilder>>,
}

#[derive(Debug, Error)]
pub enum LayerSubBuilderError {
    #[error("{0}")]
    Layer(#[from] LayerBuilderError),
    #[error("{0}")]
    LayerFolder(#[from] LayerFolderBuilderError),
}

pub type LayerGroupBuilderError = GroupBuilderError<LayerFeature, LayerSubBuilderError>;

impl GroupFeatureAccess<LayerFeature, LayerSubBuilderError> for LayerGroupBuilder {}

impl LayerGroupBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable_feature(&mut self, feature: LayerFeature) -> &mut Self {
        if feature == LayerFeature::Folders && self.layer_folders.is_none() {
            self.layer_folders = Some(vec![]);
        }

        self.features.insert(feature);
        self
    }

    pub fn add_layer(
        &mut self,
        id: u32,
        index: usize,
    ) -> Result<&mut LayerBuilder, LayerGroupBuilderError> {
        self.layers
            .push(LayerBuilder::default().id(id).index(index).to_owned());

        Ok(self.layers.last_mut().unwrap())
    }

    pub fn get_layers(&mut self) -> impl Iterator<Item = &mut LayerBuilder> {
        self.layers.iter_mut()
    }

    pub fn add_layer_folder(
        &mut self,
        id: u32,
        index: usize,
    ) -> Result<&mut LayerFolderBuilder, LayerGroupBuilderError> {
        let layer_folders = Self::require_feature(
            &self.features,
            LayerFeature::Folders,
            &mut self.layer_folders,
        )?;
        layer_folders.push(LayerFolderBuilder::default().id(id).index(index).to_owned());
        Ok(layer_folders.last_mut().unwrap())
    }

    pub fn get_layer_folders(
        &mut self,
    ) -> Result<impl Iterator<Item = &mut LayerFolderBuilder>, LayerGroupBuilderError> {
        let layer_folders = Self::require_feature(
            &self.features,
            LayerFeature::Folders,
            &mut self.layer_folders,
        )?;
        Ok(layer_folders.iter_mut())
    }

    pub fn build(&mut self) -> Result<LayerGroup, LayerGroupBuilderError> {
        let mut layers: Vec<Layer> = vec![];
        let mut layer_folders: Option<Vec<LayerFolder>> = None;

        for layer_builder in &self.layers {
            let layer = layer_builder.build().map_group_err()?;
            Self::check_feature(&self.features, LayerFeature::Name, &layer.name(), "name")?;
            Self::check_feature(
                &self.features,
                LayerFeature::Visible,
                &layer.visible(),
                "visible",
            )?;
            Self::check_feature(
                &self.features,
                LayerFeature::Editable,
                &layer.editable(),
                "editable",
            )?;
            Self::check_feature(
                &self.features,
                LayerFeature::Folders,
                &layer.folder_id(),
                "folder_id",
            )?;
            layers.push(layer);
        }

        Self::check_feature(
            &self.features,
            LayerFeature::Folders,
            &self.layer_folders,
            "layer_folders",
        )?;

        if let Some(layer_folder_builders) = &self.layer_folders {
            let mut some_layer_folders = vec![];
            for layer_folder_builder in layer_folder_builders {
                let layer_folder = layer_folder_builder.build().map_group_err()?;
                Self::check_feature(
                    &self.features,
                    LayerFeature::Name,
                    &layer_folder.name(),
                    "name",
                )?;
                Self::check_feature(
                    &self.features,
                    LayerFeature::Visible,
                    &layer_folder.visible(),
                    "visible",
                )?;
                Self::check_feature(
                    &self.features,
                    LayerFeature::Editable,
                    &layer_folder.editable(),
                    "editable",
                )?;
                some_layer_folders.push(layer_folder);
            }
            layer_folders = Some(some_layer_folders);
        }

        Ok(LayerGroup {
            features: self.features.clone(),
            layers,
            layer_folders,
        })
    }
}
