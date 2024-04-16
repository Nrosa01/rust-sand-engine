#[cfg(test)]
mod tests {
    use app_core::Transformation;

    use crate::blocks::{Blocks, Number, NumberConstants, NumbersRuntime};

    #[test]
    fn test_block_serialization_and_deserialization() {
        let block1 = Blocks::CompareNumberEquality { block1: Number::NumberConstants(NumberConstants::ParticleIdFromName("Water".to_string())), block2: Number::NumbersRuntime(NumbersRuntime::NumberOfXTouching(NumberConstants::ParticleType(1))) };
        let serialized = serde_json::to_string(&block1).unwrap();
        let deserialized: Blocks = serde_json::from_str(&serialized).unwrap();

        std::fs::write("test.json", serialized).unwrap();
        assert_eq!(block1, deserialized);
    }

    #[test]
    pub fn test_sand_serialization_and_deserialization()
    {
        let blocks = vec![
            Blocks::If { 
                condition: Box::new(Blocks::IfDirectionIsAnyType { 
                    direction: [0, -1], 
                    types: vec![NumberConstants::ParticleIdFromName("empty".to_string()), NumberConstants::ParticleIdFromName("water".to_string())] }), 
                result: Box::new(Blocks::Swap { direction: [0, -1] }), 
                r#else: Some(Box::new(Blocks::If {
                    condition: Box::new(Blocks::IfDirectionIsAnyType { 
                        direction: [-1, -1], 
                        types: vec![NumberConstants::ParticleIdFromName("empty".to_string()), NumberConstants::ParticleIdFromName("water".to_string())] }), 
                    result: Box::new(Blocks::Swap { direction: [-1, -1] }), 
                    r#else: Some(Box::new(Blocks::If {
                        condition: Box::new(Blocks::IfDirectionIsAnyType { 
                            direction: [1, -1], 
                            types: vec![NumberConstants::ParticleIdFromName("empty".to_string()), NumberConstants::ParticleIdFromName("water".to_string())] }), 
                        result: Box::new(Blocks::Swap { direction: [1, -1] }), 
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
    }

}