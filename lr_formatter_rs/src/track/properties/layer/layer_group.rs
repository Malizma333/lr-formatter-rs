use crate::track::{
    GroupBuilderBase,
    group_builder::{
        group_builder_base::GroupBuilder,
        group_builder_error::{GroupBuilderError, IntoGroupResult},
        group_builder_macro::define_group_builder,
    },
    properties::layer::{
        layer_base::{Layer, LayerBuilder, LayerBuilderError},
        layer_folder::{LayerFolder, LayerFolderBuilder, LayerFolderBuilderError},
    },
};
use std::collections::HashSet;

define_group_builder!(
  enum LayerFeature {
    Name,
    Visible,
    Editable,
    Folders,
  }

  struct LayerGroup {
    layers: Vec<Layer>, Vec<LayerBuilder>, LayerBuilderError,
    layer_folders: Option<Vec<LayerFolder>>, Option<Vec<LayerFolderBuilder>>, LayerFolderBuilderError,
  }
);

impl GroupBuilder for LayerGroupBuilder {
    fn on_enable_feature(&mut self, feature: Self::Feature) {
        if feature == LayerFeature::Folders && self.layer_folders.is_none() {
            self.layer_folders = Some(vec![]);
        }
    }

    fn build(&mut self) -> Result<Self::Output, GroupBuilderError<Self::Feature, Self::SubError>> {
        let mut layers: Vec<Layer> = vec![];
        let mut layer_folders: Option<Vec<LayerFolder>> = None;

        for layer_builder in &self.layers {
            let layer = layer_builder.build().map_group_err()?;
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
                let layer_folder = layer_folder_builder.build().map_group_err()?;
                self.check_feature(LayerFeature::Name, &layer_folder.name(), "name")?;
                self.check_feature(LayerFeature::Visible, &layer_folder.visible(), "visible")?;
                self.check_feature(LayerFeature::Editable, &layer_folder.editable(), "editable")?;
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

impl LayerGroupBuilder {
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
}
