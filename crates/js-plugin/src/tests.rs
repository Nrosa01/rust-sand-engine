#[cfg(test)]
mod tests {
    use app_core::Transformation;

    use crate::blocks::{Actions, Conditions, Direction, Number, NumbersConstant, NumbersRuntime, TransformationInternal};

    #[test]
    #[rustfmt::skip]
    fn test_block_serialization_and_deserialization() {
        let block1 = Conditions::CompareNumberEquality { block1: Number::NumbersConstant(NumbersConstant::ParticleIdFromName("Water".to_string())), block2: Number::NumbersRuntime(NumbersRuntime::NumberOfXTouching(NumbersConstant::ParticleType(1))) };
        let serialized = serde_json::to_string(&block1).unwrap();
        let deserialized: Conditions = serde_json::from_str(&serialized).unwrap();

        std::fs::write("test.json", serialized).unwrap();
        assert_eq!(block1, deserialized);
    }

    #[test]
    #[rustfmt::skip]
    pub fn test_sand_serialization_and_deserialization()
    {
        let blocks = vec![
            Actions::If { 
                condition:
                    Conditions::CheckTypesInDirection { 
                    direction: Direction::Constant([0, -1]), 
                    types: vec![NumbersConstant::ParticleIdFromName("empty".to_string()), NumbersConstant::ParticleIdFromName("water".to_string())] }, 
                result: Box::new(Actions::Swap { direction: Direction::Constant([0, -1]) }), 
                r#else: Some(Box::new(Actions::If {
                    condition: Conditions::CheckTypesInDirection { 
                        direction: Direction::Constant([-1, -1]), 
                        types: vec![NumbersConstant::ParticleIdFromName("empty".to_string()), NumbersConstant::ParticleIdFromName("water".to_string())] }, 
                    result: Box::new(Actions::Swap { direction: Direction::Constant([-1, -1]) }), 
                    r#else: Some(Box::new(Actions::If {
                        condition: Conditions::CheckTypesInDirection { 
                            direction: Direction::Constant([1, -1]), 
                            types: vec![NumbersConstant::ParticleIdFromName("empty".to_string()), NumbersConstant::ParticleIdFromName("water".to_string())] }, 
                        result: Box::new(Actions::Swap { direction: Direction::Constant([1, -1]) }), 
                        r#else: None
                    }))
                }))
            }
        ];
    
    
        println!("Block going to be serilized");
        let serialized = serde_json::to_string(&blocks).map_err(|err| err.to_string()).unwrap();
        println!("Block serilized");
    
        std::fs::write("sand.json", serialized).map_err(|err| err.to_string()).unwrap();
    }

    #[test]
    pub fn test_replicant()
    {
        let blocks = vec![
            Actions::ForEachTransformation { transformation: TransformationInternal::Rotation, block: 
            Box::new(Actions::CopyTo { direction: Direction::Constant([0, -1]) }) }
        ];
    
    
        println!("Block going to be serilized");
        let serialized = serde_json::to_string(&blocks).map_err(|err| err.to_string()).unwrap();
        println!("Block serilized");
    
        std::fs::write("replicant.json", serialized).map_err(|err| err.to_string()).unwrap();
    }

    #[test]
    pub fn test_sand2_serialization_and_deserialization() {
        let blocks = vec![Actions::RandomTransformation {
            transformation: TransformationInternal::HorizontalReflection,
            block: Box::new(Actions::If {
                condition: Conditions::CheckTypesInDirection {
                    direction: Direction::Constant([0, -1]),
                    types: vec![
                        NumbersConstant::ParticleIdFromName("empty".to_string()),
                        NumbersConstant::ParticleIdFromName("water".to_string()),
                    ],
                },
                result: Box::new(Actions::Swap { direction: Direction::Constant([0, -1]) }),
                r#else: Some(Box::new(Actions::If {
                    condition: Conditions::CheckTypesInDirection {
                        direction: Direction::Constant([-1, -1]),
                        types: vec![
                            NumbersConstant::ParticleIdFromName("empty".to_string()),
                            NumbersConstant::ParticleIdFromName("water".to_string()),
                        ],
                    },
                    result: Box::new(Actions::Swap {
                        direction: Direction::Constant([-1, -1]),
                    }),
                    r#else: Some(Box::new(Actions::If {
                        condition: Conditions::CheckTypesInDirection {
                            direction: Direction::Constant([1, -1]),
                            types: vec![
                                NumbersConstant::ParticleIdFromName("empty".to_string()),
                                NumbersConstant::ParticleIdFromName("water".to_string()),
                            ],
                        },
                        result: Box::new(Actions::Swap { direction: Direction::Constant([1, -1]) }),
                        r#else: None,
                    })),
                })),
            }),
        }];

        println!("Block going to be serilized");
        let serialized = serde_json::to_string(&blocks)
            .map_err(|err| err.to_string())
            .unwrap();
        println!("Block serilized");

        std::fs::write("sand2.json", serialized)
            .map_err(|err| err.to_string())
            .unwrap();
    }

    #[test]
    #[rustfmt::skip]
    pub fn test_transformations()
    {
        let direction = [1,0];
        let direction2 = [0,1];
        let direction3 = [1,1];
        
        let transformation = Transformation::HorizontalReflection(true);
        let new_direction = transformation.transform(&direction);
        let new_direction2 = transformation.transform(&direction2);
        let new_direction3 = transformation.transform(&direction3);
        assert_eq!(new_direction, [-1,0]);
        assert_eq!(new_direction2, direction2);
        assert_eq!(new_direction3, [-1,1]);

        let transformation = Transformation::HorizontalReflection(false);
        let new_direction = transformation.transform(&direction);
        let new_direction2 = transformation.transform(&direction2);
        let new_direction3 = transformation.transform(&direction3);
        assert_eq!(new_direction, direction);
        assert_eq!(new_direction2, direction2);
        assert_eq!(new_direction3, direction3);

        let transformation = Transformation::VerticalReflection(true);
        let new_direction = transformation.transform(&direction);
        let new_direction2 = transformation.transform(&direction2);
        let new_direction3 = transformation.transform(&direction3);
        assert_eq!(new_direction, direction);
        assert_eq!(new_direction2, [0,-1]);
        assert_eq!(new_direction3, [1,-1]);

        let transformation = Transformation::VerticalReflection(false);
        let new_direction = transformation.transform(&direction);
        let new_direction2 = transformation.transform(&direction2);
        let new_direction3 = transformation.transform(&direction3);
        assert_eq!(new_direction, direction);
        assert_eq!(new_direction2, direction2);
        assert_eq!(new_direction3, direction3);

        let transformation = Transformation::Reflection(true, true);
        let new_direction = transformation.transform(&direction);
        let new_direction2 = transformation.transform(&direction2);
        let new_direction3 = transformation.transform(&direction3);
        assert_eq!(new_direction, [-1,0]);
        assert_eq!(new_direction2, [0,-1]);
        assert_eq!(new_direction3, [-1,-1]);

        let transformation = Transformation::Reflection(false, false);
        let new_direction = transformation.transform(&direction);
        let new_direction2 = transformation.transform(&direction2);
        let new_direction3 = transformation.transform(&direction3);
        assert_eq!(new_direction, direction);
        assert_eq!(new_direction2, direction2);
        assert_eq!(new_direction3, direction3);

        let transformation = Transformation::Reflection(true, false);
        let new_direction = transformation.transform(&direction);
        let new_direction2 = transformation.transform(&direction2);
        let new_direction3 = transformation.transform(&direction3);
        assert_eq!(new_direction, [-1,0]);
        assert_eq!(new_direction2, direction2);
        assert_eq!(new_direction3, [-1,1]);

        let transformation = Transformation::Reflection(false, true);
        let new_direction = transformation.transform(&direction);
        let new_direction2 = transformation.transform(&direction2);
        let new_direction3 = transformation.transform(&direction3);
        assert_eq!(new_direction, direction);
        assert_eq!(new_direction2, [0,-1]);
        assert_eq!(new_direction3, [1,-1]);

        let transformation = Transformation::Rotation(1);
        let new_direction = transformation.transform(&direction);
        let new_direction2 = transformation.transform(&direction2);
        let new_direction3 = transformation.transform(&direction3);
        assert_eq!(new_direction, [1,-1]);
        assert_eq!(new_direction2, [1,1]);
        assert_eq!(new_direction3, [1,0]);

        let transformation = Transformation::Rotation(0);
        let new_direction = transformation.transform(&direction);
        let new_direction2 = transformation.transform(&direction2);
        let new_direction3 = transformation.transform(&direction3);
        let new_direction4 = transformation.transform(&[-1, 1]);
        assert_eq!(new_direction, direction);
        assert_eq!(new_direction2, direction2);
        assert_eq!(new_direction3, direction3);
        assert_eq!(new_direction4, [-1, 1]);

        let transformation = Transformation::Rotation(7);
        let new_direction = transformation.transform(&direction);
        let new_direction2 = transformation.transform(&direction2);
        let new_direction3 = transformation.transform(&direction3);
        let new_direction4 = transformation.transform(&[-1, 1]);
        assert_eq!(new_direction, [1,1]);
        assert_eq!(new_direction2, [-1,1]);
        assert_eq!(new_direction3, [0,1]);
        assert_eq!(new_direction4, [-1, 0]);
    }
}
