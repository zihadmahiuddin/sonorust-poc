// use bevy::prelude::*;
// use sonorust_interpreter::{
//     Executor, IterativeExecutor, MemoryWrapper, Node, ReadOnlyMemory, ReadWriteMemory,
//     SideEffectAccess, SideEffectQueue, SideEffectWrapper,
// };

// #[derive(Resource)]
// struct Nodes(Vec<Node>);

// #[derive(Resource)]
// struct EvaluateNode(Option<usize>);

fn main() {
    // let nodes = Nodes(vec![
    //     Node::Value(0.0),  // mem_index
    //     Node::Value(1.0),  // mem_block
    //     Node::Value(20.0), // a
    //     Node::Value(24.0), // b
    //     Node::Value(44.0), // c
    //     Node::FunctionCall {
    //         fn_name: "Get".to_owned(),
    //         arg_indices: vec![1, 0], // Get(mem_block, mem_index)
    //     },
    //     Node::FunctionCall {
    //         fn_name: "Set".to_owned(),
    //         arg_indices: vec![1, 0, 3], // Set(mem_block, mem_index, b)
    //     },
    //     Node::FunctionCall {
    //         fn_name: "Add".to_owned(),
    //         arg_indices: vec![2, 5], // Add(a, Get(mem_block, mem_index))
    //     },
    //     Node::FunctionCall {
    //         fn_name: "Equal".to_owned(),
    //         arg_indices: vec![7, 3], // Equal(Add(a, Get(mem_block, mem_index)), c)
    //     },
    //     Node::FunctionCall {
    //         fn_name: "Execute".to_owned(), // Execute(set_index, equal_index)
    //         arg_indices: vec![10, 11, 12],
    //     },
    //     Node::Value(6.0), // set_index
    //     Node::Value(7.0), // equal_index
    //     Node::Value(14.0),
    //     Node::Value(34.0),
    //     Node::FunctionCall {
    //         fn_name: "Spawn".to_owned(),
    //         arg_indices: vec![13],
    //     },
    // ]);
    //
    // App::new()
    //     // .add_plugins(DefaultPlugins)
    //     .insert_resource(nodes)
    //     .insert_resource(EvaluateNode(None))
    //     .insert_resource(ReadOnlyMemory::default())
    //     .insert_resource(ReadWriteMemory::default())
    //     .insert_resource(SideEffectQueue::default())
    //     .add_systems(Startup, setup)
    //     .add_systems(Update, update)
    //     .run();
}

// fn setup(mut evaluate_node: ResMut<EvaluateNode>) {
//     evaluate_node.0.replace(9);
// }

// fn update(
//     nodes: Res<Nodes>,
//     evaluate_node: Res<EvaluateNode>,
//     mut memory_wrapper: MemoryWrapper,
//     mut side_effect_wrapper: SideEffectWrapper,
// ) {
//     if let Some(evaluate_node) = evaluate_node.0 {
//         let executor = IterativeExecutor {
//             memory_access: &mut memory_wrapper,
//             nodes: &nodes.0,
//             side_effect_access: &mut side_effect_wrapper,
//         };
//         let (_, result) = executor.execute(evaluate_node);
//         dbg!(result);
//         dbg!(side_effect_wrapper.iter().collect::<Vec<_>>());
//     }
// }
