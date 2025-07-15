use crate::track::{
    Vec2,
    group_builder::{
        group_builder_base::{GroupBuilder, GroupBuilderBase},
        group_builder_error::{GroupBuilderError, IntoGroupResult},
        group_builder_macro::define_group_builder,
    },
    groups::line::{
        acceleration_line::{
            AccelerationLine, AccelerationLineBuilder, AccelerationLineBuilderError,
        },
        scenery_line::{SceneryLine, SceneryLineBuilder, SceneryLineBuilderError},
        standard_line::{StandardLine, StandardLineBuilder, StandardLineBuilderError},
    },
};
use std::collections::HashSet;

define_group_builder!(
  enum LineFeature {
    SceneryWidth,
    AccelerationMultiplier,
  }

  struct LineGroup {
    standard_lines: Vec<StandardLine>, Vec<StandardLineBuilder>, StandardLineBuilderError,
    acceleration_lines: Vec<AccelerationLine>, Vec<AccelerationLineBuilder>, AccelerationLineBuilderError,
    scenery_lines: Vec<SceneryLine>, Vec<SceneryLineBuilder>, SceneryLineBuilderError,
  }
);

impl GroupBuilder for LineGroupBuilder {
    fn build_group(&mut self) -> Result<Self::Output, GroupBuilderError<Self::SubError>> {
        let mut standard_lines: Vec<StandardLine> = vec![];
        let mut acceleration_lines: Vec<AccelerationLine> = vec![];
        let mut scenery_lines: Vec<SceneryLine> = vec![];

        for standard_line_builder in &self.standard_lines {
            let standard_line = standard_line_builder.build().map_group_err()?;
            standard_lines.push(standard_line);
        }

        for acceleration_line_builder in &self.acceleration_lines {
            let acceleration_line = acceleration_line_builder.build().map_group_err()?;
            Self::check_feature(
                &mut self.features,
                LineFeature::AccelerationMultiplier,
                &acceleration_line.multiplier(),
                "multiplier",
            )?;
            acceleration_lines.push(acceleration_line);
        }

        for scenery_line_builder in &self.scenery_lines {
            let scenery_line = scenery_line_builder.build().map_group_err()?;
            Self::check_feature(
                &mut self.features,
                LineFeature::SceneryWidth,
                &scenery_line.width(),
                "width",
            )?;
            scenery_lines.push(scenery_line);
        }

        Ok(LineGroup {
            features: self.features.clone(),
            standard_lines,
            acceleration_lines,
            scenery_lines,
        })
    }
}

impl LineGroupBuilder {
    pub fn add_standard_line(
        &mut self,
        id: u32,
        endpoints: (Vec2, Vec2),
        flipped: bool,
        left_extension: bool,
        right_extension: bool,
    ) -> &mut StandardLineBuilder {
        self.standard_lines.push(
            StandardLineBuilder::default()
                .id(id)
                .endpoints(endpoints)
                .flipped(flipped)
                .left_extension(left_extension)
                .right_extension(right_extension)
                .to_owned(),
        );

        self.standard_lines.last_mut().unwrap()
    }

    pub fn get_standard_lines(&mut self) -> impl Iterator<Item = &mut StandardLineBuilder> {
        self.standard_lines.iter_mut()
    }

    pub fn add_acceleration_line(
        &mut self,
        id: u32,
        endpoints: (Vec2, Vec2),
        flipped: bool,
        left_extension: bool,
        right_extension: bool,
    ) -> &mut AccelerationLineBuilder {
        self.acceleration_lines.push(
            AccelerationLineBuilder::default()
                .id(id)
                .endpoints(endpoints)
                .flipped(flipped)
                .left_extension(left_extension)
                .right_extension(right_extension)
                .to_owned(),
        );

        self.acceleration_lines.last_mut().unwrap()
    }

    pub fn get_acceleration_lines(&mut self) -> impl Iterator<Item = &mut AccelerationLineBuilder> {
        self.acceleration_lines.iter_mut()
    }

    pub fn add_scenery_line(
        &mut self,
        id: u32,
        endpoints: (Vec2, Vec2),
    ) -> &mut SceneryLineBuilder {
        self.scenery_lines.push(
            SceneryLineBuilder::default()
                .id(id)
                .endpoints(endpoints)
                .to_owned(),
        );

        self.scenery_lines.last_mut().unwrap()
    }

    pub fn get_scenery_lines(&mut self) -> impl Iterator<Item = &mut SceneryLineBuilder> {
        self.scenery_lines.iter_mut()
    }
}
