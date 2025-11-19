use crate::track::{
    GroupBuilderBase,
    group_builder::{
        group_builder_base::GroupBuilder,
        group_builder_error::{GroupBuilderError, IntoGroupResult},
        group_builder_macro::define_group_builder,
    },
    groups::layer::{
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
    fn build_group(&mut self) -> Result<Self::Output, GroupBuilderError<Self::SubError>> {
        let mut layers: Vec<Layer> = vec![];
        let mut layer_folders: Option<Vec<LayerFolder>> = None;

        for layer_builder in &self.layers {
            let layer = layer_builder.build().map_group_err()?;
            if layer.name().is_some() {
                self.features.insert(LayerFeature::Name);
            }
            if layer.visible().is_some() {
                self.features.insert(LayerFeature::Visible);
            }
            if layer.editable().is_some() {
                self.features.insert(LayerFeature::Editable);
            }
            if layer.folder_id().is_some() {
                self.features.insert(LayerFeature::Folders);
            }
            layers.push(layer);
        }

        if let Some(layer_folder_builders) = &self.layer_folders {
            let mut some_layer_folders = vec![];

            for layer_folder_builder in layer_folder_builders {
                let layer_folder = layer_folder_builder.build().map_group_err()?;
                if layer_folder.name().is_some() {
                    self.features.insert(LayerFeature::Name);
                }
                if layer_folder.visible().is_some() {
                    self.features.insert(LayerFeature::Visible);
                }
                if layer_folder.editable().is_some() {
                    self.features.insert(LayerFeature::Editable);
                }
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
            &mut self.features,
            LayerFeature::Folders,
            &mut self.layer_folders,
            vec![],
        );
        layer_folders.push(LayerFolderBuilder::default().id(id).index(index).to_owned());
        Ok(layer_folders.last_mut().unwrap())
    }

    pub fn get_layer_folders(
        &mut self,
    ) -> Result<impl Iterator<Item = &mut LayerFolderBuilder>, LayerGroupBuilderError> {
        let layer_folders = Self::require_feature(
            &mut self.features,
            LayerFeature::Folders,
            &mut self.layer_folders,
            vec![],
        );
        Ok(layer_folders.iter_mut())
    }
}
