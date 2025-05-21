pub mod blocks;

pub trait MemoryRegion {
    fn size(&self) -> usize;
    fn read(&self, index: usize) -> Option<f64>;
    fn write(&mut self, index: usize, value: f64);
}

// pub trait MemoryAccessFor<T> {
//     type Param<'w>: SystemParam + AsMemoryBlock<'w, T>;
// }
//
// #[derive(SystemParam)]
// pub struct ReadOnly<'w, T: Resource>(Res<'w, T>);
//
// #[derive(SystemParam)]
// pub struct ReadWrite<'w, T: Resource>(ResMut<'w, T>);
//
// pub trait AsMemoryBlock<'w, T> {
//     fn as_block(&'w self) -> &'w T;
//     fn as_block_mut(&'w mut self) -> Option<&'w mut T>;
// }
//
// impl<'w, T: Resource> AsMemoryBlock<'w, T> for ReadOnly<'w, T> {
//     fn as_block(&'w self) -> &'w T {
//         self.0.as_ref()
//     }
//     fn as_block_mut(&'w mut self) -> Option<&'w mut T> {
//         None
//     }
// }
//
// impl<'w, T: Resource> AsMemoryBlock<'w, T> for ReadWrite<'w, T> {
//     fn as_block(&'w self) -> &'w T {
//         &self.0
//     }
//     fn as_block_mut(&'w mut self) -> Option<&'w mut T> {
//         Some(&mut *self.0)
//     }
// }
//
// impl MemoryAccessFor<MockEnvironment> for PreparationStage {
//     type Param<'w> = ReadWrite<'w, MockEnvironment>;
// }
//
// impl MemoryAccessFor<MockEnvironment> for SpawningStage {
//     type Param<'w> = ReadOnly<'w, MockEnvironment>;
// }
//
// impl MemoryAccessFor<MockUi> for PreparationStage {
//     type Param<'w> = ReadOnly<'w, MockUi>;
// }
//
// impl MemoryAccessFor<MockUi> for SpawningStage {
//     type Param<'w> = ReadOnly<'w, MockUi>;
// }
//
// #[derive(Default, Resource)]
// pub struct MockEnvironment {
//     pub data: Vec<f64>,
// }
//
// #[derive(Default, Resource)]
// pub struct MockUi {
//     pub data: Vec<f64>,
// }
//
// impl MemoryRegion for MockEnvironment {
//     fn size(&self) -> usize {
//         self.data.len()
//     }
//
//     fn read(&self, index: usize) -> Option<f64> {
//         self.data.get(index).copied()
//     }
//
//     fn write(&mut self, index: usize, value: f64) {
//         if index < self.data.len() {
//             self.data[index] = value;
//         }
//     }
// }
//
// impl MemoryRegion for MockUi {
//     fn size(&self) -> usize {
//         self.data.len()
//     }
//
//     fn read(&self, index: usize) -> Option<f64> {
//         self.data.get(index).copied()
//     }
//
//     fn write(&mut self, index: usize, value: f64) {
//         if index < self.data.len() {
//             self.data[index] = value;
//         }
//     }
// }
//
// pub struct MemoryStageView<'w, Stage>
// where
//     Stage: MemoryAccessFor<MockEnvironment> + MemoryAccessFor<MockUi>,
// {
//     env: <Stage as MemoryAccessFor<MockEnvironment>>::Param<'w>,
//     ui: <Stage as MemoryAccessFor<MockUi>>::Param<'w>,
// }
//
// impl<'w, Stage> MemoryStageView<'w, Stage>
// where
//     Stage: MemoryAccessFor<MockEnvironment> + MemoryAccessFor<MockUi>,
// {
//     pub fn new(
//         env: <Stage as MemoryAccessFor<MockEnvironment>>::Param<'w>,
//         ui: <Stage as MemoryAccessFor<MockUi>>::Param<'w>,
//     ) -> Self {
//         Self { env, ui }
//     }
// }
//
// fn system_for_spawning_stage(
//     env: <SpawningStage as MemoryAccessFor<MockEnvironment>>::Param<'_>,
//     ui: <SpawningStage as MemoryAccessFor<MockUi>>::Param<'_>,
// ) {
//     let memory_view = MemoryStageView::<SpawningStage>::new(env, ui);
//     let env = memory_view.env.0; // Res
//     let ui = memory_view.ui.0; // Res
// }
//
// fn system_for_preparation_stage(
//     env: <PreparationStage as MemoryAccessFor<MockEnvironment>>::Param<'_>,
//     ui: <PreparationStage as MemoryAccessFor<MockUi>>::Param<'_>,
// ) {
//     let memory_view = MemoryStageView::<PreparationStage>::new(env, ui);
//     let env = memory_view.env.0; // ResMut
//     let ui = memory_view.ui.0; // Res
// }
