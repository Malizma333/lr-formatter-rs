use std::collections::HashSet;

use derive_more::Display;
use getset::Getters;
use thiserror::Error;

use crate::track::{
    FeatureFieldAccess, UNREACHABLE_MESSAGE,
    properties::layer::{
        layer_base::{Layer, LayerBuilder, LayerBuilderError},
        layer_folder::{LayerFolder, LayerFolderBuilder, LayerFolderBuilderError},
    },
};

#[derive(Debug, Display, PartialEq, Eq, Hash)]
pub enum LayerFeature {
    Name,
    Visible,
    Editable,
    Folders,
}

#[derive(Debug, Getters)]
pub struct LayerGroup {
    #[getset(get = "pub")]
    layers: Vec<Layer>,
    #[getset(get = "pub")]
    layer_folders: Option<Vec<LayerFolder>>,
}

pub struct LayerGroupBuilder {
    features: HashSet<LayerFeature>,
    layers: Vec<LayerBuilder>,
    layer_folders: Option<Vec<LayerFolderBuilder>>,
}

impl Default for LayerGroupBuilder {
    fn default() -> Self {
        Self {
            features: HashSet::new(),
            layers: vec![],
            layer_folders: None,
        }
    }
}

impl FeatureFieldAccess<LayerFeature, LayerGroupBuilderError> for LayerGroupBuilder {
    fn require_feature<'a, F>(
        current_features: &HashSet<LayerFeature>,
        field: &'a mut Option<F>,
        feature: LayerFeature,
    ) -> Result<&'a mut F, LayerGroupBuilderError> {
        if !current_features.contains(&feature) {
            return Err(LayerGroupBuilderError::MissingFeatureFlag(feature));
        }

        match field.as_mut() {
            Some(some_field) => Ok(some_field),
            None => unreachable!("{}", UNREACHABLE_MESSAGE),
        }
    }

    fn check_feature<T>(
        &self,
        feature: LayerFeature,
        field: &Option<T>,
        attr_name: &'static str,
    ) -> Result<(), LayerGroupBuilderError> {
        if self.features.contains(&feature) && field.is_none() {
            return Err(LayerGroupBuilderError::MissingAttribute(attr_name));
        }

        if !self.features.contains(&feature) && field.is_some() {
            return Err(LayerGroupBuilderError::MissingFeatureFlag(feature));
        }

        Ok(())
    }
}

impl LayerGroupBuilder {
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
        let layer_folders = LayerGroupBuilder::require_feature(
            &mut self.features,
            &mut self.layer_folders,
            LayerFeature::Folders,
        )?;
        layer_folders.push(LayerFolderBuilder::default().id(id).index(index).to_owned());
        Ok(layer_folders.last_mut().unwrap())
    }

    pub fn get_layer_folders(
        &mut self,
    ) -> Result<impl Iterator<Item = &mut LayerFolderBuilder>, LayerGroupBuilderError> {
        let layer_folders = LayerGroupBuilder::require_feature(
            &mut self.features,
            &mut self.layer_folders,
            LayerFeature::Folders,
        )?;
        Ok(layer_folders.iter_mut())
    }

    pub fn build(&mut self) -> Result<LayerGroup, LayerGroupBuilderError> {
        let mut layers: Vec<Layer> = vec![];
        let mut layer_folders: Option<Vec<LayerFolder>> = None;

        for layer_builder in &self.layers {
            let layer = layer_builder.build()?;
            self.check_feature(LayerFeature::Name, &layer.name(), "name")?;
            self.check_feature(LayerFeature::Visible, &layer.visible(), "visible")?;
            self.check_feature(LayerFeature::Editable, &layer.editable(), "editable")?;
            self.check_feature(LayerFeature::Folders, &layer.folder_id(), "folder_id")?;
            layers.push(layer);
        }

        self.check_feature(LayerFeature::Folders, &self.layer_folders, "layer_folders")?;
        if let Some(layer_folder_builders) = &self.layer_folders {
            let mut some_layer_folders = vec![];
            for layer_folder_builder in layer_folder_builders {
                let layer_folder = layer_folder_builder.build()?;
                self.check_feature(LayerFeature::Name, &layer_folder.name(), "name")?;
                self.check_feature(LayerFeature::Visible, &layer_folder.visible(), "visible")?;
                self.check_feature(LayerFeature::Editable, &layer_folder.editable(), "editable")?;
                some_layer_folders.push(layer_folder);
            }
            layer_folders = Some(some_layer_folders);
        }

        Ok(LayerGroup {
            layers,
            layer_folders,
        })
    }
}

#[derive(Error, Debug)]
pub enum LayerGroupBuilderError {
    #[error("Expected feature to be registered before passing feature data: {0}")]
    MissingFeatureFlag(LayerFeature),
    #[error("Expected feature data to be present because feature was enabled: {0}")]
    MissingAttribute(&'static str),
    #[error("{0}")]
    LayerBuilderError(#[from] LayerBuilderError),
    #[error("{0}")]
    LayerFolderBuilderError(#[from] LayerFolderBuilderError),
}
